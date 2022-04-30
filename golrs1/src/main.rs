use std::{fmt, time::SystemTime};
use std::{thread::sleep, time::Duration};

const SIZE: usize = 30;
const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct GameOfLife {
    grid: Grid,
}

struct Grid {
    cells: [[Cell; SIZE]; SIZE],
}

type Position = (usize, usize);

#[derive(Clone, Copy, Debug, Default)]
struct Cell {
    state: State,
    position: Position,
}

#[derive(Clone, Copy, Debug)]
enum State {
    Live,
    Dying,
    Dead,
}

impl GameOfLife {
    fn new() -> Self {
        Self { grid: Grid::new() }
    }

    fn compute_new_generation(&mut self) -> Self {
        let mut ng = Self::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                ng.grid.cells[row][col].state =
                    self.grid.cells[row][col].calculate_new_state(&self.grid);
            }
        }
        ng
    }
}

impl Grid {
    fn new() -> Self {
        // TODO: Improve this variable initialization (closure?),
        let mut cells: [[Cell; SIZE]; SIZE] = [[Cell::new((0, 0)); SIZE]; SIZE];
        for row in 0..SIZE {
            for col in 0..SIZE {
                cells[row][col] = Cell::new((row, col));
            }
        }
        Self { cells }
    }

    // TODO: Implement display  trait to print it directly.
    fn dump(&self) {
        for row in 0..self.cells.len() {
            println!();
            for col in 0..self.cells[row].len() {
                print!(" {} ", self.cells[row][col].state);
            }
        }
        println!();
    }
}

impl Cell {
    // Will create a new cell (that may be dead or live) in the specified position in the grid.
    fn new(position: Position) -> Self {
        let now = SystemTime::now();
        let seed = now.elapsed().unwrap().as_nanos();
        let state = match seed % 2 {
            0 => State::Live,
            _ => State::Dead,
        };
        Self { state, position }
    }

    fn live_neighbours(&self, grid: &Grid) -> usize {
        let mut live = 0;
        for n_row in &[-1, 0, 1] {
            for n_col in &[-1, 0, 1] {
                if !(*n_row == 0 && *n_col == 0) {
                    live += grid.cells
                        [((self.position.0 as isize + *n_row).rem_euclid(SIZE as isize)) as usize]
                        [((self.position.1 as isize + *n_col).rem_euclid(SIZE as isize)) as usize]
                        .state
                        .as_usize();
                }
            }
        }
        live
    }

    fn calculate_new_state(&self, grid: &Grid) -> State {
        let live_neighbours = self.live_neighbours(grid);
        match self.state {
            State::Live => match live_neighbours {
                2 | 3 => self.state,
                _ => State::Dying,
            },
            State::Dead | State::Dying => match live_neighbours {
                3 => State::Live,
                _ => State::Dead,
            },
        }
    }
}

impl State {
    fn as_usize(&self) -> usize {
        match self {
            Self::Dead | Self::Dying => 0,
            Self::Live => 1,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            State::Live => "*",
            State::Dying => "⋅",
            State::Dead => " ",
        };
        write!(f, "{printable}")
    }
}

impl Default for State {
    fn default() -> Self {
        State::Dead
    }
}

fn main() {
    let mut gol = GameOfLife::new();
    loop {
        print!("{}", CLEAR);
        gol = gol.compute_new_generation();
        gol.grid.dump();
        sleep(Duration::from_millis(200));
    }
}
