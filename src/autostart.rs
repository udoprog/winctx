use crate::error::ErrorKind::*;
use crate::registry::RegistryKey;
use crate::Result;

/// Helper to register and qeury for a binary to autostart.
#[non_exhaustive]
pub struct AutoStart;

impl AutoStart {
    /// Construct a new auto start helper.
    pub fn new() -> Self {
        Self
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

        let path = match key.get("OxidizeBot").map_err(GetRegistryValue)? {
            Some(path) => path,
            None => return Ok(false),
        };

        Ok(self.run_registry_entry()?.as_str() == path)
    }

    pub fn install(&self) -> Result<()> {
        let key = RegistryKey::current_user("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(GetRegistryKey)?;

        key.set("OxidizeBot", self.run_registry_entry()?)
            .map_err(SetRegistryKey)?;
        Ok(())
    }

    pub fn uninstall(&self) -> Result<()> {
        let key = RegistryKey::current_user("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(GetRegistryKey)?;

        key.delete("OxidizeBot").map_err(DeleteRegistryKey)?;
        Ok(())
    }
}
