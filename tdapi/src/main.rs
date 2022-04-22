use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn main() {
    let v = vec![1, 2, 3];
    progress(v);
}

fn progress(v: Vec<i32>) {
    let mut progress_state = 1;

    for n in v.iter() {
        println!("{}{}", CLEAR, "*".repeat(progress_state));
        progress_state += 1;
        expensive_calculation(n);
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}
