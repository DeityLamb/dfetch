use std::{fs, io};

pub fn get_host() -> io::Result<String> {
    Ok(
        fs::read_to_string("/sys/devices/virtual/dmi/id/product_version")?
            .trim()
            .to_owned(),
    )
}
