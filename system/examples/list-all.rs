fn main() {
    println!("     --- System Information ---");
    println!("CPU Architecture: {}", alexandria_system::CPU_ARCHITECTURE);
    println!("       CPU Model: {}", alexandria_system::cpu_model());
    println!("       CPU Cores: {}", alexandria_system::cpu_cores());
    println!(
        "Installed Memory: {}",
        alexandria_system::installed_memory()
    );

    println!("       OS Family: {}", alexandria_system::OS_FAMILY);
    println!("         OS Name: {}", alexandria_system::os_name());
    println!("      OS Version: {}", alexandria_system::os_version());
    println!(
        "        Hostname: {}",
        alexandria_system::hostname().unwrap()
    );
}
