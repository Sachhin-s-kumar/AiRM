use sysinfo::{ProcessesToUpdate, System};

pub fn display_processes(sys: &mut System) {
    // Refresh information about all running processes
    sys.refresh_processes(ProcessesToUpdate::All, true);

    println!("========== RUNNING PROCESSES ==========");

    // Collect all processes
    let mut processes: Vec<_> = sys.processes().iter().collect();

    // Sort processes by CPU usage (highest first)
    processes.sort_by(|a, b| b.1.cpu_usage().total_cmp(&a.1.cpu_usage()));

    // Display the top 10 processes
    for (pid, process) in processes.iter().take(10) {
        println!(
            "PID: {:<8} Name: {:<20} CPU: {:>6.2}% Memory: {:>8.2} MB",
            pid,
            process.name().to_string_lossy(),
            process.cpu_usage(),
            process.memory() as f64 / (1024.0 * 1024.0)
        );
    }

    println!("=======================================");
}
