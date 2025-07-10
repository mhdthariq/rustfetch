use sysinfo::System;

fn bytes_to_mib(bytes: u64) -> u64 {
    bytes / (1024 * 1024)
}

/// Returns Memory and Swap information.
pub fn get_memory_info() -> Vec<(String, String)> {
    let mut sys = System::new();
    sys.refresh_memory();

    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    vec![
        (
            "Memory".to_string(),
            format!(
                "{} MiB / {} MiB",
                bytes_to_mib(used_mem),
                bytes_to_mib(total_mem)
            ),
        ),
        (
            "Swap".to_string(),
            format!(
                "{} MiB / {} MiB",
                bytes_to_mib(used_swap),
                bytes_to_mib(total_swap)
            ),
        ),
    ]
}
