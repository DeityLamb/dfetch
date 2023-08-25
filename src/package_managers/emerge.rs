use std::{fs, io};

use super::package_manager::PackageManager;

const EMERGE_DIR: &str = "/var/db/pkg";

#[derive(Clone, Debug)]
pub struct EmergePackageManager {}
impl EmergePackageManager {
    const RELATIVE_DISTRO_IDS: [&str; 1] = ["gentoo"];
}
impl PackageManager for EmergePackageManager {
    fn is_relative_distro_id(&self, id: &str) -> bool {
        Self::RELATIVE_DISTRO_IDS.contains(&id)
    }

    fn count_packages(&self) -> io::Result<u64> {
        Ok(fs::read_dir(EMERGE_DIR)?
            .flatten()
            .filter(|entry| entry.metadata().is_ok_and(|v| v.is_dir()))
            .flat_map(|v| fs::read_dir(v.path()).map(|v| v.count()))
            .fold(0, |acc, count| acc + count as u64)
    )
    }

    fn is_available(&self) -> bool {
        fs::metadata(EMERGE_DIR).is_ok_and(|v| v.is_dir())
    }
}
