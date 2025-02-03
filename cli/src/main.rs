// #[cfg(feature = "dhat-heap")]
// #[global_allocator]
// static ALLOC: dhat::Alloc = dhat::Alloc;

use axum::extract::DefaultBodyLimit;
use axum::{body::Body, extract::Request, routing::get, Json};
use clap::{Parser, ValueHint};
use cloudstate_runtime::backup::BackupProgress;
use cloudstate_runtime::ServerInfo;
use cloudstate_runtime::{
    blob_storage::{
        fs_store::FsBlobStore, in_memory_store::InMemoryBlobStore, CloudstateBlobStorage,
        CloudstateBlobStorageEngine,
    },
    extensions::cloudstate::ReDBCloudstate,
    gc::mark_and_sweep,
};
use indicatif::ProgressBar;
use notify::Watcher;
use redb::{
    backends::{self},
    Database,
};
use server::cloudstate_runner::simple::SimpleCloudstateRunner;
use server::{cloudstate_runner::execute::execute_script, CloudstateServer};
use std::{
    collections::HashMap,
    fs::{self},
    future::poll_fn,
    os::unix::fs::MetadataExt,
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::net::TcpListener;
use tokio::runtime::Runtime; // 0.3.5
use tokio::sync::RwLock;
use tower::Service;
use tracing::{debug, info};

#[derive(clap::Parser)]
struct CliArguments {
    #[clap(value_hint = ValueHint::FilePath)]
    // #[arg(required = true, long, help = "The filename to serve")]
    filename: String,
    #[arg(
        long,
        short,
        num_args = 0,
        required = false,
        help = "Watch the file for changes"
    )]
    watch: bool,

    #[arg(
        long = "memory-only",
        num_args = 0,
        required = false,
        help = "Only store data in memory"
    )]
    memory_only: bool,
}

#[derive(clap::Parser)]
struct GcArguments {
    #[arg(
        required = true,
        long,
        help = "The database file to run the garbage collector on"
    )]
    filename: String,
}

#[derive(clap::Parser)]
struct BackupArguments {
    #[arg(
        required = true,
        long,
        help = "The database file to backup",
        default_value = "cloudstate"
    )]
    filename: String,
    #[arg(
        required = true,
        long,
        help = "The backup file to write to",
        default_value = "backup"
    )]
    backup_filename: String,
}

#[derive(clap::Parser)]
#[clap(
    name = "cloudstate",
    bin_name = "cloudstate",
    version = env!("CARGO_PKG_VERSION"),
    about = "Cloudstate is a command line tool to manage the Cloudstate runtime"
)]
enum Cli {
    #[command(
        name = "run",
        about = "Runs a file on the cloudstate runtime",
        long_about = "Runs a file on the cloudstate runtime. This is useful for single time use code, or for testing code."
    )]
    Run(CliArguments),
    #[command(name = "serve", about = "Serves a file on the cloudstate runtime")]
    Serve(CliArguments),
    #[command(name = "gc", about = "Runs the garbage collector on a database file")]
    Gc(GcArguments),
    #[command(name = "backup", about = "Backs up a database file")]
    Backup(BackupArguments),
}

