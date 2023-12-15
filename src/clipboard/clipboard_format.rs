use std::fmt;

use windows_sys::Win32::System::Ole as ole;

/// A clipboard format.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ClipboardFormat(u16);

#[allow(unused)]
impl ClipboardFormat {
    /// A handle to a bitmap (HBITMAP).
    pub const BITMAP: Self = Self(ole::CF_BITMAP);

    /// A memory object containing a BITMAPINFO structure followed by the bitmap
    /// bits.
    pub const DIB: Self = Self(ole::CF_DIB);

    /// A memory object containing a BITMAPV5HEADER structure followed by the
    /// bitmap color space information and the bitmap bits.
    pub const DIBV5: Self = Self(ole::CF_DIBV5);

    /// Software Arts' Data Interchange Format.
    pub const DIF: Self = Self(ole::CF_DIF);

    /// Bitmap display format associated with a private format. The hMem
    /// parameter must be a handle to data that can be displayed in bitmap
    /// format in lieu of the privately formatted data.
    pub const DSPBITMAP: Self = Self(ole::CF_DSPBITMAP);

    /// Enhanced metafile display format associated with a private format. The
    /// hMem parameter must be a handle to data that can be displayed in
    /// enhanced metafile format in lieu of the privately formatted data.
    pub const DSPENHMETAFILE: Self = Self(ole::CF_DSPENHMETAFILE);

    /// Metafile-picture display format associated with a private format. The
    /// hMem parameter must be a handle to data that can be displayed in
    /// metafile-picture format in lieu of the privately formatted data.
    pub const DSPMETAFILEPICT: Self = Self(ole::CF_DSPMETAFILEPICT);

    /// Text display format associated with a private format. The hMem parameter
    /// must be a handle to data that can be displayed in text format in lieu of
    /// the privately formatted data.
    pub const DSPTEXT: Self = Self(ole::CF_DSPTEXT);

    /// A handle to an enhanced metafile (HENHMETAFILE).
    pub const ENHMETAFILE: Self = Self(ole::CF_ENHMETAFILE);

    /// Start of a range of integer values for application-defined GDI object
    /// clipboard formats. The end of the range is CF_GDIOBJLAST. Handles
    /// associated with clipboard formats in this range are not automatically
    /// deleted using the GlobalFree function when the clipboard is emptied.
    /// Also, when using values in this range, the hMem parameter is not a
    /// handle to a GDI object, but is a handle allocated by the GlobalAlloc
    /// function with the GMEM_MOVEABLE flag.
    pub const GDIOBJFIRST: Self = Self(ole::CF_GDIOBJFIRST);

    /// See CF_GDIOBJFIRST.
    pub const GDIOBJLAST: Self = Self(ole::CF_GDIOBJLAST);

    /// A handle to type HDROP that identifies a list of files. An application
    /// can retrieve information about the files by passing the handle to the
    /// DragQueryFile function.
    pub const HDROP: Self = Self(ole::CF_HDROP);

    /// The data is a handle (HGLOBAL) to the locale identifier (LCID)
    /// associated with text in the clipboard. When you close the clipboard, if
    /// it contains CF_TEXT data but no CF_LOCALE data, the system automatically
    /// sets the CF_LOCALE format to the current input language. You can use the
    /// CF_LOCALE format to associate a different locale with the clipboard
    /// text. An application that pastes text from the clipboard can retrieve
    /// this format to determine which character set was used to generate the
    /// text. Note that the clipboard does not support plain text in multiple
    /// character sets. To achieve this, use a formatted text data type such as
    /// RTF instead. The system uses the code page associated with CF_LOCALE to
    /// implicitly convert from CF_TEXT to CF_UNICODETEXT. Therefore, the
    /// correct code page table is used for the conversion.
    pub const LOCALE: Self = Self(ole::CF_LOCALE);

    /// Handle to a metafile picture format as defined by the METAFILEPICT
    /// structure. When passing a CF_METAFILEPICT handle by means of DDE, the
    /// application responsible for deleting hMem should also free the metafile
    /// referred to by the CF_METAFILEPICT handle.
    pub const METAFILEPICT: Self = Self(ole::CF_METAFILEPICT);

    /// Text format containing characters in the OEM character set. Each line
    /// ends with a carriage return/linefeed (CR-LF) combination. A null
    /// character signals the end of the data.
    pub const OEMTEXT: Self = Self(ole::CF_OEMTEXT);

    /// Owner-display format. The clipboard owner must display and update the
    /// clipboard viewer window, and receive the WM_ASKCBFORMATNAME,
    /// WM_HSCROLLCLIPBOARD, WM_PAINTCLIPBOARD, WM_SIZECLIPBOARD, and
    /// WM_VSCROLLCLIPBOARD messages. The hMem parameter must be NULL.
    pub const OWNERDISPLAY: Self = Self(ole::CF_OWNERDISPLAY);

