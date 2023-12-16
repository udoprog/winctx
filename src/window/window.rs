use std::ffi::OsStr;
use std::fmt;
use std::io;
use std::ptr;

use windows_sys::Win32::Foundation::GetLastError;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::System::DataExchange::COPYDATASTRUCT;
use windows_sys::Win32::UI::WindowsAndMessaging::FindWindowExW;
use windows_sys::Win32::UI::WindowsAndMessaging::SendMessageW;
use windows_sys::Win32::UI::WindowsAndMessaging::WM_COPYDATA;

use crate::convert::ToWide;

/// Helper to find windows by title or class.
#[derive(Default)]
pub struct FindWindow {
    class: Option<Vec<u16>>,
    title: Option<Vec<u16>>,
}

impl FindWindow {
    /// Creates a blank find window query.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::window::FindWindow;
    ///
    /// let mut options = FindWindow::new().class("se.tedro.Example");
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Find window by the specified class name.
    ///
    /// The `class` argument matches what has been provided to
    /// [`ContextBuilder::with_class_name`].
    ///
    /// [`ContextBuilder::with_class_name`]: crate::ContextBuilder::with_class_name
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::window::FindWindow;
    ///
    /// let window = FindWindow::new().class("se.tedro.Example").find()?;
    /// # Ok::<_, std::io::Error>(())
    /// ```
    pub fn class<C>(&mut self, class: C) -> &mut Self
    where
        C: AsRef<OsStr>,
    {
        self.class = Some(class.as_ref().to_wide_null());
        self
    }

    /// Find window by the specified class name.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::window::FindWindow;
    ///
    /// let window = FindWindow::new().title("Example Application").find()?;
    /// # Ok::<_, std::io::Error>(())
    /// ```
    pub fn title<T>(&mut self, title: T) -> &mut Self
    where
        T: AsRef<OsStr>,
    {
        self.title = Some(title.as_ref().to_wide_null());
        self
    }

    /// Construct an iterator over all matching windows.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::window::FindWindow;
    ///
    /// let window = FindWindow::new().class("se.tedro.Example").find()?;
    /// # Ok::<_, std::io::Error>(())
    /// ```
    pub fn find(&self) -> io::Result<Option<Window>> {
        // SAFETY: All arguments are correctly popuplated by this builder.
        unsafe {
            let hwnd = FindWindowExW(
                0,
                0,
                self.class.as_ref().map_or(ptr::null(), |c| c.as_ptr()),
                self.title.as_ref().map_or(ptr::null(), |c| c.as_ptr()),
            );

            if hwnd == 0 {
                let code = GetLastError();

                if code == 0 {
                    return Ok(None);
                }

                return Err(io::Error::from_raw_os_error(code as i32));
            }

            Ok(Some(Window { hwnd }))
        }
    }
}

/// Handle to a window on the system.
pub struct Window {
    hwnd: HWND,
}

impl Window {
    /// Copy bytes to the given process.
    ///
    /// Data is received as an [`Event::CopyData`] event.
    ///
    /// [`Event::CopyData`]: crate::Event::CopyData
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use winctx::window::FindWindow;
    ///
    /// let Some(window) = FindWindow::new().class("se.tedro.Example").find()? else {
    ///     println!("Could not find window");
    ///     return Ok(());
    /// };
    ///
    /// window.copy_data(42, b"foobar")?;
    /// # Ok::<_, std::io::Error>(())
    /// ```
    pub fn copy_data(&self, ty: usize, bytes: &[u8]) -> io::Result<()> {
        // SAFETY: All arguments are correctly popuplated by this builder.
        unsafe {
            let data = COPYDATASTRUCT {
                dwData: ty,
                cbData: bytes.len() as u32,
                lpData: (bytes.as_ptr() as *mut u8).cast(),
            };

            SendMessageW(self.hwnd, WM_COPYDATA, 0, &data as *const _ as isize);
            Ok(())
        }
    }
}

impl fmt::Debug for Window {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Window").field("hwnd", &self.hwnd).finish()
    }
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}
