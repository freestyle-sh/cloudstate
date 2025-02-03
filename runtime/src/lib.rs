pub mod backup;
pub mod bincode;
pub mod blob_storage;
pub mod cloudstate_extensions;
pub mod execution;
pub mod extensions;
pub mod gc;
pub mod permissions;
pub mod print;
pub mod tables;
pub mod transpile;

#[macro_use]
pub mod v8_macros;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct ServerInfo {
    pub deployment_id: Option<String>,
}