#[tokio::main]
async fn main() {
    // #[cfg(feature = "dhat-heap")]
    // let _profiler = dhat::Profiler::new_heap();

    tracing_subscriber::fmt::init();

    debug!("Starting cloudstate");

    match Cli::parse() {
        Cli::Run(CliArguments {
            filename,
            memory_only,
            ..
        }) => {
            let script = fs::read_to_string(filename).unwrap();

            let db = if memory_only {
                Database::builder()
                    .create_with_backend(backends::InMemoryBackend::default())
                    .unwrap()
            } else {
                Database::create("./cloudstate").unwrap()
            };

            let engine: Arc<dyn CloudstateBlobStorageEngine> = if memory_only {
                Arc::new(InMemoryBlobStore::new())
            } else {
                Arc::new(FsBlobStore::new("./cloudstate-blobs".into()))
            };

            let blob_storage = CloudstateBlobStorage::new(engine);

            // todo get output
            let result = execute_script(
                &format!(
                    "try {{
                    {script}
                }} catch (e) {{
                    globalThis.result = {{
                        error: {{
                            message: e.message,
                            stack: e.stack,
                        }}
                    }}
                }}"
                ),
                "",
                ReDBCloudstate::new(Arc::new(Mutex::new(db))),
                blob_storage,
                ServerInfo {
                    deployment_id: None,
                },
            )
            .await;

            debug!("{result}");
        }
        Cli::Serve(CliArguments {
            filename,
            watch,
            memory_only,
        }) => {
            let env: HashMap<String, String> = std::env::vars().collect();

            let db = if memory_only {
                Database::builder()
                    .create_with_backend(backends::InMemoryBackend::default())
                    .unwrap()
            } else {
                Database::create("./cloudstate").unwrap()
            };

            let blob_storage_engine: Arc<dyn CloudstateBlobStorageEngine> = if memory_only {
                Arc::new(InMemoryBlobStore::new())
            } else {
                // cwd is the current working directory
                Arc::new(FsBlobStore::new(
                    std::env::current_dir().unwrap().join("cloudstate-blobs"),
                ))
            };

            let blob_storage = CloudstateBlobStorage::new(blob_storage_engine.clone());

            let classes = fs::read_to_string(&filename).unwrap_or("".to_string());
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            let cloudstate = ReDBCloudstate::new(Arc::new(Mutex::new(db)));
            let server = CloudstateServer::new(
                cloudstate.clone(),
                blob_storage.clone(),
                &classes,
                env.clone(),
                "http://localhost:8910/__invalidate__".to_string(),
                SimpleCloudstateRunner::new(),
                ServerInfo {
                    deployment_id: None,
                },
            )
            .await;

            let app_state = Arc::new(RwLock::new(server));

            let cloned = Arc::clone(&app_state);
            let other_thread = tokio::spawn(async move {
                info!("Starting server on {:?}", listener.local_addr().unwrap());
                let _ = run_server(cloned, listener).await;
            });

            if watch {
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
                            info!("Reloading Cloudstate");

                            Runtime::new().unwrap().block_on(async {
                                if let Ok(new_classes) = fs::read_to_string(&pre_cloned_filename) {
                                    let mut server = app_state.write().await;

                                    *server = CloudstateServer::new(
                                        cloudstate.clone(),
                                        blob_storage.clone(),
                                        &new_classes,
                                        env.clone(),
                                        "http://localhost:8910/__invalidate__".to_string(),
                                        SimpleCloudstateRunner::new(),
                                        ServerInfo {
                                            deployment_id: None,
                                        },
                                    )
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
        Cli::Gc(GcArguments { filename }) => {
            let metadata_before = fs::metadata(filename.clone()).unwrap();

            if let Ok(mut cloudstate) = Database::open(filename.clone()) {
                info!("Running garbage collection");
                match mark_and_sweep(&cloudstate) {
                    Ok(_) => {
                        info!("Garbage collection complete");
                    }
                    Err(e) => {
                        info!("Error running garbage collection: {:?}", e);
                    }
                }

                info!("Compacting database");

                match cloudstate.compact() {
                    Ok(_) => {
                        info!("Database compacted");
                    }
                    Err(_) => {
                        info!("Failed to compact database");
                    }
                }
            } else {
                info!("Failed to open file");
                return;
            }

            let metadata_after = fs::metadata(filename.clone()).unwrap();

            // info!("File size before: {}", metadata_before.size());
            // info!("File size after: {}", metadata_after.size());
            let megabytes_before = metadata_before.size() / 1024 / 1024;
            let megabytes_after = metadata_after.size() / 1024 / 1024;
            let megabytes_saved = megabytes_before - megabytes_after;
            info!(
                "File size reduced from {}MB to {}MB, saving {}MB",
                megabytes_before, megabytes_after, megabytes_saved
            )
        }
        Cli::Backup(BackupArguments {
            filename,
            backup_filename,
        }) => {
            let db = match Database::open(filename.clone()) {
                Ok(db) => db,
                Err(e) => {
                    if matches!(e, redb::DatabaseError::Storage(_)) {
                        info!("We couldn't find a database file at {:?}", filename);
                        return;
                    } else {
                        info!("Failed to open file: {:?}", e);
                        return;
                    }
                }
            };

            let cloudstate = ReDBCloudstate::new(Arc::new(Mutex::new(db)));
            let style = indicatif::ProgressStyle::default_bar()
                .template("{msg:<18} {bar:40.green/white} ({pos}/{len})")
                .unwrap();
            // .progress_chars("=>|");
            let mut current_progress_bar = ProgressBarState::None;
            cloudstate
                .backup(
                    backup_filename.clone(),
                    &mut Some(Box::new(|progress: BackupProgress| {
                        if let Some(current_table) = progress.tables.last() {
                            let (table_name, table_progress) = current_table;
                            match &mut current_progress_bar {
                                ProgressBarState::None => {
                                    let bar = ProgressBar::new(table_progress.total)
                                        .with_style(style.clone())
                                        .with_message(format!("Backing up {}", table_name));

                                    current_progress_bar =
                                        ProgressBarState::Named(table_name.clone(), bar);
                                }
                                ProgressBarState::Named(name, pb) => {
                                    if name != table_name {
                                        pb.finish();
                                        current_progress_bar = ProgressBarState::Named(
                                            table_name.clone(),
                                            ProgressBar::new(table_progress.total)
                                                .with_style(style.clone())
                                                .with_message(format!("Backing up {}", table_name)),
                                        );
                                    } else {
                                        pb.set_position(table_progress.current);
                                        if table_progress.current == table_progress.total {
                                            pb.finish();
                                        }
                                    }
                                }
                            }
                        }

                        // progress_bar_height = progress.tables.len();
                    })),
                )
                .unwrap();
        }
    };
}

pub enum ProgressBarState {
    None,
    Named(String, ProgressBar),
}

async fn run_server(
    server: Arc<RwLock<CloudstateServer<SimpleCloudstateRunner>>>,
    listener: TcpListener,
) {
    let handle = |req: Request| async move {
        debug!("{}: {}", req.method().to_string(), req.uri().to_string());
        tokio::task::spawn_blocking(move || handler(server.clone(), req))
            .await
            .unwrap()
    };

    let svr = axum::Router::new()
        .route("/cloudstate/status", get(|| async { Json("OK") }))
        .fallback(
            get(handle.clone())
                .post(handle.clone())
                .delete(handle.clone())
                .put(handle.clone())
                .patch(handle.clone()),
        )
        .layer(DefaultBodyLimit::disable());

    let out = axum::serve(listener, svr);

    out.await.unwrap();
}

#[tokio::main(flavor = "current_thread")]
async fn handler(
    server: Arc<RwLock<CloudstateServer<SimpleCloudstateRunner>>>,
    req: Request<Body>,
) -> axum::http::Response<Body> {
    debug!("Pulling service");

    let server = server.read().await;
    let router = server.router.clone();

    drop(server);

    let mut service: axum::routing::RouterIntoService<Body> = router.into_service();

    debug!("Preparing service");

    poll_fn(|cx: &mut std::task::Context<'_>| {
        <axum::routing::RouterIntoService<Body>>::poll_ready(&mut service, cx)
    })
    .await
    .unwrap();

    debug!("Calling service");

    let response = service.call(req).await.unwrap();

    debug!("Returning response");

    return response;
}
