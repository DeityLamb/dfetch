use core::fmt;
use std::{collections::HashMap, fmt::Display, fs, io};

#[derive(Default)]
pub struct Memory {
    pub total: u64,
    pub used: u64,
}

impl Memory {
    pub fn new() -> io::Result<Memory> {
        let fields = fs::read_to_string("/proc/meminfo")?
            .lines()
            .map(|line| {
                let (key, value) = line.split_once(':').unwrap_or(("_", "0"));
                let (memory, unit) = value.trim().split_once(' ').unwrap_or((value, ""));

                let mem: u64 = memory
                    .trim()
                    .parse()
                    .expect("Failed to parse a memory value");
                (key.to_owned(), Self::to_bites(mem, unit))
            })
            .collect::<HashMap<String, u64>>();

        let total = *fields.get("MemTotal").unwrap_or(&0);

        let available = if let Some(available) = fields.get("MemAvailable") {
            *available
        } else {
            // Linux < 3.14 may not have MemAvailable in /proc/meminfo
            // https://github.com/KittyKatt/screenFetch/issues/386#issuecomment-249312716

            fields
                .get("MemFree")
                .unwrap_or(&0)
                .saturating_add(*fields.get("SReclaimable").unwrap_or(&0))
                .saturating_add(*fields.get("Buffers").unwrap_or(&0))
                .saturating_add(*fields.get("Cached").unwrap_or(&0))
                .saturating_sub(*fields.get("Shmem").unwrap_or(&0))
        };

        Ok(Self {
            total,
            used: total - available,
        })
    }

    fn to_bites(value: u64, unit: &str) -> u64 {
        value
            * match unit {
                "kB" => 1024,
                "" => 1,
                _ => todo!(),
            }
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}M / {}M",
            self.used / 1024 / 1024,
            self.total / 1024 / 1024
        )
    }
}
