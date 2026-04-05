const UNITS: [&str; 5] = ["b", "kb", "mb", "gb", "tb"];

pub fn format(bytes: u64) -> String {
    let mut bytes = bytes as f64;
    let mut unit = 0;
    while bytes >= 1024.0 && unit < UNITS.len() - 1 {
        bytes /= 1024.0;
        unit += 1;
    }
    format!("{:.2} {}", bytes, UNITS[unit])
}
