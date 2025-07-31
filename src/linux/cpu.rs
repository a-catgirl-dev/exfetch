fn process_cpu_name(text: &str) -> String {
    text.replace("(R)", "")
        .replace("(TM)", "")
        .replace(" @ ", "(")
        .replace("CPU", "")
        .replace("GHz", "GHz)")
}

pub fn cpu() -> Option<String> {
    if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if !line.starts_with("model name") {
                continue;
            }
            let parts: Vec<&str> = line.split(':').collect();
            if parts.is_empty() {
                continue;
            }
            let cpu_name = parts[1].trim();
            return Some(process_cpu_name(cpu_name));
        }
    }

    None
}

