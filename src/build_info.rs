#![allow(unreachable_code)]
use chrono::{DateTime, SecondsFormat, Utc};
use std::env;
fn get_version() -> String {
    return env::var("CARGO_PKG_VERSION").unwrap().to_string();
}

fn get_build_time() -> String {
    let now: DateTime<Utc> = Utc::now();
    let iso_string_secs = now.to_rfc3339_opts(SecondsFormat::Secs, true);
    return iso_string_secs;
}

fn get_build_target_arch() -> String {
    #[cfg(target_arch = "aarch64")]
    return "aarch64".to_string();

    #[cfg(target_arch = "x86_64")]
    return "x86_64".to_string();

    #[cfg(target_arch = "loongarch64")]
    return "loongarch64".to_string();

    return "unknown".to_string();
}

fn get_build_target_os() -> String {
    #[cfg(target_os = "linux")]
    return "linux".to_string();

    #[cfg(target_os = "windows")]
    return "windows".to_string();

    #[cfg(target_os = "macos")]
    return "darwin".to_string();

    return "unknown".to_string();
}
fn get_build_target_vendor() -> String {
    #[cfg(target_vendor = "apple")]
    return "apple".to_string();

    #[cfg(target_vendor = "unknown")]
    return "unknown".to_string();

    return "pc".to_string();
}

pub fn show_version_info() {
    println!(
        "Days-Counter \nversion: {} @ {}-{}-{} ",
        get_version(),
        get_build_target_arch(),
        get_build_target_vendor(),
        get_build_target_os()
    );
    println!("build time: {}", get_build_time());
}
