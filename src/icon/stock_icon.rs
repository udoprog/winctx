use windows_sys::Win32::UI::Shell;

/// A stock icon.
#[derive(Debug)]
#[repr(transparent)]
pub struct StockIcon(i32);

impl StockIcon {
    /// Document of a type with no associated application.
    pub const DOCNOASSOC: Self = Self(Shell::SIID_DOCNOASSOC);

    /// Document of a type with an associated application.
    pub const DOCASSOC: Self = Self(Shell::SIID_DOCASSOC);

    /// Generic application with no custom icon.
    pub const APPLICATION: Self = Self(Shell::SIID_APPLICATION);

    /// Folder (generic, unspecified state).
    pub const FOLDER: Self = Self(Shell::SIID_FOLDER);

    /// Folder (open).
    pub const FOLDEROPEN: Self = Self(Shell::SIID_FOLDEROPEN);

    /// 5.25-inch disk drive.
    pub const DRIVE525: Self = Self(Shell::SIID_DRIVE525);

    /// 3.5-inch disk drive.
    pub const DRIVE35: Self = Self(Shell::SIID_DRIVE35);

    /// Removable drive.
    pub const DRIVEREMOVE: Self = Self(Shell::SIID_DRIVEREMOVE);

    /// Fixed drive (hard disk).
    pub const DRIVEFIXED: Self = Self(Shell::SIID_DRIVEFIXED);

    /// Network drive (connected).
    pub const DRIVENET: Self = Self(Shell::SIID_DRIVENET);

    /// Network drive (disconnected).
    pub const DRIVENETDISABLED: Self = Self(Shell::SIID_DRIVENETDISABLED);

    /// CD drive.
    pub const DRIVECD: Self = Self(Shell::SIID_DRIVECD);

    /// RAM disk drive.
    pub const DRIVERAM: Self = Self(Shell::SIID_DRIVERAM);

    /// The entire network.
    pub const WORLD: Self = Self(Shell::SIID_WORLD);

    /// A computer on the network.
    pub const SERVER: Self = Self(Shell::SIID_SERVER);

    /// A local printer or print destination.
    pub const PRINTER: Self = Self(Shell::SIID_PRINTER);

    /// The Network virtual folder (FOLDERID_NetworkFolder/CSIDL_NETWORK).
    pub const MYNETWORK: Self = Self(Shell::SIID_MYNETWORK);

    /// The Search feature.
    pub const FIND: Self = Self(Shell::SIID_FIND);

    /// The Help and Support feature.
    pub const HELP: Self = Self(Shell::SIID_HELP);

    /// Overlay for a shared item.
    pub const SHARE: Self = Self(Shell::SIID_SHARE);

    /// Overlay for a shortcut.
    pub const LINK: Self = Self(Shell::SIID_LINK);

    /// Overlay for items that are expected to be slow to access.
    pub const SLOWFILE: Self = Self(Shell::SIID_SLOWFILE);

    /// The Recycle Bin (empty).
    pub const RECYCLER: Self = Self(Shell::SIID_RECYCLER);

    /// The Recycle Bin (not empty).
    pub const RECYCLERFULL: Self = Self(Shell::SIID_RECYCLERFULL);

    /// Audio CD media.
    pub const MEDIACDAUDIO: Self = Self(Shell::SIID_MEDIACDAUDIO);

    /// Security lock.
    pub const LOCK: Self = Self(Shell::SIID_LOCK);

    /// A virtual folder that contains the results of a search.
    pub const AUTOLIST: Self = Self(Shell::SIID_AUTOLIST);

    /// A network printer.
    pub const PRINTERNET: Self = Self(Shell::SIID_PRINTERNET);

    /// A server shared on a network.
    pub const SERVERSHARE: Self = Self(Shell::SIID_SERVERSHARE);

    /// A local fax printer.
    pub const PRINTERFAX: Self = Self(Shell::SIID_PRINTERFAX);

    /// A network fax printer.
    pub const PRINTERFAXNET: Self = Self(Shell::SIID_PRINTERFAXNET);

    /// A file that receives the output of a Print to file operation.
    pub const PRINTERFILE: Self = Self(Shell::SIID_PRINTERFILE);

    /// A category that results from a Stack by command to organize the contents
    /// of a folder.
    pub const STACK: Self = Self(Shell::SIID_STACK);

