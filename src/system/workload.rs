use sysinfo::Process;

/// Identifies whether a process is likely to be an AI/ML workload.
/// Returns the detected workload type, or None if no match is found.
pub fn detect_ai_workload(process: &Process) -> Option<&'static str> {
    // Read the process name.
    let name = process.name().to_string_lossy().to_lowercase();

    // Read the executable path, if available.
    let executable = process
        .exe()
        .map(|path| path.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    // Read command-line arguments.
    let command = process
        .cmd()
        .iter()
        .map(|arg| arg.to_string_lossy().to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");

    // Combine process information for matching.
    let details = format!("{name} {executable} {command}");

    // Detect known AI/ML workload indicators.
    if ["torchrun", "vllm", "ollama", "llama-server"]
        .iter()
        .any(|keyword| name.contains(keyword))
    {
        return Some("AI Inference");
    }

    if [
        "pytorch",
        "torch",
        "tensorflow",
        "keras",
        "ultralytics",
        "yolo",
        "transformers",
        "accelerate",
    ]
    .iter()
    .any(|keyword| details.contains(keyword))
    {
        return Some("AI/ML Workload");
    }

    None
}
