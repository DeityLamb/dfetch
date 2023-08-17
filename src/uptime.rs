use core::fmt;
use std::{
    fmt::Display,
    fs::{self, File},
    io,
    io::BufRead,
    time::Duration,
};

#[derive(Default)]
pub struct Uptime {
    uptime: Duration,
    idle: Duration,
}

impl Uptime {
    pub fn new() -> io::Result<Uptime> {
        const PATH: &str = "/proc/uptime";

        let Some((uptime, idle)) = fs::read_to_string(PATH)?
        .split_once(" ")
        .map(|(idle, uptime)| (
          idle.trim().parse::<f32>().unwrap_or(0.0),
          uptime.trim().parse::<f32>().unwrap_or(0.0)
        )) else {
          panic!("Failed to parse {} file", PATH);
        };

        Ok(Uptime {
            uptime: Duration::from_secs_f32(uptime),
            idle: Duration::from_secs_f32(idle / get_cpu_count()? as f32),
        })
    }
}

impl Display for Uptime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let seconds = &self.uptime.as_secs() - &self.idle.as_secs();
        let days = seconds / (60 * 60 * 24);
        let hours = seconds / (60 * 60) % 60 % 24;
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

fn get_cpu_count() -> io::Result<u64> {
    Ok(io::BufReader::new(File::open("/proc/cpuinfo")?)
        .lines()
        .flatten()
        .find(|v| v.starts_with("siblings"))
        .map(|v| {
            v.split_once(":")
                .unwrap_or(("_", "0"))
                .1
                .trim()
                .parse::<u64>()
                .unwrap_or(0)
        })
        .unwrap())
}
