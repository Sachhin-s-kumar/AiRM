use sysinfo::System;

// Stores the memory statistics collected from the system.
pub struct MemoryStats {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f64,
}

// Retrieve RAM statistics from the existing system monitor.
pub fn get_memory_usage(sys: &mut System) -> MemoryStats {
    // Refresh memory information.
    sys.refresh_memory();

    // Collect memory values in bytes.
    let total = sys.total_memory();
    let used = sys.used_memory();
    let available = sys.available_memory();

    // Calculate RAM utilization percentage.
    let usage_percent = if total > 0 {
        (used as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    MemoryStats {
        total,
        used,
        available,
        usage_percent,
    }
}
