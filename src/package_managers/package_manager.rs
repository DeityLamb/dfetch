use std::io;

pub trait PackageManager {
    fn count_packages(&self) -> io::Result<u64>;
    fn is_available(&self) -> bool;
    fn is_relative_distro_id(&self, id: &str) -> bool;
}
