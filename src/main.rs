mod host;
mod memory;
mod package_managers;
mod uptime;
use crate::{host::get_host, memory::Memory, package_managers::PackageManagers, uptime::Uptime};
use colored::*;

#[cfg(not(unix))]
compile_error!("Sorry, this app works only with unix systems");

fn main() {
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
        pkgs = PackageManagers::new()
            .count_pkgs_by_distro()
            .expect("Failed to get package manager for this distro !")
            .expect("Failed to count packages in this distro !"),
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
