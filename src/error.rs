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
            ErrorKind::ThreadError(..) => write!(f, "Error in window thread"),
            ErrorKind::ClipboardPoll(..) => write!(f, "Failed to poll clipboard"),
            ErrorKind::DeleteRegistryKey(..) => write!(f, "Failed to delete registry key"),
            ErrorKind::GetRegistryValue(..) => write!(f, "Failed to get registry value"),
            ErrorKind::SetRegistryKey(..) => write!(f, "Failed to set registry key"),
            ErrorKind::CurrentExecutable(..) => write!(f, "Could not get current executable"),
            ErrorKind::BuildPopupMenu(..) => write!(f, "Failed to build popup menu"),
            ErrorKind::SetupIcons(..) => write!(f, "Failed to setup icons"),
            ErrorKind::SetupMenu(..) => write!(f, "Failed to setup menu"),
            ErrorKind::AddNotification(..) => write!(f, "Failed to add notification area"),
            ErrorKind::ModifyNotification(..) => write!(f, "Failed to modify notification area"),
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
            ErrorKind::ThreadError(error) => Some(error),
            ErrorKind::ClipboardPoll(error) => Some(error),
            ErrorKind::DeleteRegistryKey(error) => Some(error),
            ErrorKind::GetRegistryValue(error) => Some(error),
            ErrorKind::SetRegistryKey(error) => Some(error),
            ErrorKind::CurrentExecutable(error) => Some(error),
            ErrorKind::BuildPopupMenu(error) => Some(error),
            ErrorKind::SetupIcons(error) => Some(error),
            ErrorKind::SetupMenu(error) => Some(error),
            ErrorKind::AddNotification(error) => Some(error),
            ErrorKind::ModifyNotification(error) => Some(error),
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
    AddClipboardFormatListener(io::Error),
    OpenClipboard(io::Error),
    GetClipboardData(io::Error),
    LockClipboardData(io::Error),
    ClassNameTooLong(usize),
    ThreadPanicked,
    ThreadExited,
}

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WindowError::Init(..) => write!(f, "Failed to initialize window"),
            WindowError::AddClipboardFormatListener(..) => {
                write!(f, "Failed to add clipboard format listener")
            }
            WindowError::OpenClipboard(..) => write!(f, "Failed to open clipboard"),
            WindowError::GetClipboardData(..) => write!(f, "Failed to get clipboard data"),
            WindowError::LockClipboardData(..) => write!(f, "Failed to lock clipboard data"),
            WindowError::ClassNameTooLong(len) => write!(
                f,
                "Class name of length {len} is longer than maximum of 256 bytes"
            ),
            WindowError::ThreadPanicked => write!(f, "Window thread panicked"),
            WindowError::ThreadExited => write!(f, "Window thread unexpectedly exited"),
        }
    }
}

impl std::error::Error for WindowError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WindowError::Init(error) => Some(error),
            WindowError::AddClipboardFormatListener(error) => Some(error),
            WindowError::OpenClipboard(error) => Some(error),
            WindowError::GetClipboardData(error) => Some(error),
            WindowError::LockClipboardData(error) => Some(error),
            WindowError::ClassNameTooLong(..) => None,
            WindowError::ThreadPanicked => None,
            WindowError::ThreadExited => None,
        }
    }
}

#[derive(Debug)]
pub(super) enum ErrorKind {
    WindowSetup(WindowError),
    ThreadError(WindowError),
    ClipboardPoll(WindowError),
    DeleteRegistryKey(io::Error),
    GetRegistryValue(io::Error),
    SetRegistryKey(io::Error),
    CurrentExecutable(io::Error),
    BuildPopupMenu(io::Error),
    SetupIcons(SetupIconsError),
    SetupMenu(SetupMenuError),
    AddNotification(io::Error),
    ModifyNotification(io::Error),
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
pub(super) enum SetupIconsError {
    BuildIcon(io::Error),
}

impl fmt::Display for SetupIconsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BuildIcon(..) => write!(f, "Failed to construct icon"),
        }
    }
}

impl std::error::Error for SetupIconsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BuildIcon(error) => Some(error),
        }
    }
}

#[derive(Debug)]
pub(super) enum SetupMenuError {
    AddMenuEntry(usize, io::Error),
    AddMenuSeparator(usize, io::Error),
}

impl fmt::Display for SetupMenuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddMenuEntry(index, ..) => {
                write!(f, "Failed to add menu entry {index}")
            }
            Self::AddMenuSeparator(index, ..) => {
                write!(f, "Failed to add menu separator {index}")
            }
        }
    }
}

impl std::error::Error for SetupMenuError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AddMenuEntry(_, error) => Some(error),
            Self::AddMenuSeparator(_, error) => Some(error),
        }
    }
}
