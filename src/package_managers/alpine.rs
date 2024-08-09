use std::{
    fs::{self, File},
    io::{self, BufRead},
};

use super::package_manager::PackageManager;

const ALPINE_INSTALLED: &str = "/lib/apk/db/installed";

#[derive(Clone, Debug)]
pub struct AlpinePackageManager {}
impl AlpinePackageManager {
    const RELATIVE_DISTRO_IDS: [&str; 1] = ["alpine"];
}
impl PackageManager for AlpinePackageManager {
    fn is_relative_distro_id(&self, id: &str) -> bool {
        Self::RELATIVE_DISTRO_IDS.contains(&id)
    }

    fn count_packages(&self) -> io::Result<u64> {
        Ok(io::BufReader::new(File::open(ALPINE_INSTALLED)?)
            .lines()
            .map_while(Result::ok)
            .filter(|line| line.starts_with("P:"))
            .count() as u64)
    }

    fn is_available(&self) -> bool {
        fs::metadata(ALPINE_INSTALLED).is_ok_and(|v| v.is_file())
    }
}
