use std::{
    fs::{self, File},
    io::{self, BufRead},
};

use super::package_manager::PackageManager;

const XBPS_DIR: &str = "/var/db/xbps";

#[derive(Clone, Debug)]
pub struct XBPSPackageManager {}
impl XBPSPackageManager {
    const RELATIVE_DISTRO_IDS: [&str; 1] = ["void"];
}
impl PackageManager for XBPSPackageManager {
    fn is_relative_distro_id(&self, id: &str) -> bool {
        Self::RELATIVE_DISTRO_IDS.contains(&id)
    }

    fn count_packages(&self) -> io::Result<u64> {
        let file = fs::read_dir(XBPS_DIR)?
            .flatten()
            .find(|file| {
                file.file_name()
                    .to_str()
                    .is_some_and(|f| f.starts_with("pkgdb-"))
            })
            .expect("Failed to locate the pkgdb file !");

        Ok(io::BufReader::new(File::open(file.path())?)
            .lines()
            .flatten()
            .filter(|line| line.contains("<key>repository</key>"))
            .count() as u64)
    }

    fn is_available(&self) -> bool {
        fs::metadata(XBPS_DIR).is_ok_and(|v| v.is_dir())
    }
}
