use sysinfo::System;

/// Returns CPU information.
pub fn get_cpu_info() -> Vec<(String, String)> {
    let mut sys = System::new();
    sys.refresh_cpu();
    let cpus = sys.cpus();

    if let Some(cpu) = cpus.get(0) {
        let brand = cpu.brand();
        let frequency = cpu.frequency();
        let physical_cores = sys.physical_core_count().unwrap_or(0);
        let logical_cores = cpus.len();

        vec![
            ("CPU".to_string(), format!("{} @ {} MHz", brand, frequency)),
            (
                "Cores".to_string(),
                format!("{} ({})", logical_cores, physical_cores),
            ),
        ]
    } else {
        vec![("CPU".to_string(), "Information not available".to_string())]
    }
}