    /// Handle to a color palette. Whenever an application places data in the
    /// clipboard that depends on or assumes a color palette, it should place
    /// the palette on the clipboard as well. If the clipboard contains data in
    /// the CF_PALETTE (logical color palette) format, the application should
    /// use the SelectPalette and RealizePalette functions to realize (compare)
    /// any other data in the clipboard against that logical palette. When
    /// displaying clipboard data, the clipboard always uses as its current
    /// palette any object on the clipboard that is in the CF_PALETTE format.
    pub const PALETTE: Self = Self(ole::CF_PALETTE);

    /// Data for the pen extensions to the Microsoft Windows for Pen Computing.
    pub const PENDATA: Self = Self(ole::CF_PENDATA);

    /// Start of a range of integer values for private clipboard formats. The
    /// range ends with CF_PRIVATELAST. Handles associated with private
    /// clipboard formats are not freed automatically; the clipboard owner must
    /// free such handles, typically in response to the WM_DESTROYCLIPBOARD
    /// message.
    pub const PRIVATEFIRST: Self = Self(ole::CF_PRIVATEFIRST);

    /// See CF_PRIVATEFIRST.
    pub const PRIVATELAST: Self = Self(ole::CF_PRIVATELAST);

    /// Represents audio data more complex than can be represented in a CF_WAVE
    /// standard wave format.
    pub const RIFF: Self = Self(ole::CF_RIFF);

    /// Microsoft Symbolic Link (SYLK) format.
    pub const SYLK: Self = Self(ole::CF_SYLK);

    /// Text format. Each line ends with a carriage return/linefeed (CR-LF)
    /// combination. A null character signals the end of the data. Use this
    /// format for ANSI text.
    pub const TEXT: Self = Self(ole::CF_TEXT);

    /// Tagged-image file format.
    pub const TIFF: Self = Self(ole::CF_TIFF);

    /// Unicode text format. Each line ends with a carriage return/linefeed
    /// (CR-LF) combination. A null character signals the end of the data.
    pub const UNICODETEXT: Self = Self(ole::CF_UNICODETEXT);

    /// Represents audio data in one of the standard wave formats, such as 11
    /// kHz or 22 kHz PCM.
    pub const WAVE: Self = Self(ole::CF_WAVE);
}

impl ClipboardFormat {
    /// Construct a new clipboard format from the given raw value.
    pub(super) const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Get the raw value of this clipboard format.
    pub(super) const fn as_u16(self) -> u16 {
        self.0
    }
}

impl fmt::Debug for ClipboardFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self(ole::CF_BITMAP) => write!(f, "BITMAP"),
            Self(ole::CF_DIB) => write!(f, "DIB"),
            Self(ole::CF_DIBV5) => write!(f, "DIBV5"),
            Self(ole::CF_DIF) => write!(f, "DIF"),
            Self(ole::CF_DSPBITMAP) => write!(f, "DSPBITMAP"),
            Self(ole::CF_DSPENHMETAFILE) => write!(f, "DSPENHMETAFILE"),
            Self(ole::CF_DSPMETAFILEPICT) => write!(f, "DSPMETAFILEPICT"),
            Self(ole::CF_DSPTEXT) => write!(f, "DSPTEXT"),
            Self(ole::CF_ENHMETAFILE) => write!(f, "ENHMETAFILE"),
            Self(ole::CF_GDIOBJFIRST) => write!(f, "GDIOBJFIRST"),
            Self(ole::CF_GDIOBJLAST) => write!(f, "GDIOBJLAST"),
            Self(ole::CF_HDROP) => write!(f, "HDROP"),
            Self(ole::CF_LOCALE) => write!(f, "LOCALE"),
            Self(ole::CF_METAFILEPICT) => write!(f, "METAFILEPICT"),
            Self(ole::CF_OEMTEXT) => write!(f, "OEMTEXT"),
            Self(ole::CF_OWNERDISPLAY) => write!(f, "OWNERDISPLAY"),
            Self(ole::CF_PALETTE) => write!(f, "PALETTE"),
            Self(ole::CF_PENDATA) => write!(f, "PENDATA"),
            Self(ole::CF_PRIVATEFIRST) => write!(f, "PRIVATEFIRST"),
            Self(ole::CF_PRIVATELAST) => write!(f, "PRIVATELAST"),
            Self(ole::CF_RIFF) => write!(f, "RIFF"),
            Self(ole::CF_SYLK) => write!(f, "SYLK"),
            Self(ole::CF_TEXT) => write!(f, "TEXT"),
            Self(ole::CF_TIFF) => write!(f, "TIFF"),
            Self(ole::CF_UNICODETEXT) => write!(f, "UNICODETEXT"),
            Self(ole::CF_WAVE) => write!(f, "WAVE"),
            Self(format) => write!(f, "UNKNOWN({})", format),
        }
    }
}
