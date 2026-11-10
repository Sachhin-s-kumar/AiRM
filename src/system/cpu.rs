use sysinfo::System;

// Retrieve CPU usage from the existing system monitor
pub fn get_cpu_usage(sys: &mut System) -> f32 {
    // Refresh CPU statistics
    sys.refresh_cpu_usage();

    // Return overall CPU usage percentage
    sys.global_cpu_usage()
}
