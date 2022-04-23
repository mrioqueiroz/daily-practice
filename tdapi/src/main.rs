use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<Iter> {
    iter: Iter,
    progress_state: usize,
}

// For all types Iter, implement Progress of Iter.
impl<Iter> Progress<Iter> {
    // Constructor,
    fn new(iter: Iter) -> Self {
        Self {
            iter,
            progress_state: 0,
        }
    }
}

// Make the compiler understand that the progress data structure is an iterator
// and can be given to a for loop, by satisfying the requirements of the
// Iterator interface.
impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    // Item is whatever is returned from the inner iterator.
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.progress_state));
        self.progress_state += 1;
        self.iter.next()
    }
}

// Make the method progress() work for all types that implement Iterator:
// For all types Iter, implement the trait ProgressIteratorExt for that
// quantified type (Iter). We need the Sized bound here because Progress<Self>
// doesn't have a size known at compile-time.
trait ProgressIteratorExt: Sized {
    // Requires a function `progress` that takes an iterator and returns a
    // Progress of that iterator.
    fn progress(self) -> Progress<Self>;
}

// Implement ProgressIteratorExt for all iterators.
impl<Iter> ProgressIteratorExt for Iter {
    fn progress(self) -> Progress<Self> {
        // Just calls the constructor.
        Progress::new(self)
    }
}

fn expensive_calculation<T>(_n: &T) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = vec![1, 2, 3];
    for n in v.iter().progress() {
        expensive_calculation(n);
    }
}
