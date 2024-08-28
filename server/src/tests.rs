use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use cloudstate_runtime::extensions::cloudstate::ReDBCloudstate;
use http_body_util::BodyExt;
use serde_json::json;
use std::sync::{Arc, Mutex};
use tower::util::ServiceExt;

#[tokio::test]
async fn test_fetch() {
    let router = crate::CloudstateServer::new(
        Arc::new(Mutex::new(ReDBCloudstate {
            db: redb::Database::builder()
                .create_with_backend(redb::backends::InMemoryBackend::default())
                .unwrap(),
            transactions: std::collections::HashMap::new(),
        })),
        r"export class CounterCS {
            static id = 'counter';
            count = 0;
            increment() {
                return ++this.count;
            }
        }",
    )
    .await;

    let response = router
        .router
        .oneshot(
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

    assert_eq!(
        serde_json::to_value(&body[..]).unwrap(),
        json!({
            "result": 1
        })
    );
}
