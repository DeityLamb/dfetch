use core::fmt;
use std::{collections::HashMap, fmt::Display, fs, io};

// (used, total)
#[derive(Default)]
pub struct Memory(u64, u64);

static PATH: &str = "/proc/meminfo";
static MB: u64 = 1024 * 1024;

impl Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}M / {}M", self.0 / MB, self.1 / MB)
    }
}

pub fn get() -> io::Result<Memory> {
    let meminfo = fs::read_to_string(PATH)?;
    let fields = meminfo
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(':').unwrap_or(("_", "0"));
            let (memory, unit) = value.trim().split_once(' ').unwrap_or((value, ""));

            let mem: u64 = memory
                .trim()
                .parse()
                .expect("Failed to parse a memory value");
            (key, to_bites(mem, unit))
        })
        .collect::<HashMap<&str, u64>>();

    let total = *fields.get("MemTotal").unwrap_or(&0);

    let available = fields.get("MemAvailable").copied().unwrap_or_else(|| {
        // Linux < 3.14 may not have the MemAvailable field in /proc/meminfo
        // https://github.com/KittyKatt/screenFetch/issues/386#issuecomment-249312716

        fields
            .get("MemFree")
            .unwrap_or(&0)
            .saturating_add(*fields.get("SReclaimable").unwrap_or(&0))
            .saturating_add(*fields.get("Buffers").unwrap_or(&0))
            .saturating_add(*fields.get("Cached").unwrap_or(&0))
            .saturating_sub(*fields.get("Shmem").unwrap_or(&0))
    });

    Ok(Memory(total - available, total))
}

fn to_bites(value: u64, unit: &str) -> u64 {
    match unit {
        "kB" => 1024 * value,
        "" => 1 * value,
        _ => todo!(),
    }
}
