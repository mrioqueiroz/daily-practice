use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Bounded {
    bound: usize,
    delims: (char, char),
}
struct Unbounded;

struct Progress<Iter, Bound> {
    iter: Iter,
    state: usize,
    bound: Bound,
}

// Ties the Bounded and Unbounded data structures together.
trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            "*".repeat(progress.state),
            " ".repeat(self.bound - progress.state),
            self.delims.1
        );
    }
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "*".repeat(progress.state));
    }
}

// For all types Iter, implement Progress of Iter (with Unbounded as the
// default).
impl<Iter> Progress<Iter, Unbounded> {
    // Constructor,
    fn new(iter: Iter) -> Self {
        Self {
            iter,
            state: 0,
            bound: Unbounded,
        }
    }
}

// ExactSizeIterator requires that anything that implements this trait also
// implement Iterator. So, ExactSizeIterator is an iterator that ALSO has the
// two methods len() and is_empty(). Here, it inherits only the specification,
// not the implementation (use inheritance-style to express requirements).
impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    // Add this method to the Progress data structure, but only where this type
    // Iter implements the trait ExactSizeIterator. This will change the type
    // of the progress bar (type state).
    fn with_bound(self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        Progress {
            state: self.state,
            iter: self.iter,
            bound,
        }
    }
}

// Is only implemented for the bounded state (when we know the type Bounded is
// the thing insidedof our Progress data structure).
impl<Iter> Progress<Iter, Bounded> {
    // Customize delimiters.
    fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

// Make the compiler understand that the progress data structure is an iterator
// and can be given to a for loop, by satisfying the requirements of the
// Iterator interface.
impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where
    Iter: Iterator,
    Bound: ProgressDisplay,
{
    // Item is whatever is returned from the inner iterator.
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        self.bound.display(self);
        self.state += 1;
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
    fn progress(self) -> Progress<Self, Unbounded>;
}

// Implement ProgressIteratorExt for all iterators.
impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self, Unbounded> {
        // Just calls the constructor.
        Progress::new(self)
    }
}

fn expensive_calculation<T>(_n: &T) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let brackets = ('<', '>');
    // Bounded.
    let v = vec![1, 2, 3];
    for n in v.iter().progress().with_bound().with_delims(brackets) {
        expensive_calculation(n);
    }

    // Unbounded.
    for n in (0..).progress() {
        expensive_calculation(&n);
    }
}
