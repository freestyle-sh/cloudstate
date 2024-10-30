use axum::{
    body::Body,
    http::{self, Request},
};
use cloudstate_runtime::extensions::cloudstate::ReDBCloudstate;
use http_body_util::BodyExt;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tower::{util::ServiceExt, Service};

#[tokio::test]
async fn test_fetch_request() {
    tracing_subscriber::fmt::init();

    let cloudstate = ReDBCloudstate::new(Arc::new(Mutex::new(
        redb::Database::builder()
            .create_with_backend(redb::backends::InMemoryBackend::default())
            .unwrap(),
    )));

    let mut router = crate::CloudstateServer::new(
        cloudstate,
        r#"export class CounterCS {
            static id = 'fetch-test';
            fetch() {
                return new Response('Hello, World!');
            }
        }"#,
        HashMap::new(),
        "http://localhost:8910/__invalidate__".to_string(),
    )
    .await;

    let response = ServiceExt::<Request<Body>>::ready(&mut router.router)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/cloudstate/instances/fetch-test")
                .method("GET")
                .header(http::header::HOST, "localhost")
                .body(Body::empty()) // Assuming `Body::empty()` is the correct usage here
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(body_str, "Hello, World!");
}
