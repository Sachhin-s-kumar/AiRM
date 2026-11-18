use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

/// Refreshes all running processes, including executable paths and command-line
/// arguments required for dynamic AI/ML workload detection.
pub fn refresh_processes(sys: &mut System) {
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing()
            .with_memory()
            .with_cpu()
            .with_disk_usage()
            .with_exe(UpdateKind::Always)
            .with_cmd(UpdateKind::Always)
            .with_tasks(),
    );
}

pub fn display_processes(sys: &mut System) {
    // Refresh information about all running processes
    refresh_processes(sys);

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::process::{Child, Command, Stdio};
    use std::thread;
    use std::time::Duration;
    use sysinfo::Pid;

    struct ChildGuard(Child);

    impl ChildGuard {
        fn pid(&self) -> Pid {
            Pid::from_u32(self.0.id())
        }

        fn stop(mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }

    impl Drop for ChildGuard {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }

    #[test]
    fn default_sysinfo_refresh_processes_omits_cmd_for_new_process() {
        let mut sys = System::new_all().expect("Failed to initialize system information");

        let child = ChildGuard(
            Command::new("sh")
                .args(["-c", "read _", "sh", "torch_training.py"])
                .stdin(Stdio::piped())
                .spawn()
                .expect("Failed to spawn test process"),
        );
        let pid = child.pid();

        // Reproduce the previous buggy refresh call.
        sys.refresh_processes(ProcessesToUpdate::All, true);

        let process = sys
            .process(pid)
            .expect("Newly spawned process should be discovered in process table");
        assert!(
            process.cmd().is_empty(),
            "sysinfo::System::refresh_processes does not populate cmd() for newly discovered processes"
        );
        assert_eq!(crate::system::workload::detect_ai_workload(process), None);
    }

    #[test]
    fn refresh_processes_discovers_and_classifies_newly_launched_workload() {
        let mut sys = System::new_all().expect("Failed to initialize system information");

        let child = ChildGuard(
            Command::new("sh")
                .args(["-c", "read _", "sh", "torch_training.py"])
                .stdin(Stdio::piped())
                .spawn()
                .expect("Failed to spawn test process"),
        );
        let pid = child.pid();

        refresh_processes(&mut sys);

        let process = sys
            .process(pid)
            .expect("Newly spawned process should be present after refresh");
        assert!(
            process
                .cmd()
                .iter()
                .any(|arg| arg.to_string_lossy().contains("torch_training.py")),
            "Expected cmd() to contain torch_training.py, got {:?}",
            process.cmd()
        );

        let telemetry = crate::system::telemetry::collect_process_telemetry(&sys);
        let observation = telemetry
            .iter()
            .find(|obs| obs.pid == pid.as_u32())
            .expect("Telemetry should include the newly spawned process");
        assert_eq!(
            observation.heuristic_label.as_deref(),
            Some("AI/ML Workload")
        );

        // Verify exited processes are cleaned up on subsequent refresh.
        child.stop();
        refresh_processes(&mut sys);
        assert!(
            sys.process(pid).is_none(),
            "Terminated process should be removed after refresh"
        );
    }

    #[test]
    fn refresh_processes_updates_cmd_after_exec_in_place() {
        let mut sys = System::new_all().expect("Failed to initialize system information");

        // Start a process whose initial argv has no AI/ML keywords, then exec in place
        // (same PID and start_time) with a command line containing torch_training.py.
        let mut child = ChildGuard(
            Command::new("sh")
                .env("AIRM_TEST_SCRIPT", "torch_training.py")
                .args(["-c", "read _; exec sh -c 'read _' sh \"$AIRM_TEST_SCRIPT\""])
                .stdin(Stdio::piped())
                .spawn()
                .expect("Failed to spawn exec test process"),
        );
        let pid = child.pid();

        // First observation before exec should see the process without an AI/ML label.
        refresh_processes(&mut sys);
        let initial = sys
            .process(pid)
            .expect("Process should be present before exec");
        assert_eq!(crate::system::workload::detect_ai_workload(initial), None);

        // Trigger the in-place exec and verify cmd() is refreshed for the existing PID.
        {
            let stdin = child.0.stdin.as_mut().expect("Child stdin should be piped");
            stdin
                .write_all(b"\n")
                .expect("Failed to signal child process");
            stdin.flush().expect("Failed to flush child stdin");
        }

        let mut detected = false;
        for _ in 0..30 {
            thread::sleep(Duration::from_millis(50));
            refresh_processes(&mut sys);
            if let Some(proc) = sys.process(pid)
                && crate::system::workload::detect_ai_workload(proc) == Some("AI/ML Workload")
            {
                detected = true;
                break;
            }
        }

        assert!(
            detected,
            "Expected process to be classified as AI/ML Workload after exec updated cmdline"
        );
    }
}
