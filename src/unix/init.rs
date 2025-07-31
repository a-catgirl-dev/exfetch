use std::fs::File;
use std::io::{BufReader, Read};

#[cfg(unix)]
static INIT_SYSTEMS_UNIX: &[(&str, &str)] = &[
    #[cfg(target_os = "linux")]
    ("/usr/lib/systemd/systemd", "systemd"), // ew
    ("/sbin/openrc", "openrc"),
    ("/sbin/dinit", "dinit"),
];

#[allow(unreachable_code)]
pub fn get() -> String {
    #[cfg(target_os = "macos")] {
        // macos is unix like so it fits here. it uses launchd
        // (also xml config wtf?)
        return "launchd".to_string();
    }
    #[cfg(target_os = "android")] {
        // android runs linux, but custom init (not in INIT_SYSTEMS_UNIX), so specify here
        return "init.rc".to_string();
    }
    #[cfg(unix)] {
        // other types of unixes, eg *BSD, Linux, etc
        for (init_path, init_name) in INIT_SYSTEMS_UNIX {
            if File::open(init_path).is_err() {
                continue;
            }
            return (*init_name).to_string();
        }

        // FALLBACK: if we cannot find the init, return the contents of /proc/1/comm
        // we dont do this by default because on openrc, /proc/1/comm is `init`. very descriptive.
        let file = File::open("/proc/1/comm");
        if file.is_err() {
            return String::new();
        }

        let mut reader = BufReader::new(file.unwrap());
        let mut buf = String::new();
        reader.read_to_string(&mut buf).unwrap();
        buf
    }
}

