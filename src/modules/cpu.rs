use sysinfo::System;

/// Fetches and prints CPU information using a more reliable method.
pub fn print_cpu_info() {
    // We'll create a new System instance.
    let mut sys = System::new();

    // Refresh the CPU-specific information.
    sys.refresh_cpu();

    // Get the list of CPUs.
    let cpus = sys.cpus();

    // The brand and frequency should be the same for all cores,
    // so we can safely take it from the first CPU in the list.
    // We add a check to make sure the list is not empty.
    if let Some(cpu) = cpus.get(0) {
        let brand = cpu.brand();
        let frequency = cpu.frequency();

        let physical_cores = sys.physical_core_count().unwrap_or(0);
        let logical_cores = cpus.len();

        println!("CPU: {} @ {} MHz", brand, frequency);
        println!("Cores: {} ({})", logical_cores, physical_cores);
    } else {
        println!("CPU: Information not available.");
    }
}
