use axum::{body::Body, extract::Request, routing::get};
use tokio::{runtime::Runtime, sync::RwLock}; // 0.3.5

use clap::ValueHint;
use cloudstate_runtime::extensions::cloudstate::ReDBCloudstate;
use notify::Watcher;
use redb::{
    backends::{self},
    Database,
};
use server::{execute_script, CloudstateServer};
use std::{
    collections::HashMap,
    fs::{self},
    future::poll_fn,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::net::TcpListener;
use tower::Service;
use tracing::event;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    event!(tracing::Level::DEBUG, "Starting cloudstate");

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

    let memory_only_arg = clap::Arg::new("memory-only")
        .help("Only store data in memory")
        .long("memory-only")
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
            ).arg(memory_only_arg.clone()),
        ).subcommand(
            clap::Command::new("serve").about("Serves a file on the cloudstate runtime")
                .arg(filename_arg.clone())
                .arg(watch_arg.clone())
                .arg(memory_only_arg.clone())
        );

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("run", run_matches)) => {
            let filename = run_matches.get_one::<String>("filename").unwrap();
            let memory_only = run_matches.get_one::<bool>("memory-only").unwrap();
            let script = fs::read_to_string(filename).unwrap();

            let db = if memory_only.clone() {
                Database::builder()
                    .create_with_backend(backends::InMemoryBackend::default())
                    .unwrap()
            } else {
                Database::create("./cloudstate").unwrap()
            };

            execute_script(
                &script,
                &"",
                Arc::new(Mutex::new(ReDBCloudstate {
                    db: db,
                    transactions: HashMap::new(),
                })),
            )
            .await;
        }
        Some(("serve", serve_matches)) => {
            let serve_matches = serve_matches.clone();
            let filename = serve_matches.get_one::<String>("filename").unwrap().clone();
            let watch = serve_matches.get_one::<bool>("watch").unwrap();
            let memory_only = serve_matches.get_one::<bool>("memory-only").unwrap();

            let db = if memory_only.clone() {
                Database::builder()
                    .create_with_backend(backends::InMemoryBackend::default())
                    .unwrap()
            } else {
                Database::create("./cloudstate").unwrap()
            };

            let classes = fs::read_to_string(&filename).unwrap_or("".to_string());
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            let cloudstate = Arc::new(Mutex::new(ReDBCloudstate {
                db: db,
                transactions: HashMap::new(),
            }));
            let server = CloudstateServer::new(cloudstate.clone(), &classes).await;

            let app_state = Arc::new(RwLock::new(server));

            let cloned = Arc::clone(&app_state);
            let other_thread = tokio::spawn(async move {
                event!(tracing::Level::INFO, "Starting server");
                let _ = run_server(cloned, listener).await;
            });

            if *watch {
                let pre_cloned_filename: String = filename.clone();

                let mut watcher = notify::recommended_watcher(
                    move |evt: Result<notify::Event, notify::Error>| {
                        let evt = evt.unwrap();
                        let should_reload = match evt.kind {
                            notify::EventKind::Any => false,
                            notify::EventKind::Access(_) => false,
                            notify::EventKind::Create(_) => true,
                            notify::EventKind::Modify(_) => true,
                            notify::EventKind::Remove(_) => false,
                            notify::EventKind::Other => false,
                        };
                        if should_reload {
                            event!(tracing::Level::INFO, "Reloading file");

                            Runtime::new().unwrap().block_on(async {
                                if let Ok(new_classes) = fs::read_to_string(&pre_cloned_filename) {
                                    let mut server = app_state.write().await;

                                    *server =
                                        CloudstateServer::new(cloudstate.clone(), &new_classes)
                                            .await;

                                    drop(server);
                                }
                            })
                        }
                    },
                )
                .unwrap();

                watcher
                    .configure(notify::Config::default().with_poll_interval(Duration::from_secs(2)))
                    .unwrap();

                watcher
                    .watch(
                        Path::new(&filename).parent().unwrap(),
                        notify::RecursiveMode::Recursive,
                    )
                    .unwrap();

                // I know this else block is weird but it doesn't work without it
                other_thread.await.unwrap()
            } else {
                other_thread.await.unwrap()
            }
        }
        _ => {
            event!(tracing::Level::INFO, "No subcommand found");
        }
    }
}

async fn run_server(server: Arc<RwLock<CloudstateServer>>, listener: TcpListener) {
    let handle = |req: Request| async move {
        tokio::task::spawn_blocking(move || handler(server.clone(), req))
            .await
            .unwrap()
        // let response = std::thread::spawn(move || handler(server.clone(), req))
        //     .join()
        //     .unwrap();
    };

    let svr = axum::Router::new().fallback(
        get(handle.clone())
            .post(handle.clone())
            .delete(handle.clone())
            .put(handle.clone())
            .patch(handle.clone()),
    );

    let out = axum::serve(listener, svr);

    out.await.unwrap();
}

#[tokio::main(flavor = "current_thread")]
async fn handler(
    server: Arc<RwLock<CloudstateServer>>,
    req: Request<Body>,
) -> axum::http::Response<Body> {
    event!(tracing::Level::INFO, "Pulling service");

    let server = server.read().await;
    let router = server.router.clone();

    drop(server);

    let mut service: axum::routing::RouterIntoService<Body> = router.into_service();

    event!(tracing::Level::DEBUG, "Preparing service");

    poll_fn(|cx: &mut std::task::Context<'_>| {
        <axum::routing::RouterIntoService<Body>>::poll_ready(&mut service, cx)
    })
    .await
    .unwrap();

    event!(tracing::Level::DEBUG, "Calling service");

    let response = service.call(req).await.unwrap();

    event!(tracing::Level::DEBUG, "Returning response");

    return response;
}
