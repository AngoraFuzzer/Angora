use std::time;

pub fn format_time(duration: time::Duration) -> String {
    let mut s = duration.as_secs();
    let mut m = s / 60;
    let h = m / 60;
    s = s % 60;
    m = m % 60;
    format!("[{:02}:{:02}:{:02}]", h, m, s)
}

pub fn format_count(c: usize) -> String {
    if c > 1000000000 {
        let f = c / 10000000;
        format!("{:.2}b", f as f32 / 100.0)
    } else if c > 1000000 {
        let f = c / 10000;
        format!("{:.2}m", f as f32 / 100.0)
    } else if c > 10000 {
        let f = c / 10;
        format!("{:.2}k", f as f32 / 100.0)
    } else {
        format!("{}", c)
    }
}
