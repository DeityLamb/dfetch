use std::io;

use super::read_lines;

pub fn count() -> io::Result<u64> {
    Ok(read_lines("/lib/apk/db/installed")?
        .filter(|line| line.starts_with("P:"))
        .count() as u64)
}
