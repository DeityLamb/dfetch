use std::{fs, io};

pub fn count() -> io::Result<u64> {
    Ok(fs::read_dir("/var/db/pkg")?
        .flatten()
        .filter(|entry| entry.metadata().is_ok_and(|v| v.is_dir()))
        .flat_map(|category| fs::read_dir(category.path()).map(|v| v.count() as u64))
        .sum())
}
