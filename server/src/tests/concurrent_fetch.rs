use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cloudstate_runtime::extensions::cloudstate::ReDBCloudstate;
use http_body_util::BodyExt;
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tower::{util::ServiceExt, Service};

#[tokio::test]
async fn test_concurrent_fetch() {
    let mut router = crate::CloudstateServer::new(
        ReDBCloudstate::new(Arc::new(Mutex::new(
            redb::Database::builder()
                .create_with_backend(redb::backends::InMemoryBackend::default())
                .unwrap(),
        ))),
        r"export class FetcherCS {
            static id = 'test';
            count = 0;
            async tryFetch() {
                await new Promise(resolve => setTimeout(resolve, 10000));
            }
        }",
        HashMap::new(),
    )
    .await;

    let response = ServiceExt::<Request<Body>>::ready(&mut router.router)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/cloudstate/instances/test/tryFetch")
                .method("POST")
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

    let response_2 = ServiceExt::<Request<Body>>::ready(&mut router.router)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/cloudstate/instances/test/tryFetch")
                .method("POST")
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

    assert_eq!(response.status(), StatusCode::OK);

    let _body = response.into_body().collect().await.unwrap().to_bytes();
    let _body = response_2.into_body().collect().await.unwrap().to_bytes();
    // let body_str = String::from_utf8(body.to_vec()).unwrap();
    // let body_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    // assert_eq!(
    //     body_json,
    //     json!({
    //         "result": 1
    //     })
    // );
}
