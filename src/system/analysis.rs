const CPU_THRESHOLD: f32 = 85.0;
const RAM_THRESHOLD: f64 = 85.0;
const NVIDIA_GPU_THRESHOLD: u32 = 90;
const INTEL_GPU_THRESHOLD: f64 = 90.0;

/// Analyses system resource usage and identifies high utilization.
///
/// Returns a list of resource pressure warnings.
pub fn analyze_resources(
    cpu_usage: f32,
    ram_usage: f64,
    nvidia_gpu_usage: Option<u32>,
    intel_gpu_usage: Option<f64>,
) -> Vec<&'static str> {
    let mut warnings = Vec::new();

    // Analyse CPU utilization.
    if cpu_usage > CPU_THRESHOLD {
        warnings.push("High CPU usage detected");
    }

    // Analyse RAM utilization.
    if ram_usage > RAM_THRESHOLD {
        warnings.push("High RAM usage detected");
    }

    // Analyse NVIDIA GPU utilization.
    if let Some(gpu_usage) = nvidia_gpu_usage {
        if gpu_usage > NVIDIA_GPU_THRESHOLD {
            warnings.push("High NVIDIA GPU usage detected");
        }
    }

    // Analyse Intel GPU Render/3D utilization.
    if let Some(gpu_usage) = intel_gpu_usage {
        if gpu_usage > INTEL_GPU_THRESHOLD {
            warnings.push("High Intel GPU usage detected");
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_high_cpu_usage() {
        let warnings = analyze_resources(90.0, 40.0, None, None);
        assert!(warnings.contains(&"High CPU usage detected"));
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn detects_high_ram_usage() {
        let warnings = analyze_resources(40.0, 90.0, None, None);
        assert!(warnings.contains(&"High RAM usage detected"));
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn detects_high_nvidia_gpu_usage() {
        let warnings = analyze_resources(40.0, 40.0, Some(95), None);
        assert!(warnings.contains(&"High NVIDIA GPU usage detected"));
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn detects_high_intel_gpu_usage() {
        let warnings = analyze_resources(40.0, 40.0, None, Some(95.0));
        assert!(warnings.contains(&"High Intel GPU usage detected"));
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn detects_multiple_resource_warnings() {
        let warnings = analyze_resources(90.0, 90.0, Some(95), Some(95.0));
        assert_eq!(warnings.len(), 4);
    }

    #[test]
    fn reports_no_pressure_under_thresholds() {
        let warnings = analyze_resources(50.0, 60.0, Some(70), Some(70.0));
        assert!(warnings.is_empty());
    }

    #[test]
    fn handles_unavailable_gpu_telemetry() {
        let warnings = analyze_resources(50.0, 60.0, None, None);
        assert!(warnings.is_empty());
    }
}
