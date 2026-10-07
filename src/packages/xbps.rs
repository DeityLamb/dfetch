use std::{fs, io, os::unix::ffi::OsStrExt};

use super::read_lines;

pub fn count() -> io::Result<u64> {
    let pkgdb = fs::read_dir("/var/db/xbps")?
        .flatten()
        .find(|entry| entry.file_name().as_bytes().starts_with(b"pkgdb-"))
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "xbps pkgdb not found"))?;

    Ok(read_lines(pkgdb.path())?
        .filter(|line| line.contains("<key>repository</key>"))
        .count() as u64)
}
