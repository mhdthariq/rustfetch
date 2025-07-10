mod modules;
use colored::*;

fn main() {
    println!("--- RustFetch ---");

    // A vector to hold all our fetched information as (label, value) tuples
    let mut info: Vec<(String, String)> = Vec::new();

    // Gather data from all modules
    info.extend(modules::os::get_os_info());
    info.extend(modules::cpu::get_cpu_info());
    info.extend(modules::memory::get_memory_info());
    info.extend(modules::disk::get_disk_info());

    // Find the length of the longest label to use for alignment
    let max_label_len = info.iter().map(|(label, _)| label.len()).max().unwrap_or(0);

    // Iterate and print the formatted information
    for (label, value) in info {
        // Use the `colored` crate to make the label cyan and bold
        let colored_label = label.cyan().bold();

        // **CHANGES HERE**:
        // 1. We use `{:<width$}` for left-alignment.
        // 2. We add a space after the colon for better readability.
        println!(
            "{:<width$}: {}",
            colored_label,
            value,
            width = max_label_len
        );
    }

    println!("-----------------");
}
