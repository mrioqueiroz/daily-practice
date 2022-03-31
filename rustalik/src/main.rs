use std::ops::{Index, Range};

#[derive(Debug)]
struct Board<T> {
    // 2D array.
    // The size will be customizable.
    // Runtime value (dynamic).
    elems: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Board<T>
where
    T: Clone + Copy,
{
    fn new(rows: usize, cols: usize, x: T) -> Self {
        Self {
            rows,
            cols,
            elems: vec![x; rows * cols],
        }
    }

    fn rows_range(&self) -> Range<usize> {
        0..self.rows
    }

    fn cols_range(&self) -> Range<usize> {
        0..self.cols
    }
}

impl<T> Index<(usize, usize)> for Board<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.elems[row * self.cols + col]
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Floor,
    VertWall,
    HorzWall,
    Passage,
    Door,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

impl Cell {
    fn to_char(self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::Floor => '.',
            Cell::VertWall => '|',
            Cell::HorzWall => '-',
            Cell::Passage => '#',
            Cell::Door => '+',
        }
    }
}

fn main() {
    let board = Board::<Cell>::new(10, 10, Cell::Floor);
    for row in board.rows_range() {
        for col in board.cols_range() {
            print!("{}", board[(row, col)].to_char());
        }
        println!();
    }
}
