use axum::{
    body::Body,
    extract::{Request, State},
    handler,
    http::StatusCode,
    routing::{get, IntoMakeService},
    Router,
};
use tokio::runtime::Runtime; // 0.3.5

use clap::ValueHint;
use cloudstate_runtime::extensions::cloudstate::ReDBCloudstate;
use notify::{EventHandler, Watcher};
use redb::{backends::InMemoryBackend, Database};
use server::CloudstateServer;
use std::{
    collections::HashMap,
    fs,
    future::{poll_fn, Future},
    path::Path,
    pin::Pin,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tower::Service;

#[tokio::main]
async fn main() {
    let filename_arg = clap::Arg::new("filename")
        .help("The filename to serve")
        .value_hint(ValueHint::FilePath)
        .required(true);
    let watch_arg = clap::Arg::new("watch")
        .short('w')
        .long("watch")
        .help("Watch the file for changes")
        .required(false)
        .num_args(0);

    let cmd = clap::Command::new("cloudstate")
        .bin_name("cloudstate")
        .name("cloudstate")
        .about("cloudstate is a command line tool to manage the Cloudstate runtime")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            clap::Command::new("run").about("Runs a file on the cloudstate runtime").long_about("
            Runs a file on the cloudstate runtime. This is useful for single time use code, or for testing code.
            ").arg(
               filename_arg.clone()
            ).arg(
                watch_arg.clone()
            ),
        ).subcommand(
            clap::Command::new("serve").about("Serves a file on the cloudstate runtime").arg(
                filename_arg.clone()
            ).arg(
                watch_arg.clone()
            )
        );

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            let filename = run_matches.get_one::<String>("filename").unwrap();
            let watch = run_matches.get_one::<bool>("watch");

            // let run_func = |event: notify::Event| {
            //     let matches_clone = matches.clone();
            //     println!("File changed: {:?}", event);
            // };
            // let run_func = |event: notify::Event| {

            // };

            if *watch.unwrap_or(&false) {
                // watch_file(Path::new(filename), run_func);
            }

            println!("Running file: {:?}", filename);
            println!("Watching: {:?}", watch);
        }
        Some(("serve", serve_matches)) => {
            let serve_matches = serve_matches.clone();
            let filename = serve_matches.get_one::<String>("filename").unwrap().clone();
            let watch = serve_matches.get_one::<bool>("watch");

            let classes = fs::read_to_string(&filename).unwrap();
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            let cloudstate = Arc::new(Mutex::new(ReDBCloudstate {
                db: Database::builder()
                    .create_with_backend(InMemoryBackend::default())
                    .unwrap(),
                transactions: HashMap::new(),
            }));
            let server = CloudstateServer::new(cloudstate.clone(), &classes).await;

            let app_state = Arc::new(Mutex::new(server));

            let _ = run_server(app_state.clone(), listener).await;

            if *watch.unwrap_or(&false) {
                let pre_cloned_filename = filename.clone();
                watch_file(
                    Path::new(&filename),
                    Box::new(move |_| {
                        Runtime::new().unwrap().block_on(async {
                            if let Ok(new_classes) = fs::read_to_string(&pre_cloned_filename) {
                                let mut server = app_state.lock().unwrap();

                                *server = CloudstateServer::new(cloudstate.clone(), &new_classes).await;

                                drop(server);
                            }
                        })
                    }),
                );
            }
        }
        _ => {
            println!("No subcommand found");
        }
    }
}

fn watch_file(path: &Path, mut func: Box<impl FnMut(notify::Event) -> () + std::marker::Send + 'static>) {
    notify::recommended_watcher(move |evt: Result<notify::Event, notify::Error>| {
        let unwrapped = evt.expect("Error watching filesystem");
        func(unwrapped);

    })
    .unwrap()
    .watch(path, notify::RecursiveMode::NonRecursive)
    .unwrap();
}

async fn run_server(server: Arc<Mutex<CloudstateServer>>, listener: TcpListener) {
    let handle = |req: Request| async move {
        let response = handler(server.clone(), req);
        response
    };
    let svr = axum::Router::new().route(
        "*",
        get(handle.clone())
            .post(handle.clone())
            .delete(handle.clone())
            .put(handle.clone())
            .patch(handle.clone()),
    );

    let out = axum::serve(listener, svr);

    out.await.unwrap();
}

#[tokio::main]
async fn handler(
    server: Arc<Mutex<CloudstateServer>>,
    req: Request<Body>,
) -> axum::http::Response<Body> {
    let server = server.lock().unwrap();
    let router = server.router.clone();
    drop(server);

    let mut service: axum::routing::RouterIntoService<Body> = router.into_service();

    poll_fn(|cx: &mut std::task::Context<'_>| {
        <axum::routing::RouterIntoService<Body>>::poll_ready(&mut service, cx)
    })
    .await
    .unwrap();

    let response = service.call(req).await.unwrap();

    return response;
}
