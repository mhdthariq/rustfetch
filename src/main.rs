mod modules;

fn main() {
    println!("--- RustFetch ---");

    // Call the function from module
    modules::os::print_os_info();
    modules::cpu::print_cpu_info();
    modules::memory::print_memory_info();

    println!("-----------------");
}
