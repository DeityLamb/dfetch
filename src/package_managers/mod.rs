mod package_manager;
mod pacman;
mod xbps;
use std::vec;
use std::{
    fs::File,
    io::{self, BufRead},
};

use package_manager::PackageManager;

use self::pacman::PacmanPackageManager;
use self::xbps::XBPSPackageManager;

const OS_RELEASE_FILE: &str = "/etc/os-release";

const XBPS: XBPSPackageManager = XBPSPackageManager {};
const PACMAN: PacmanPackageManager = PacmanPackageManager {};

pub struct PackageManagers {
    managers: Vec<&'static dyn PackageManager>,
}
impl PackageManagers {
    pub fn new() -> Self {
        Self {
            managers: vec![&XBPS, &PACMAN],
        }
    }

    pub fn count_pkgs_by_distro(&self) -> Option<io::Result<u64>> {
        io::BufReader::new(File::open(OS_RELEASE_FILE).expect("Failed to open os-release file !"))
            .lines()
            .flatten()
            .filter(|line| line.starts_with("ID") || line.starts_with("ID_LIKE"))
            .find_map(|line| {
                let distro_id = line.split_once('=').map(|(_, id)| id.trim_matches('"'))?;

                self.managers
                    .iter()
                    .find(|manager| manager.is_relative_distro_id(distro_id))
                    .and_then(|v| v.is_available().then_some(*v))
            })
            .map(|v| v.count_packages())
    }

    #[allow(dead_code)]
    pub fn count_pkgs_by_available_manager(&self) -> Option<io::Result<u64>> {
        self.managers
            .iter()
            .find(|v| v.is_available())
            .map(|v| v.count_packages())
    }
}
