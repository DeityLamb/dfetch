use std::{fs, io};

// In this case, "host" refers to the version of the device on which Linux is running.
// For example, a "ThinkPad E14 Gen 3".

pub fn get_host() -> io::Result<String> {
    Ok(
        fs::read_to_string("/sys/devices/virtual/dmi/id/product_family")?
            .trim()
            .to_owned(),
    )
}
