use std::{fs, io};

// Every installed package is a "<name>-<version>-<rel>" directory;
// the only other entry is the ALPM_DB_VERSION file.
pub fn count() -> io::Result<u64> {
    Ok(fs::read_dir("/var/lib/pacman/local")?
        .flatten()
        .filter(|entry| entry.file_type().is_ok_and(|v| v.is_dir()))
        .count() as u64)
}
