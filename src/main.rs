
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

    // Initialize system monitor only once.
    let mut sys = sysinfo::System::new_all()
        .expect("Failed to initialize system information");

    // Continuously monitor system resources.
    loop {
        // CPU monitoring.
        let cpu_usage = system::cpu::get_cpu_usage(&mut sys);
        println!("CPU Usage: {:.2}%", cpu_usage);

        // RAM monitoring.
        let memory = system::memory::get_memory_usage(&mut sys);
        let gb = 1024.0 * 1024.0 * 1024.0;

        println!("RAM Usage: {:.2}%", memory.usage_percent);
        println!(
            "RAM: {:.2} GB / {:.2} GB",
            memory.used as f64 / gb,
            memory.total as f64 / gb
        );
        println!("Available RAM: {:.2} GB", memory.available as f64 / gb);

        // Process monitoring.
        // This refreshes process information for the current cycle.
        system::process::display_processes(&mut sys);

        // Collect process telemetry from the refreshed snapshot.
        let telemetry =
            system::telemetry::collect_process_telemetry(&sys);

        println!("========== TELEMETRY COLLECTION ==========");
        println!(
            "Process observations collected: {}",
            telemetry.len()
        );
        println!("==========================================");

        // AI/ML workload detection using collected telemetry.
        println!("========== AI/ML WORKLOADS ==========");

        let mut detected = false;

        for observation in &telemetry {
            if let Some(workload) = &observation.heuristic_label {
                detected = true;

                println!(
                    "PID: {} | Process: {} | Type: {}",
                    observation.pid,
                    observation.process_name,
                    workload
                );
            }
        }

        if !detected {
            println!("No AI/ML workloads detected.");
        }

        println!("=====================================");

        // NVIDIA GPU monitoring.
        let mut nvidia_gpu_usage = None;

        match gpu::nvidia::get_nvidia_info() {
            Ok(info) => {
                println!("========== NVIDIA GPU ==========");
                println!("GPU Usage: {}%", info.utilization);
                println!(
                    "VRAM: {} MiB / {} MiB",
                    info.memory_used,
                    info.memory_total
                );
                println!("Temperature: {} °C", info.temperature);
                println!("Power Draw: {:.2} W", info.power_draw);

                nvidia_gpu_usage = Some(info.utilization);
            }

            Err(error) => {
                eprintln!("NVIDIA Monitoring Error: {}", error);
            }
        }

        // Intel Integrated GPU monitoring.
        let mut intel_gpu_usage = None;

        match gpu::integrated::get_integrated_info() {
            Ok(info) => {
                println!("========== INTEL INTEGRATED GPU ==========");
                println!("Render/3D Usage: {:.2}%", info.render_usage);
                println!("Blitter Usage: {:.2}%", info.blitter_usage);
                println!("Video Usage: {:.2}%", info.video_usage);
                println!(
                    "Video Enhancement: {:.2}%",
                    info.video_enhance_usage
                );
                println!("GPU Power: {:.2} W", info.gpu_power);
                println!("Package Power: {:.2} W", info.package_power);
                println!("RC6: {:.2}%", info.rc6);

                intel_gpu_usage = Some(info.render_usage);
            }

            Err(error) => {
                eprintln!("Intel GPU Monitoring Error: {}", error);
            }
        }

        // Resource analysis.
        println!("========== RESOURCE ANALYSIS ==========");

        let warnings = system::analysis::analyze_resources(
            cpu_usage,
            memory.usage_percent,
            nvidia_gpu_usage,
            intel_gpu_usage,
        );

        if warnings.is_empty() {
            println!("No resource pressure detected.");
        } else {
            for warning in warnings {
                println!("WARNING: {}", warning);
            }
        }

        println!("=======================================");

        // Wait one second before the next reading.
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
