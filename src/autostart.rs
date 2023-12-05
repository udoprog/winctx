use std::fmt;

use crate::error::ErrorKind::*;
use crate::registry::RegistryKey;
use crate::Result;

/// Helper to register and qeury for a binary to autostart.
#[non_exhaustive]
pub struct AutoStart {
    name: Box<str>,
}

impl AutoStart {
    /// Construct a new auto start helper.
    ///
    /// The name should be something suitable for a registry key, like
    /// `OxidizeBot`.
    #[inline]
    pub fn new<N>(name: N) -> Self where N: fmt::Display {
        Self {
            name: name.to_string().into()
        }
    }
}

impl AutoStart {
    /// Entry for automatic startup.
    fn run_registry_entry(&self) -> Result<String> {
        let exe = std::env::current_exe().map_err(CurrentExecutable)?;
        let exe = exe.to_str().ok_or(BadExecutable)?;
        Ok(format!("\"{}\" --silent", exe))
    }

    /// If the program is installed to run at startup.
    pub fn is_installed(&self) -> Result<bool> {
        let key = RegistryKey::current_user("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(GetRegistryKey)?;

        let path = match key.get(&self.name).map_err(GetRegistryValue)? {
            Some(path) => path,
            None => return Ok(false),
        };

        Ok(self.run_registry_entry()?.as_str() == path)
    }

    pub fn install(&self) -> Result<()> {
        let key = RegistryKey::current_user("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(GetRegistryKey)?;

        key.set(&self.name, self.run_registry_entry()?)
            .map_err(SetRegistryKey)?;
        Ok(())
    }

    pub fn uninstall(&self) -> Result<()> {
        let key = RegistryKey::current_user("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(GetRegistryKey)?;

        key.delete(&self.name).map_err(DeleteRegistryKey)?;
        Ok(())
    }
}
