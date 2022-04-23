use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<Iter> {
    iter: Iter,
    progress_state: usize,
    bound: Option<usize>,
}

// For all types Iter, implement Progress of Iter.
impl<Iter> Progress<Iter> {
    // Constructor,
    fn new(iter: Iter) -> Self {
        Self {
            iter,
            progress_state: 0,
            bound: None,
        }
    }
}

// According to signature, ExactSizeIterator requires that anything that
// implements this trait also implement Iterator. So, ExactSizeIterator is an
// iterator that ALSO has the two methods len() and is_empty(). Here, it
// inherits only the specification, not the implementation (use
// inheritance-style to express requirements).
impl<Iter> Progress<Iter>
where
    Iter: ExactSizeIterator,
{
    // Add this method to the Progress data structure, but only where this type
    // Iter implements the trait ExactSizeIterator.
    #[allow(dead_code)]
    fn with_bound(mut self) -> Self {
        self.bound = Some(self.iter.len());
        self
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
        print!("{}", CLEAR);
        match self.bound {
            // If we have the bound...
            Some(bound) => println!(
                "[{}{}]",
                "*".repeat(self.progress_state),
                " ".repeat(bound - self.progress_state)
            ),
            None => println!("{}", "*".repeat(self.progress_state)),
        }
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
impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self> {
        // Just calls the constructor.
        Progress::new(self)
    }
}

fn expensive_calculation<T>(_n: &T) {
    sleep(Duration::from_secs(1));
}

fn main() {
    // Bounded.
    let v = vec![1, 2, 3];
    for n in v.iter().progress().with_bound() {
        expensive_calculation(n);
    }

    // Not bounded.
    for n in (0..).progress() {
        expensive_calculation(&n);
    }
}
