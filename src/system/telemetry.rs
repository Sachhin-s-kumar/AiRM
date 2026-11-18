use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::System;

/// Stores one observation of a running process.
#[derive(Debug, Serialize)]
pub struct ProcessTelemetry {
    pub timestamp_unix_ms: u64,
    pub pid: u32,
    pub process_name: String,
    pub cpu_usage_percent: f32,
    pub memory_bytes: u64,
    pub heuristic_label: Option<String>,
}


/// Collects telemetry observations from running processes.
pub fn collect_process_telemetry(sys: &System) -> Vec<ProcessTelemetry> {
    // Generate a timestamp for this collection cycle.
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    // Collect telemetry for each running process.
    sys.processes()
        .iter()
        .map(|(pid, process)| {
            let heuristic_label =
                crate::system::workload::detect_ai_workload(process)
                    .map(String::from);

            ProcessTelemetry {
                timestamp_unix_ms: timestamp,
                pid: pid.as_u32(),
                process_name: process.name().to_string_lossy().into_owned(),
                cpu_usage_percent: process.cpu_usage(),
                memory_bytes: process.memory(),
                heuristic_label,
            }
        })
        .collect()
}