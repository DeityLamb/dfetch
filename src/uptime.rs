use core::fmt;
use std::{
    fmt::Display,
    fs::{self},
    io,
};

static PATH: &str = "/proc/uptime";

#[derive(Default)]
pub struct Uptime(u64);

impl Display for Uptime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let seconds = self.0;
        let days = seconds / (60 * 60 * 24);
        let hours = seconds / (60 * 60) % 24;
        let minutes = (seconds / 60) % 60;

        if days != 0 {
            return write!(f, "{}d, {}h", days, hours);
        }

        if hours != 0 {
            return write!(f, "{}h, {:0<2}m", hours, minutes);
        }

        write!(f, "{:0<2}m", minutes)
    }
}

pub fn get() -> io::Result<Uptime> {
    let uptime = fs::read_to_string(PATH)?
        .split_once(' ')
        .map(|(uptime, _)| uptime.trim().parse::<f64>().unwrap_or(0.0))
        .unwrap_or(0.0);

    Ok(Uptime(uptime as u64))
}
