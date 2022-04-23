use std::collections::HashSet;
use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn main() {
    let v = vec![1, 2, 3];
    progress(v.iter(), expensive_calculation);

    let mut h = HashSet::new();
    h.insert(0);
    progress(h.iter(), expensive_calculation);
}

fn progress<Iter>(iter: Iter, f: fn(Iter::Item) -> ())
where
    Iter: Iterator,
{
    let mut progress_state = 1;

    for n in iter {
        println!("{}{}", CLEAR, "*".repeat(progress_state));
        progress_state += 1;
        f(n);
    }
}

fn expensive_calculation<T>(_n: &T) {
    sleep(Duration::from_secs(1));
}
