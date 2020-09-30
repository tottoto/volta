use std::ffi::OsString;

use super::executor::{ToolCommand, ToolKind};
use super::{debug_active_image, debug_no_platform};
use crate::error::{ErrorKind, Fallible};
use crate::platform::{Platform, System};
use crate::session::Session;
use lazy_static::lazy_static;
use semver::Version;

lazy_static! {
    /// The minimum required npm version that includes npx (5.2.0)
    static ref REQUIRED_NPM_VERSION: Version = Version::new(5, 2, 0);
}

/// Build a `ToolCommand` for npx
pub(super) fn command(args: &[OsString], session: &mut Session) -> Fallible<ToolCommand> {
    let platform = Platform::current(session)?;

    Ok(ToolCommand::new("npx", args, platform, ToolKind::Npx))
}

/// Determine the execution context (PATH and failure error message) for npx
pub(super) fn execution_context(
    platform: Option<Platform>,
    session: &mut Session,
) -> Fallible<(OsString, ErrorKind)> {
    match platform {
        Some(plat) => {
            let image = plat.checkout(session)?;

            // If the npm version is lower than the minimum required, we can show a helpful error
            // message instead of a 'command not found' error.
            let active_npm = image.resolve_npm()?;
            if active_npm.value < *REQUIRED_NPM_VERSION {
                return Err(ErrorKind::NpxNotAvailable {
                    version: active_npm.value.to_string(),
                }
                .into());
            }

            let path = image.path()?;
            debug_active_image(&image);

            Ok((path, ErrorKind::BinaryExecError))
        }
        None => {
            let path = System::path()?;
            debug_no_platform();
            Ok((path, ErrorKind::NoPlatform))
        }
    }
}