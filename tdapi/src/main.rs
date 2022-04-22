use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn main() {
    let v = vec![1, 2, 3];
    progress(v, expensive_calculation);
}

fn progress<T>(v: Vec<T>, f: fn(&T) -> ()) {
    let mut progress_state = 1;

    for n in v.iter() {
        println!("{}{}", CLEAR, "*".repeat(progress_state));
        progress_state += 1;
        f(n);
    }
}

fn expensive_calculation<T>(_n: &T) {
    sleep(Duration::from_secs(1));
}
