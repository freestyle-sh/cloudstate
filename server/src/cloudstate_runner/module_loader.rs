use std::sync::{Arc, RwLock};

use deno_core::{
    error::ModuleLoaderError, resolve_import, ModuleLoadResponse, ModuleLoader, ModuleSource,
    ModuleSourceCode, ModuleSpecifier, ModuleType, ResolutionKind,
};

pub enum CloudstateModuleLoaderLibrary {
    Sync(String),
    Async(tokio::sync::oneshot::Receiver<String>),
}
pub struct CloudstateModuleLoader {
    lib: Arc<std::sync::Mutex<CloudstateModuleLoaderLibrary>>,
}

impl CloudstateModuleLoader {
    pub fn new(lib: String) -> Self {
        Self {
            lib: Arc::new(std::sync::Mutex::new(CloudstateModuleLoaderLibrary::Sync(
                lib,
            ))),
        }
    }
    pub fn new_async(lib: tokio::sync::oneshot::Receiver<String>) -> Self {
        Self {
            lib: Arc::new(std::sync::Mutex::new(CloudstateModuleLoaderLibrary::Async(
                lib,
            ))),
        }
    }
}

impl ModuleLoader for CloudstateModuleLoader {
    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> deno_core::ModuleLoadResponse {
        let mut lib_lock = self.lib.lock().expect("failed to lock lib");

        let replaced_lib = std::mem::replace(
            &mut *lib_lock,
            CloudstateModuleLoaderLibrary::Sync(String::new()),
        );
        match replaced_lib {
            CloudstateModuleLoaderLibrary::Sync(lib) => {
                ModuleLoadResponse::Sync(Ok(ModuleSource::new(
                    ModuleType::JavaScript,
                    ModuleSourceCode::String(lib.into()),
                    module_specifier,
                    None,
                )))
            }
            CloudstateModuleLoaderLibrary::Async(receiver) => {
                let specifier = module_specifier.clone();
                let lib_arc = Arc::clone(&self.lib);
                ModuleLoadResponse::Async(Box::pin(async move {
                    let lib = receiver.await.expect("failed to receive lib");
                    {
                        let mut lock = lib_arc.lock().expect("failed to lock lib");
                        *lock = CloudstateModuleLoaderLibrary::Sync(lib.clone());
                    }
                    Ok(ModuleSource::new(
                        ModuleType::JavaScript,
                        ModuleSourceCode::String(lib.into()),
                        &specifier,
                        None,
                    ))
                }))
            }
        }
        // *lib_lock = CloudstateModuleLoaderLibrary::Sync(lib.clone());
    }
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, ModuleLoaderError> {
        Ok(resolve_import(specifier, referrer)?)
    }
}
