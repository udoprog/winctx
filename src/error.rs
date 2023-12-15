use std::char::DecodeUtf16Error;
use std::fmt;
use std::io;

/// The error raised by this library.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Construct a new error.
    pub(super) fn new<K>(kind: K) -> Self
    where
        ErrorKind: From<K>,
    {
        Self { kind: kind.into() }
    }
}

impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::WindowSetup(..) => write!(f, "Failed to set up window"),
            ErrorKind::WindowError(..) => write!(f, "Error in window thread"),
            ErrorKind::DeleteRegistryKey(..) => write!(f, "Failed to delete registry key"),
            ErrorKind::GetRegistryValue(..) => write!(f, "Failed to get registry value"),
            ErrorKind::SetRegistryKey(..) => write!(f, "Failed to set registry key"),
            ErrorKind::CurrentExecutable(..) => write!(f, "Could not get current executable"),
            ErrorKind::SetupMenu(..) => write!(f, "Failed to setup menu"),
            ErrorKind::BuildIcon(..) => write!(f, "Failed to construct icon"),
            ErrorKind::BuildErrorIcon(..) => write!(f, "Failed to construct error icon"),
            ErrorKind::SetIcon(..) => write!(f, "Failed to set icon from buffer"),
            ErrorKind::SetTooltip(..) => write!(f, "Failed to set tooltip message"),
            ErrorKind::SendNotification(..) => write!(f, "Failed to send notification"),
            ErrorKind::CreateMutex(..) => write!(f, "Failed to construct mutex"),
            ErrorKind::OpenRegistryKey(..) => write!(f, "Failed to open registry key"),
            ErrorKind::MissingNotification => write!(f, "Missing notification state"),
            ErrorKind::BadAutoStartExecutable(..) => write!(f, "Bad autostart executable"),
            ErrorKind::BadAutoStartArgument(..) => write!(f, "Bad autostart argument"),
            ErrorKind::WindowClosed => write!(f, "Window has been closed"),
            ErrorKind::PostMessageDestroy => write!(f, "Failed to post destroy window message"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::WindowSetup(error) => Some(error),
            ErrorKind::WindowError(error) => Some(error),
            ErrorKind::DeleteRegistryKey(error) => Some(error),
            ErrorKind::GetRegistryValue(error) => Some(error),
            ErrorKind::SetRegistryKey(error) => Some(error),
            ErrorKind::CurrentExecutable(error) => Some(error),
            ErrorKind::SetupMenu(error) => Some(error),
            ErrorKind::BuildIcon(error) => Some(error),
            ErrorKind::BuildErrorIcon(error) => Some(error),
            ErrorKind::SetIcon(error) => Some(error),
            ErrorKind::SetTooltip(error) => Some(error),
            ErrorKind::SendNotification(error) => Some(error),
            ErrorKind::CreateMutex(error) => Some(error),
            ErrorKind::OpenRegistryKey(error) => Some(error),
            ErrorKind::BadAutoStartExecutable(error) => Some(error),
            ErrorKind::BadAutoStartArgument(error) => Some(error),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(super) enum WindowError {
    Init(io::Error),
    DeleteIcon(io::Error),
    AddClipboardFormatListener(io::Error),
    OpenClipboard(io::Error),
    GetClipboardData(io::Error),
    LockClipboardData(io::Error),
    ThreadPanicked,
    ThreadExited,
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WindowError::Init(..) => write!(f, "Failed to initialize window"),
            WindowError::DeleteIcon(..) => write!(f, "Failed to delete icon"),
            WindowError::AddClipboardFormatListener(..) => {
                write!(f, "Failed to add clipboard format listener")
            }
            WindowError::OpenClipboard(..) => write!(f, "Failed to open clipboard"),
            WindowError::GetClipboardData(..) => write!(f, "Failed to get clipboard data"),
            WindowError::LockClipboardData(..) => write!(f, "Failed to lock clipboard data"),
            WindowError::ThreadPanicked => write!(f, "Window thread panicked"),
            WindowError::ThreadExited => write!(f, "Window thread unexpectedly exited"),
        }
    }
}

impl std::error::Error for WindowError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WindowError::Init(error) => Some(error),
            WindowError::DeleteIcon(error) => Some(error),
            WindowError::AddClipboardFormatListener(error) => Some(error),
            WindowError::OpenClipboard(error) => Some(error),
            WindowError::GetClipboardData(error) => Some(error),
            WindowError::LockClipboardData(error) => Some(error),
            WindowError::ThreadPanicked => None,
            WindowError::ThreadExited => None,
        }
    }
}

#[derive(Debug)]
pub(super) enum ErrorKind {
    WindowSetup(WindowError),
    WindowError(WindowError),
    DeleteRegistryKey(io::Error),
    GetRegistryValue(io::Error),
    SetRegistryKey(io::Error),
    CurrentExecutable(io::Error),
    SetupMenu(SetupMenuError),
    BuildIcon(io::Error),
    BuildErrorIcon(io::Error),
    SetIcon(io::Error),
    SetTooltip(io::Error),
    SendNotification(io::Error),
    CreateMutex(io::Error),
    OpenRegistryKey(io::Error),
    MissingNotification,
    BadAutoStartExecutable(DecodeUtf16Error),
    BadAutoStartArgument(DecodeUtf16Error),
    WindowClosed,
    PostMessageDestroy,
}

#[derive(Debug)]
pub(super) enum SetupMenuError {
    AddMenuEntry(usize, io::Error),
    AddMenuSeparator(usize, io::Error),
}

impl fmt::Display for SetupMenuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetupMenuError::AddMenuEntry(index, ..) => {
                write!(f, "Failed to add menu entry {index}")
            }
            SetupMenuError::AddMenuSeparator(index, ..) => {
                write!(f, "Failed to add menu separator {index}")
            }
        }
    }
}

impl std::error::Error for SetupMenuError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SetupMenuError::AddMenuEntry(_, error) => Some(error),
            SetupMenuError::AddMenuSeparator(_, error) => Some(error),
        }
    }
}
