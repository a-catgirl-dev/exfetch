use sysinfo_dot_h::collect;

pub fn memory_readout() -> String {
    let si = collect();
    let total = si.totalram / 1024000;
    let used = (si.totalram - si.freeram) / 1024000;

    format!("{used}/{total} MB")
}

pub fn swap_readout() -> String {
    let si = collect();
    let swap = si.totalswap;
    if swap == 0 {
        return "Disabled".into();
    }
    let used = (swap - si.freeswap) / 1024000;
    let total = swap / 1024000;

    format!("{used}/{total} MB")
}

