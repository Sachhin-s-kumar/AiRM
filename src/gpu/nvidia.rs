use std::process::Command;

#[derive(Debug)]
pub struct NvidiaInfo {
    pub utilization: u32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub temperature: u32,
    pub power_draw: f32,
}

pub fn get_nvidia_info() -> Result<NvidiaInfo, String> {
    let output = Command::new("nvidia-smi")
        .arg("--query-gpu=utilization.gpu,memory.used,memory.total,temperature.gpu,power.draw")
        .arg("--format=csv,noheader,nounits")
        .output()
        .map_err(|e| format!("NVIDIA-SMI execution failed: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let data = String::from_utf8_lossy(&output.stdout);

    let values: Vec<&str> = data.trim().split(',').collect();

    if values.len() != 5 {
        return Err("Unexpected NVIDIA-SMI output".to_string());
    }

    Ok(NvidiaInfo {
        utilization: values[0]
            .trim()
            .parse()
            .map_err(|e| format!("GPU utilization error: {}", e))?,

        memory_used: values[1]
            .trim()
            .parse()
            .map_err(|e| format!("VRAM used error: {}", e))?,

        memory_total: values[2]
            .trim()
            .parse()
            .map_err(|e| format!("VRAM total error: {}", e))?,

        temperature: values[3]
            .trim()
            .parse()
            .map_err(|e| format!("Temperature error: {}", e))?,

        power_draw: values[4]
            .trim()
            .parse()
            .map_err(|e| format!("Power draw error: {}", e))?,
    })
}
