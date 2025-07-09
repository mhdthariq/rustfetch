// No need to import SystemExt, and for this module,
// we don't even need a System instance.
use sysinfo::System;

/// Fetches and prints basic operating system information.
pub fn print_os_info() {
    // These methods are associated functions, so we call them directly on the `System` type.
    // They return `Option<String>`, so we handle the case where the value might not be available.
    let os_name = System::name().unwrap_or_else(|| "N/A".to_string());
    let kernel_ver = System::kernel_version().unwrap_or_else(|| "N/A".to_string());
    let os_ver = System::os_version().unwrap_or_else(|| "".to_string());
    let hostname = System::host_name().unwrap_or_else(|| "N/A".to_string());

    // The .trim() on os_ver cleans up any potential extra whitespace.
    println!("OS: {} {} {}", os_name, kernel_ver, os_ver.trim());
    println!("Host: {}", hostname);
}