    /// Super Video CD (SVCD) media.
    pub const MEDIASVCD: Self = Self(Shell::SIID_MEDIASVCD);

    /// A folder that contains only subfolders as child items.
    pub const STUFFEDFOLDER: Self = Self(Shell::SIID_STUFFEDFOLDER);

    /// Unknown drive type.
    pub const DRIVEUNKNOWN: Self = Self(Shell::SIID_DRIVEUNKNOWN);

    /// DVD drive.
    pub const DRIVEDVD: Self = Self(Shell::SIID_DRIVEDVD);

    /// DVD media.
    pub const MEDIADVD: Self = Self(Shell::SIID_MEDIADVD);

    /// DVD-RAM media.
    pub const MEDIADVDRAM: Self = Self(Shell::SIID_MEDIADVDRAM);

    /// DVD-RW media.
    pub const MEDIADVDRW: Self = Self(Shell::SIID_MEDIADVDRW);

    /// DVD-R media.
    pub const MEDIADVDR: Self = Self(Shell::SIID_MEDIADVDR);

    /// DVD-ROM media.
    pub const MEDIADVDROM: Self = Self(Shell::SIID_MEDIADVDROM);

    /// CD+ (enhanced audio CD) media.
    pub const MEDIACDAUDIOPLUS: Self = Self(Shell::SIID_MEDIACDAUDIOPLUS);

    /// CD-RW media.
    pub const MEDIACDRW: Self = Self(Shell::SIID_MEDIACDRW);

    /// CD-R media.
    pub const MEDIACDR: Self = Self(Shell::SIID_MEDIACDR);

    /// A writable CD in the process of being burned.
    pub const MEDIACDBURN: Self = Self(Shell::SIID_MEDIACDBURN);

    /// Blank writable CD media.
    pub const MEDIABLANKCD: Self = Self(Shell::SIID_MEDIABLANKCD);

    /// CD-ROM media.
    pub const MEDIACDROM: Self = Self(Shell::SIID_MEDIACDROM);

    /// An audio file.
    pub const AUDIOFILES: Self = Self(Shell::SIID_AUDIOFILES);

    /// An image file.
    pub const IMAGEFILES: Self = Self(Shell::SIID_IMAGEFILES);

    /// A video file.
    pub const VIDEOFILES: Self = Self(Shell::SIID_VIDEOFILES);

    /// A mixed file.
    pub const MIXEDFILES: Self = Self(Shell::SIID_MIXEDFILES);

    /// Folder back.
    pub const FOLDERBACK: Self = Self(Shell::SIID_FOLDERBACK);

    /// Folder front.
    pub const FOLDERFRONT: Self = Self(Shell::SIID_FOLDERFRONT);

    /// Security shield. Use for UAC prompts only.
    pub const SHIELD: Self = Self(Shell::SIID_SHIELD);

    /// Warning.
    pub const WARNING: Self = Self(Shell::SIID_WARNING);

    /// Informational.
    pub const INFO: Self = Self(Shell::SIID_INFO);

    /// Error.
    pub const ERROR: Self = Self(Shell::SIID_ERROR);

    /// Key.
    pub const KEY: Self = Self(Shell::SIID_KEY);

    /// Software.
    pub const SOFTWARE: Self = Self(Shell::SIID_SOFTWARE);

    /// A UI item, such as a button, that issues a rename command.
    pub const RENAME: Self = Self(Shell::SIID_RENAME);

    /// A UI item, such as a button, that issues a delete command.
    pub const DELETE: Self = Self(Shell::SIID_DELETE);

    /// Audio DVD media.
    pub const MEDIAAUDIODVD: Self = Self(Shell::SIID_MEDIAAUDIODVD);

    /// Movie DVD media.
    pub const MEDIAMOVIEDVD: Self = Self(Shell::SIID_MEDIAMOVIEDVD);

    /// Enhanced CD media.
    pub const MEDIAENHANCEDCD: Self = Self(Shell::SIID_MEDIAENHANCEDCD);

    /// Enhanced DVD media.
    pub const MEDIAENHANCEDDVD: Self = Self(Shell::SIID_MEDIAENHANCEDDVD);

    /// High definition DVD media in the HD DVD format.
    pub const MEDIAHDDVD: Self = Self(Shell::SIID_MEDIAHDDVD);

    /// High definition DVD media in the Blu-ray Disc™ format.
    pub const MEDIABLURAY: Self = Self(Shell::SIID_MEDIABLURAY);

