mod host;
mod memory;
mod packages;
mod uptime;
use crate::host::get_host;
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
        hostname = whoami::fallible::hostname()
            .unwrap_or_else(|_| "unknown".to_owned())
            .yellow()
            .bold(),
        os = whoami::distro(),
        host = get_host().unwrap_or("Unknown".to_owned()),
        uptime = uptime::get().unwrap_or_default(),
        memory = memory::get().unwrap_or_default(),
        pkgs = packages::get().unwrap_or("Unknown".to_owned()),
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
