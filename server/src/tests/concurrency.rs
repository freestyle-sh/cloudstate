use axum::{
    body::Body,
    http::{self, Request},
};
use cloudstate_runtime::extensions::cloudstate::ReDBCloudstate;
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

#[tokio::test]
async fn the_real_deal() {
    let mut router = crate::CloudstateServer::new(
        ReDBCloudstate::new(Arc::new(Mutex::new(
            redb::Database::builder()
                .create_with_backend(redb::backends::InMemoryBackend::default())
                .unwrap(),
        ))),
        r"export class CounterCS {
                static id = 'counter';
                count = 0;
                async increment() {
                    console.log('incrementing');
                    await new Promise(resolve => setTimeout(resolve, 5000));
                    console.log('incremented');
                    return ++this.count;
                }
            }",
        HashMap::new(),
    )
    .await;

    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, router.router).await.unwrap();
    });

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build_http();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{addr}"))
                .header("Host", "localhost")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_vec(&json!({
                        "params": []
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], b"Hello, World!");
}
