mod apk;
mod emerge;
mod pacman;
mod xbps;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

type Count = fn() -> io::Result<u64>;

const MANAGERS: [(&str, Count); 4] = [
    ("xbps", xbps::count),
    ("apk", apk::count),
    ("pacman", pacman::count),
    ("emerge", emerge::count),
];

// Counts installed packages of every package manager found on the system.
// A single manager is shown as just the count, several as "xbps(632), apk(251)".
pub fn get() -> io::Result<String> {
    let counts: Vec<_> = MANAGERS
        .iter()
        .filter_map(|(name, count)| count().ok().map(|count| (name, count)))
        .collect();

    match counts.as_slice() {
        [] => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "no supported package manager found",
        )),
        [(_, count)] => Ok(count.to_string()),
        _ => Ok(counts
            .iter()
            .map(|(name, count)| format!("{name}({count})"))
            .collect::<Vec<_>>()
            .join(", ")),
    }
}

fn read_lines(path: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    Ok(BufReader::new(File::open(path)?)
        .lines()
        .map_while(Result::ok))
}
