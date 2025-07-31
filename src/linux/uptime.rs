pub fn uptime() -> String {
    let si = sysinfo_dot_h::collect();
    exfetch::format_uptime_from_secs(si.uptime)
}
