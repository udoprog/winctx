use std::env::current_exe;
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::Path;

use crate::convert::encode_escaped_os_str;
use crate::error::Error;
use crate::error::ErrorKind::*;
use crate::registry::OpenRegistryKey;
use crate::Result;

/// Helper to register and qeury for a binary to autostart.
#[non_exhaustive]
pub struct AutoStart {
    name: Box<OsStr>,
    executable: Box<Path>,
    arguments: Vec<OsString>,
}

impl AutoStart {
    /// Helper to make the current executable automatically start.
    pub fn current_exe<N>(name: N) -> Result<Self>
    where
        N: AsRef<OsStr>,
    {
        let executable = current_exe().map_err(CurrentExecutable)?;
        Ok(Self::new(name, executable))
    }

    /// Construct a new auto start helper.
    ///
    /// The name should be something suitable for a registry key, like
    /// `OxidizeBot`. Note that in the registry it is case-insensitive.
    #[inline]
    pub fn new<N, E>(name: N, executable: E) -> Self
    where
        N: AsRef<OsStr>,
        E: AsRef<Path>,
    {
        Self {
            name: name.as_ref().into(),
            executable: executable.as_ref().into(),
            arguments: Vec::new(),
        }
    }

    /// Append arguments to the executable when autostarting.
    pub fn arguments<A>(&mut self, arguments: A)
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
        let key = OpenRegistryKey::current_user()
            .open("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(OpenRegistryKey)?;

        let path = match key.get_string(&self.name) {
            Ok(path) => path,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(false),
            Err(e) => return Err(Error::new(GetRegistryValue(e))),
        };

        Ok(self.registry_entry()?.as_str() == path)
    }

    /// Install the current executable to be automatically started.
    pub fn install(&self) -> Result<()> {
        let key = OpenRegistryKey::current_user()
            .set_value()
            .open("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(OpenRegistryKey)?;
        key.set(&self.name, self.registry_entry()?)
            .map_err(SetRegistryKey)?;
        Ok(())
    }

    /// Remove the program from automatic startup.
    pub fn uninstall(&self) -> Result<()> {
        let key = OpenRegistryKey::current_user()
            .set_value()
            .open("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
            .map_err(OpenRegistryKey)?;
        key.delete(&self.name).map_err(DeleteRegistryKey)?;
        Ok(())
    }
}
