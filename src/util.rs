use std::time::Duration;

pub fn to_bps(bytes: u64, duration: Duration) -> f64 {
    let bits = bytes * 8;
    bits as f64 / duration.as_secs_f64()
}

pub fn format_bps(mut bbs: f64) -> String {
    let units = ["bps", "Kbps", "Mbps", "Gbps", "Tbps"];
    let mut idx = 0;
    while bbs > 1024f64 && idx < units.len() {
        idx += 1;
        bbs /= 1024f64;
    }

    format!("{:.2} {}", bbs, units[idx])
}

pub fn format_bytes(bytes: u64) -> String {
    let mut bytes = bytes as f64;
    let units = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut idx = 0;
    while bytes >= 1024f64 && idx < units.len() {
        idx += 1;
        bytes /= 1024f64;
    }

    format!("{} {}", bytes, units[idx])
}
