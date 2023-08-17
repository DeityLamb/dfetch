mod host;
mod memory;
mod packages;
mod uptime;
use crate::{host::get_host, memory::Memory, packages::get_packages_count, uptime::Uptime};
use colored::*;
use whoami;

fn main() {
    if !cfg!(unix) {
        panic!("Sorry, this app works only with unix systems")
    }

    println!(
        r#"
  /、          {username}@{hostname}
  (❍ˎ ❍ 7      {os_field:<width$} {os}
  |、 ϛ        {host_field:<width$} {host}
  |    \       {uptime_field:<width$} {uptime}
  |     \      {pkgs_field:<width$} {pkgs}
  じ しˍ,)ノ   {memory_field:<width$} {memory}
"#,
        username = whoami::username().yellow().bold(),
        hostname = whoami::hostname().yellow().bold(),
        os = whoami::distro(),
        host = get_host().unwrap_or("Unknown".to_owned()),
        uptime = Uptime::new().unwrap_or_default(),
        memory = Memory::new().unwrap_or_default(),
        pkgs = get_packages_count().unwrap_or(0),
        os_field = wrap_field("os"),
        host_field = wrap_field("host"),
        memory_field = wrap_field("memory"),
        uptime_field = wrap_field("uptime"),
        pkgs_field = wrap_field("pkgs"),
        width = 6
    );
}

fn wrap_field(field: &str) -> ColoredString {
    field.green().bold()
}
