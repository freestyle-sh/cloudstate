use std::path::{Path, PathBuf};

use deno_fetch::FetchPermissions;
use deno_net::NetPermissions;
use deno_runtime::deno_permissions::PermissionCheckError;
use deno_web::TimersPermission;
use tracing::debug;

pub struct CloudstatePermissions {}

impl TimersPermission for CloudstatePermissions {
    fn allow_hrtime(&mut self) -> bool {
        false
    }
}

impl FetchPermissions for CloudstatePermissions {
    fn check_net_url(
        &mut self,
        _url: &url::Url,
        _api_name: &str,
    ) -> Result<(), PermissionCheckError> {
        debug!("checking net url fetch permission");
        Ok(())
    }

    fn check_read<'a>(
        &mut self,
        resolved: bool,
        p: &'a Path,
        api_name: &str,
    ) -> Result<std::borrow::Cow<'a, Path>, deno_fs::FsError> {
        debug!("checking read fetch permission");
        Ok(p.to_path_buf().into())
    }

    // fn check_read<'a>(
    //     &mut self,
    //     p: &'a Path,
    //     api_name: &str,
    // ) -> Result<std::borrow::Cow<'a, Path>, PermissionCheckError> {
    //     debug!("checking read fetch permission");
    //     Ok(p.to_path_buf().into())
    // }

    // fn check_read<'a>(
    //     &mut self,
    //     p: &'a Path,
    //     _api_name: &str,
    //     _
    // ) -> Result<std::borrow::Cow<'a, Path>, PermissionCheckError> {
    //     debug!("checking read fetch permission");
    //     Ok(p.to_path_buf().into())
    // }

    // fn check_read<'a>(
    //     &mut self,
    //     p: &'a Path,
    //     _api_name: &str,
    // ) -> Result<std::borrow::Cow<'a, Path>, error::AnyError> {
    //     debug!("checking read fetch permission");
    //     Ok(p.to_path_buf().into())
    // }
}

impl NetPermissions for CloudstatePermissions {
    fn check_net<T: AsRef<str>>(
        &mut self,
        _host: &(T, Option<u16>),
        _api_name: &str,
    ) -> Result<(), PermissionCheckError> {
        debug!("checking net permission");
        Ok(())
    }

    fn check_read(&mut self, p: &str, _api_name: &str) -> Result<PathBuf, PermissionCheckError> {
        debug!("checking read permission");
        Ok(p.to_string().into())
    }

    fn check_write(&mut self, p: &str, _api_name: &str) -> Result<PathBuf, PermissionCheckError> {
        debug!("checking write permission");
        Ok(p.to_string().into())
    }

    fn check_write_path<'a>(
        &mut self,
        p: &'a std::path::Path,
        _api_name: &str,
    ) -> Result<std::borrow::Cow<'a, std::path::Path>, PermissionCheckError> {
        debug!("checking write path permission");
        Ok(p.to_path_buf().into())
    }
}
