use std::os::unix::ffi::OsStrExt;
use std::{fs, io};

use super::package_manager::PackageManager;

const PACMAN_DIR: &str = "/var/lib/pacman/local";

#[derive(Clone, Debug)]
pub struct PacmanPackageManager {}
impl PacmanPackageManager {
    const RELATIVE_DISTRO_IDS: [&str; 2] = ["arch", "artix"];
}
impl PackageManager for PacmanPackageManager {
    fn is_relative_distro_id(&self, id: &str) -> bool {
        Self::RELATIVE_DISTRO_IDS.contains(&id)
    }

    fn count_packages(&self) -> io::Result<u64> {
        Ok(fs::read_dir(PACMAN_DIR)?
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

    fn is_available(&self) -> bool {
        fs::metadata(PACMAN_DIR).is_ok_and(|v| v.is_dir())
    }
}
