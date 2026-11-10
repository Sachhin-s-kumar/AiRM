mod gpu;
mod scheduler;
mod system;
mod utils;

const APP_NAME: &str = "AiRM";
const APP_VERSION: &str = "0.1.0";

fn get_version() -> &'static str {
    APP_VERSION
}

fn main() {
    println!("==================================");
    println!("       {} v{}", APP_NAME, get_version());
    println!("==================================");
    println!("AI Training Resource Manager");
    println!("Platform: Fedora Linux");
    println!("Status: Continuous Monitoring");
    println!("Press Ctrl + C to stop.");
    println!("----------------------------------");

    // Initialize system monitor only once
    let mut sys = sysinfo::System::new_all().expect("Failed to initialize system information");

    // Continuously monitor CPU usage
    loop {
        let cpu_usage = system::cpu::get_cpu_usage(&mut sys);

        println!("CPU Usage: {:.2}%", cpu_usage);

        // Wait one second before the next reading
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_correct() {
        assert_eq!(get_version(), "0.1.0");
    }
}
