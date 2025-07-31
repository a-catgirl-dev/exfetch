pub fn memory() -> String {
    #[cfg(target_os = "linux")] return crate::linux::memory::memory_readout();
}

pub fn cpu() -> Option<String> {
    #[cfg(target_os = "linux")] return crate::linux::cpu::cpu();
}

pub fn arch() -> String {
    std::env::consts::ARCH.into() // oh my malloc
}

pub fn init() -> String {
    #[cfg(unix)] return crate::unix::init::get()
}

pub fn uptime() -> String {
    #[cfg(target_os = "linux")] return crate::linux::uptime::uptime()
}

pub fn os_ver() -> Option<String> {
    #[cfg(target_os = "linux")] return crate::linux::os::get();
}

pub fn swap() -> String {
    #[cfg(target_os = "linux")] return crate::linux::memory::swap_readout();
}

pub fn pkg() -> String {
    #[cfg(target_os = "linux")] return crate::linux::pkg::get();
}

pub fn shell() -> Option<String> {
    #[cfg(unix)] return crate::unix::shell::get();
}

