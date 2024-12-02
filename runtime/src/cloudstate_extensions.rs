use std::sync::Arc;

use crate::{
    extensions::{bootstrap::bootstrap, cloudstate::cloudstate},
    permissions::CloudstatePermissions,
};

pub fn cloudstate_extensions() -> Vec<deno_core::Extension> {
    let deno_blob_storage = Arc::new(deno_web::BlobStore::default());

    vec![
        deno_webidl::deno_webidl::init_ops_and_esm(),
        deno_telemetry::deno_telemetry::init_ops_and_esm(),
        deno_url::deno_url::init_ops_and_esm(),
        deno_console::deno_console::init_ops_and_esm(),
        deno_web::deno_web::init_ops_and_esm::<CloudstatePermissions>(deno_blob_storage, None),
        deno_crypto::deno_crypto::init_ops_and_esm(None),
        bootstrap::init_ops_and_esm(),
        deno_fetch::deno_fetch::init_ops_and_esm::<CloudstatePermissions>(Default::default()),
        deno_net::deno_net::init_ops_and_esm::<CloudstatePermissions>(None, None),
        cloudstate::init_ops_and_esm(),
    ]
}