    /// Video CD (VCD) media.
    pub const MEDIAVCD: Self = Self(Shell::SIID_MEDIAVCD);

    /// DVD+R media.
    pub const MEDIADVDPLUSR: Self = Self(Shell::SIID_MEDIADVDPLUSR);

    /// DVD+RW media.
    pub const MEDIADVDPLUSRW: Self = Self(Shell::SIID_MEDIADVDPLUSRW);

    /// A desktop computer.
    pub const DESKTOPPC: Self = Self(Shell::SIID_DESKTOPPC);

    /// A mobile computer (laptop).
    pub const MOBILEPC: Self = Self(Shell::SIID_MOBILEPC);

    /// The User Accounts Control Panel item.
    pub const USERS: Self = Self(Shell::SIID_USERS);

    /// Smart media.
    pub const MEDIASMARTMEDIA: Self = Self(Shell::SIID_MEDIASMARTMEDIA);

    /// CompactFlash media.
    pub const MEDIACOMPACTFLASH: Self = Self(Shell::SIID_MEDIACOMPACTFLASH);

    /// A cell phone.
    pub const DEVICECELLPHONE: Self = Self(Shell::SIID_DEVICECELLPHONE);

    /// A digital camera.
    pub const DEVICECAMERA: Self = Self(Shell::SIID_DEVICECAMERA);

    /// A digital video camera.
    pub const DEVICEVIDEOCAMERA: Self = Self(Shell::SIID_DEVICEVIDEOCAMERA);

    /// An audio player.
    pub const DEVICEAUDIOPLAYER: Self = Self(Shell::SIID_DEVICEAUDIOPLAYER);

    /// Connect to network.
    pub const NETWORKCONNECT: Self = Self(Shell::SIID_NETWORKCONNECT);

    /// The Network and Internet Control Panel item.
    pub const INTERNET: Self = Self(Shell::SIID_INTERNET);

    /// A compressed file with a .zip file name extension.
    pub const ZIPFILE: Self = Self(Shell::SIID_ZIPFILE);

    /// The Additional Options Control Panel item.
    pub const SETTINGS: Self = Self(Shell::SIID_SETTINGS);

    /// Windows Vista with Service Pack 1 (SP1) and later. High definition DVD
    /// drive (any type - HD DVD-ROM, HD DVD-R, HD-DVD-RAM) that uses the HD DVD
    /// format.
    pub const DRIVEHDDVD: Self = Self(Shell::SIID_DRIVEHDDVD);

    /// Windows Vista with SP1 and later. High definition DVD drive (any type -
    /// BD-ROM, BD-R, BD-RE) that uses the Blu-ray Disc format.
    pub const DRIVEBD: Self = Self(Shell::SIID_DRIVEBD);

    /// Windows Vista with SP1 and later. High definition DVD-ROM media in the
    /// HD DVD-ROM format.
    pub const MEDIAHDDVDROM: Self = Self(Shell::SIID_MEDIAHDDVDROM);

    /// Windows Vista with SP1 and later. High definition DVD-R media in the HD
    /// DVD-R format.
    pub const MEDIAHDDVDR: Self = Self(Shell::SIID_MEDIAHDDVDR);

    /// Windows Vista with SP1 and later. High definition DVD-RAM media in the
    /// HD DVD-RAM format.
    pub const MEDIAHDDVDRAM: Self = Self(Shell::SIID_MEDIAHDDVDRAM);

    /// Windows Vista with SP1 and later. High definition DVD-ROM media in the
    /// Blu-ray Disc BD-ROM format.
    pub const MEDIABDROM: Self = Self(Shell::SIID_MEDIABDROM);

    /// Windows Vista with SP1 and later. High definition write-once media in
    /// the Blu-ray Disc BD-R format.
    pub const MEDIABDR: Self = Self(Shell::SIID_MEDIABDR);

    /// Windows Vista with SP1 and later. High definition read/write media in
    /// the Blu-ray Disc BD-RE format.
    pub const MEDIABDRE: Self = Self(Shell::SIID_MEDIABDRE);

    /// Windows Vista with SP1 and later. A cluster disk array.
    pub const CLUSTEREDDRIVE: Self = Self(Shell::SIID_CLUSTEREDDRIVE);

    /// Get the underlying icon identifier.
    pub(crate) fn as_id(&self) -> i32 {
        self.0
    }
}
