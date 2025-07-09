use sysinfo::System;

// Helper function to convert bytes to megabytes (MiB).
fn bytes_to_mib(bytes: u64) -> u64 {
    bytes / (1024 * 1024)
}

// Fetches and prints system memory and swap information.
pub fn print_memory_info() {
    let mut sys = System::new();
    // Refresh memory-specific information.
    sys.refresh_memory();

    // Get total and used memory in bytes.
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();

    // Get total and used swap in bytes.
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    // Print the formatted information in MiB.
    println!(
        "Memory: {} MiB / {} MiB",
        bytes_to_mib(used_mem),
        bytes_to_mib(total_mem)
    );
    println!(
        "Swap: {} MiB / {} MiB",
        bytes_to_mib(used_swap),
        bytes_to_mib(total_swap)
    );
}
