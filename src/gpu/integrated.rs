use serde_json::Value;
use std::process::Command;

const INTEL_DRM_DEVICE: &str = "drm:/dev/dri/card1";

#[derive(Debug)]
pub struct IntegratedGpuInfo {
    pub render_usage: f64,
    pub blitter_usage: f64,
    pub video_usage: f64,
    pub video_enhance_usage: f64,
    pub gpu_power: f64,
    pub package_power: f64,
    pub rc6: f64,
}

// Read a numeric value from a nested JSON object.
fn read_metric(data: &Value, path: &[&str]) -> Result<f64, String> {
    let mut current = data;

    for key in path {
        current = current
            .get(*key)
            .ok_or_else(|| format!("Missing telemetry field: {}", key))?;
    }

    current
        .as_f64()
        .ok_or_else(|| format!("Invalid numeric telemetry: {:?}", path))
}

pub fn get_integrated_info() -> Result<IntegratedGpuInfo, String> {
    let output = Command::new("intel_gpu_top")
        .arg("-J")
        .arg("-d")
        .arg(INTEL_DRM_DEVICE)
        .arg("-s")
        .arg("1000")
        .arg("-n")
        .arg("2")
        .output()
        .map_err(|e| format!("Intel GPU telemetry execution failed: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let data: Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Intel JSON parsing failed: {}", e))?;

    // Use the latest sample, avoiding the initial short sample.
    let samples = data
        .as_array()
        .ok_or("Expected an array of Intel GPU samples")?;

    let latest = samples
        .last()
        .ok_or("No Intel GPU telemetry samples received")?;

    Ok(IntegratedGpuInfo {
        render_usage: read_metric(latest, &["engines", "Render/3D", "busy"])?,

        blitter_usage: read_metric(latest, &["engines", "Blitter", "busy"])?,

        video_usage: read_metric(latest, &["engines", "Video", "busy"])?,

        video_enhance_usage: read_metric(latest, &["engines", "VideoEnhance", "busy"])?,

        gpu_power: read_metric(latest, &["power", "GPU"])?,

        package_power: read_metric(latest, &["power", "Package"])?,

        rc6: read_metric(latest, &["rc6", "value"])?,
    })
}
