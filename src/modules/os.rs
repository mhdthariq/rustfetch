use sysinfo::System;

/// Returns OS and Host information as a vector of (label, value) tuples.
pub fn get_os_info() -> Vec<(String, String)> {
    let os_name = System::name().unwrap_or_else(|| "N/A".to_string());
    let kernel_ver = System::kernel_version().unwrap_or_else(|| "N/A".to_string());
    let os_ver = System::os_version().unwrap_or_else(|| "".to_string());
    let hostname = System::host_name().unwrap_or_else(|| "N/A".to_string());

    vec![
        (
            "OS".to_string(),
            format!("{} {} {}", os_name, kernel_ver, os_ver.trim()),
        ),
        ("Host".to_string(), hostname),
    ]
}
