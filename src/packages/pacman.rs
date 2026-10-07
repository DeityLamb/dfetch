use std::{fs, io, os::unix::ffi::OsStrExt};

pub fn count() -> io::Result<u64> {
    Ok(fs::read_dir("/var/lib/pacman/local")?
        .flatten()
        .filter(|entry| {
            entry
                .file_name()
                .as_bytes()
                .iter()
                .all(u8::is_ascii_lowercase)
        })
        .count() as u64)
}
