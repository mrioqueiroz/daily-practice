use std::{fmt, time::SystemTime};

struct Grid {
    rows: usize,
    cols: usize,
    elements: Vec<Cell>,
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    state: State,
}

#[derive(Clone, Copy, Debug)]
enum State {
    Live,
    Dead,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            elements: vec![Cell { state: State::Dead }; rows * cols],
        }
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.rows + col
    }

    #[allow(dead_code)]
    fn debug(&self) {
        for row in 0..self.rows {
            println!();
            for col in 0..self.cols {
                let index = self.get_index(row, col);
                print!(" {}.{}.{}.{:?} ", row, col, index, self.elements[index]);
            }
        }
        println!();
    }

    fn dump(&self) {
        for row in 0..self.rows {
            println!();
            for col in 0..self.cols {
                let index = self.get_index(row, col);
                print!(" {} ", self.elements[index].state);
            }
        }
        println!();
    }

    fn populate(&mut self) {
        let now = SystemTime::now();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let index = self.get_index(row, col);
                let seed = now.elapsed().unwrap().as_nanos();
                let state = {
                    if seed % 2 == 0 {
                        State::Live
                    } else {
                        State::Dead
                    }
                };
                self.elements[index] = Cell { state };
            }
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            State::Live => "*",
            State::Dead => "⋅",
        };
        write!(f, "{printable}")
    }
}

fn main() {
    let mut grid = Grid::new(30, 30);
    grid.populate();
    grid.dump();
}
