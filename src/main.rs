mod readout;

mod linux;
mod unix;

// TODO: better name for this?
// Registry.0 is an optional function ptr to String
// Registry.1 is an optional function ptr to Option<String>
// Registry.2 is the icon
//
// putting into typedef because otherwise clippy would complain. heh
type FunctionRegistry<'a> = (Option<fn() -> String>, Option<fn() -> Option<String>>, &'a str);

fn get_max_value_of_vec(vec: &[usize]) -> usize {
    vec.iter().max().map_or_else(|| panic!("the entire vector is empty, wtf?"), |max| *max)
}

fn draw_box_opening(text: &str, times: usize) {
    println!("╭─{}{}╮", text, "─".repeat((times + 4) - text.len()));
}

fn draw_box_closure(times: usize) {
    let padding = "─".repeat(times + 5);
    println!("╰{padding}╯");
}

fn iterate<'a>(
    string_registry: &mut Vec<(String, &'a str)>,
    lengths: &mut Vec<usize>,
    func_one: Option<fn() -> String>,
    func_two: Option<fn() -> Option<String>>,
    icon: &'a str
) {
    if let Some(string) = func_one {
        let string = string();

        lengths.push(icon.len() + string.len());
        string_registry.push((string, icon));
    } else if let Some(string) = func_two {
        let string = string();

        if string.is_none() { return };
        let string = string.unwrap();
        lengths.push(icon.len() + string.len());
        string_registry.push((string, icon));
    } else { panic!("invalid state: both function ptrs are None, wtf?") }
}

fn print(text: &str, icon: &str, long: usize) {
    let padding = long.saturating_sub(icon.len() + text.len());
    let padding = " ".repeat(padding);
    println!("│ \x1B[0;35m{icon}\x1B[0m ~ {text}{padding} │");
}

fn main() {
    // the ordering in which these are laid out determine where they will show when it gets
    // printed.
    let hardware_registry: [FunctionRegistry; 4] = [
        (None, Some(readout::cpu),"CPU"),
        (Some(readout::memory), None, "Mem"),
        (Some(readout::arch), None, "Arch"),
        (Some(readout::uptime), None, "Up"),
    ];

    let software_registry: [FunctionRegistry; 5] = [
        (None, Some(readout::shell), "sh"),
        (Some(readout::pkg), None, "PKGs"),
        (None, Some(readout::os_ver), "OS"),
        (Some(readout::init), None, "Init"),
        (Some(readout::swap), None, "Swap"),
    ];

    // store results because we need to calculate lengths
    let mut hardware_registry_cache: Vec<(String, &str)> = Vec::new();
    let mut software_registry_cache: Vec<(String, &str)> = Vec::new();
    // for box calculation later (find longest string)
    let mut lengths: Vec<usize> = Vec::new();

    for (func_one, func_two, icon) in &hardware_registry {
        iterate(&mut hardware_registry_cache, &mut lengths, *func_one, *func_two, icon);
    }
    for (func_one, func_two, icon) in &software_registry {
        iterate(&mut software_registry_cache, &mut lengths, *func_one, *func_two, icon);
    }

    let long = get_max_value_of_vec(&lengths);

    draw_box_opening("HARDWARE", long);
    for (text, icon) in hardware_registry_cache {
        print(&text, icon, long);
    }
    draw_box_closure(long);
    draw_box_opening("SOFTWARE", long);
    for (text, icon) in software_registry_cache {
        print(&text, icon, long);
    }
    draw_box_closure(long);
}

