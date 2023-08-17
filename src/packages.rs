use std::{
    fs::{self, File},
    io::{self, BufRead},
};

const XBPS_DB_DIR: &str = "/var/db/xbps";

pub fn get_packages_count() -> io::Result<u64> {
    match whoami::distro().as_str() {
        "Void Linux" => get_void_packages_count(),
        _ => unimplemented!("Right now, focus on implementing it specifically for Void Linux."),
    }
}

fn get_void_packages_count() -> io::Result<u64> {
    let files = fs::read_dir(XBPS_DB_DIR)?;

    let Some(file) = files
        .flatten()
        .find(|file| file.file_name().to_str().is_some_and(|f| f.starts_with("pkgdb-")))
    else {
        panic!("Failed to locate the pkgdb file in {} directory !", XBPS_DB_DIR);
    };

    let file = File::open(file.path())?;

    Ok(io::BufReader::new(file)
        .lines()
        .flatten()
        .filter(|line| line.contains("<key>repository</key>"))
        .count() as u64)
}
