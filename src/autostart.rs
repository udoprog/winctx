use std::env::current_exe;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::Path;

use crate::convert::encode_escaped_os_str;
use crate::error::ErrorKind::*;
use crate::registry::RegistryKey;
use crate::Result;

/// Helper to register and qeury for a binary to autostart.
#[non_exhaustive]
pub struct AutoStart {
    name: Box<str>,
    executable: Box<Path>,
    arguments: Vec<OsString>,
}

impl AutoStart {
    /// Helper to make the current executable automatically start.
    pub fn current_exe<N>(name: N) -> Result<Self>
    where
        N: fmt::Display,
    {
        let executable = current_exe().map_err(CurrentExecutable)?;
        Ok(Self::new(name, executable))
    }

    /// Construct a new auto start helper.
    ///
    /// The name should be something suitable for a registry key, like
    /// `OxidizeBot`.
    #[inline]
    pub fn new<N, E>(name: N, executable: E) -> Self
    where
        N: fmt::Display,
        E: AsRef<Path>,
    {
        Self {
            name: name.to_string().into(),
            executable: executable.as_ref().into(),
            arguments: Vec::new(),
        }
    }

    /// Append arguments to the executable when autostarting.
    pub fn with_arguments<A>(&mut self, arguments: A)
    where
        A: IntoIterator,
        A::Item: AsRef<OsStr>,
    {
        self.arguments = arguments
            .into_iter()
            .map(|a| a.as_ref().to_os_string())
            .collect();
    }
}

impl AutoStart {
    /// Entry for automatic startup.
    fn registry_entry(&self) -> Result<String> {
        let mut entry = String::new();

        encode_escaped_os_str(&mut entry, self.executable.as_os_str())
            .map_err(BadAutoStartExecutable)?;

        for argument in &self.arguments {
            entry.push(' ');
            encode_escaped_os_str(&mut entry, argument).map_err(BadAutoStartArgument)?;
        }

        Ok(entry)
    }

    /// If the program is installed to run at startup.
    pub fn is_installed(&self) -> Result<bool> {
        let key = RegistryKey::current_user("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(GetRegistryKey)?;

        let path = match key.get(&self.name).map_err(GetRegistryValue)? {
            Some(path) => path,
            None => return Ok(false),
        };

        Ok(self.registry_entry()?.as_str() == path)
    }

    /// Install the current executable to be automatically started.
    pub fn install(&self) -> Result<()> {
        let key = RegistryKey::current_user("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(GetRegistryKey)?;

        key.set(&self.name, self.registry_entry()?)
            .map_err(SetRegistryKey)?;
        Ok(())
    }

    /// Remove the program from automatic startup.
    pub fn uninstall(&self) -> Result<()> {
        let key = RegistryKey::current_user("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(GetRegistryKey)?;

        key.delete(&self.name).map_err(DeleteRegistryKey)?;
        Ok(())
    }
}
