use sysinfo::Disks;

fn bytes_to_gib(bytes: u64) -> f32 {
    bytes as f32 / (1024.0 * 1024.0 * 1024.0)
}

/// Returns disk usage information.
pub fn get_disk_info() -> Vec<(String, String)> {
    let disks = Disks::new_with_refreshed_list();
    let mut disk_info = Vec::new();

    // The first disk gets the "Disks" label
    let mut first = true;
    for disk in &disks {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        let used_space = total_space - available_space;

        let label = if first {
            "Disks".to_string()
        } else {
            "".to_string()
        };
        first = false;

        disk_info.push((
            label,
            format!(
                "- {}: {:.2} GiB / {:.2} GiB",
                disk.mount_point().display(),
                bytes_to_gib(used_space),
                bytes_to_gib(total_space)
            ),
        ));
    }
    disk_info
}
