use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cloudstate_runtime::{
    blob_storage::{in_memory_store::InMemoryBlobStore, CloudstateBlobStorage},
    extensions::cloudstate::ReDBCloudstate,
    print::print_database,
    ServerInfo,
};
use http_body_util::BodyExt;
use serde_json::json;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tower::{util::ServiceExt, Service};

use crate::{cloudstate_runner::simple::SimpleCloudstateRunner, CloudstateServer};

// mod concurrency;
mod fetch_method;

#[tokio::test]
async fn test_method_request() {
    let _ = tracing_subscriber::fmt::try_init();

    let cloudstate = ReDBCloudstate::new(Arc::new(Mutex::new(
        redb::Database::builder()
            .create_with_backend(redb::backends::InMemoryBackend::default())
            .unwrap(),
    )));

    let mut router = CloudstateServer::new(
        cloudstate.clone(),
        CloudstateBlobStorage::new(Arc::new(InMemoryBlobStore::default())),
        r"export class CounterCS {
            static id = 'counter';
            count = 0;
            increment() {
                return ++this.count;
            }
        }",
        HashMap::new(),
        "http://localhost:8910/__invalidate__".to_string(),
        SimpleCloudstateRunner::new(),
        ServerInfo {
            deployment_id: None,
            domain: None,
        },
    )
    .await;

    print_database(&cloudstate.get_database_mut());

    let response = ServiceExt::<Request<Body>>::ready(&mut router.router)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/cloudstate/instances/counter/increment")
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

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let body_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    assert_eq!(
        body_json,
        json!({
            "result": 1
        })
    );

    let response = ServiceExt::<Request<Body>>::ready(&mut router.router)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/cloudstate/instances/counter/increment")
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

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let body_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    assert_eq!(
        body_json,
        json!({
            "result": 2
        })
    );
}

#[tokio::test]
async fn test_async_write() {
    let _ = tracing_subscriber::fmt::try_init();

    let mut router = CloudstateServer::new(
        ReDBCloudstate::new(Arc::new(Mutex::new(
            redb::Database::builder()
                .create_with_backend(redb::backends::InMemoryBackend::default())
                .unwrap(),
        ))),
        CloudstateBlobStorage::new(Arc::new(InMemoryBlobStore::default())),
        r#"export class DelayedCounter {
            static id = 'delayed-counter';
            count = 0;
            async increment() {
                await new Promise(resolve => setTimeout(resolve, 1000));
                return ++this.count;
            }
            getCount() {
                return this.count;
            }
        }"#,
        HashMap::new(),
        "http://localhost:8910/__invalidate__".to_string(),
        SimpleCloudstateRunner::new(),
        ServerInfo {
            deployment_id: None,
            domain: None,
        },
    )
    .await;

    let increment_response = ServiceExt::<Request<Body>>::ready(&mut router.router)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/cloudstate/instances/delayed-counter/increment")
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

    assert_eq!(increment_response.status(), StatusCode::OK);

    let body = increment_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let body_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    assert_eq!(
        body_json,
        json!({
            "result": 1,
        })
    );

    let get_count_response = ServiceExt::<Request<Body>>::ready(&mut router.router)
        .await
        .unwrap()
        .call(
            Request::builder()
                .uri("/cloudstate/instances/delayed-counter/getCount")
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

    let body = get_count_response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let body_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    assert_eq!(
        body_json,
        json!({
            "result": 1,
        })
    );
}
