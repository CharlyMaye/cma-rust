use std::fs::{File, OpenOptions};
use std::io::Result;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

/// Ouvre un fichier de log en mode append avec partage de lecture cross-platform
pub fn open_log_file(path: &Path) -> Result<File> {
    #[cfg(unix)]
    {
        // Sur Unix, utiliser les permissions standard
        OpenOptions::new()
            .append(true)
            .create(true)
            .mode(0o644) // rw-r--r-- : propriétaire peut lire/écrire, autres peuvent lire
            .open(path)
    }

    #[cfg(windows)]
    {
        const FILE_SHARE_READ: u32 = 0x00000001;
        const FILE_SHARE_WRITE: u32 = 0x00000002;

        // Sur Windows, autoriser explicitement le partage en lecture
        OpenOptions::new()
            .append(true)
            .create(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
            .open(path)
    }

    #[cfg(not(any(unix, windows)))]
    {
        // Fallback pour autres OS
        OpenOptions::new().append(true).create(true).open(path)
    }
}
