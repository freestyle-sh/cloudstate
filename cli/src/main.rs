use clap::ValueHint;
use cloudstate_runtime::extensions::cloudstate::ReDBCloudstate;
use notify::{EventHandler, Watcher};
use redb::{backends::InMemoryBackend, Database};
use server::CloudstateServer;
use tokio::net::TcpListener;
use std::{collections::HashMap, fs, path::Path};

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

            let run_func = |event: notify::Event| {
                println!("File changed: {:?}", event);
            };
            // let run_func = |event: notify::Event| {

            // };

            if *watch.unwrap_or(&false) {
                watch_file(Path::new(filename), run_func);
            }

            println!("Running file: {:?}", filename);
            println!("Watching: {:?}", watch);
        }
        Some(("serve", serve_matches)) => {
            let filename = serve_matches.get_one::<String>("filename").unwrap();
            let watch = serve_matches.get_one::<bool>("watch");

            let classes = fs::read_to_string(filename).unwrap();
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

            let _ = run_server(&classes, listener).await;
            

            println!("Serving file: {:?}", filename);
            if *watch.unwrap_or(&false) {
                watch_file(Path::new(filename), |_|{

                });
            }
        }
        _ => {
            println!("No subcommand found");
        }
    }
}

fn watch_file(path: &Path, func: fn(notify::Event) -> ()) {
    notify::recommended_watcher(move |evt: Result<notify::Event, notify::Error>| {
        let unwrapped = evt.expect("Error watching filesystem");

        func(unwrapped);
    })
    .unwrap()
    .watch(path, notify::RecursiveMode::NonRecursive)
    .unwrap();
}

async fn run_server(classes: &str, listener: TcpListener){
    let server = CloudstateServer::new(
        ReDBCloudstate {
            db: Database::builder()
                .create_with_backend(InMemoryBackend::default())
                .unwrap(),
            transactions: HashMap::new(),
        },
        classes,
    )
    .await;
   let out= axum::serve(listener, server.router);

   out.await.unwrap();



}