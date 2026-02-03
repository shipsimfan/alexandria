fn main() {
    println!("     --- System Information ---");
    println!(
        "CPU Architecture: {}",
        alexandria::system::Architecture::CURRENT
    );
    println!("       CPU Model: {}", alexandria::system::cpu_model());
    println!("       CPU Cores: {}", alexandria::system::cpu_cores());
    println!(
        "Installed Memory: {}",
        alexandria::system::installed_memory()
    );
    println!(
        "       OS Family: {}",
        alexandria::system::OsFamily::CURRENT
    );
    println!("         OS Name: {}", alexandria::system::os_name());
    println!(
        "      OS Version: {}",
        alexandria::system::os_version().unwrap()
    );
    println!(
        "        Hostname: {}",
        alexandria::system::hostname().unwrap()
    );
}
