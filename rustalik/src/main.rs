use std::ops::{Index, IndexMut, Range};

use geometry::Rectangle;

use crate::geometry::Point;

mod geometry {
    use std::cmp::{max, min};

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub struct Point(pub usize, pub usize);

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    pub struct Rectangle(Point, Point);

    impl Rectangle {
        #[allow(dead_code)]
        pub fn new(Point(row1, col1): Point, Point(row2, col2): Point) -> Self {
            Self(
                Point(min(row1, row2), min(col1, col2)),
                Point(max(row1, row2), max(col1, col2)),
            )
        }

        pub fn corner1(&self) -> &Point {
            &self.0
        }

        pub fn corner2(&self) -> &Point {
            &self.1
        }
    }
}

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

    #[allow(dead_code)]
    fn fill_rectangle(&mut self, rectangle: Rectangle, x: T) {
        let Point(row1, col1) = *rectangle.corner1();
        let Point(row2, col2) = *rectangle.corner2();
        // let Rectangle(Point(row1, col1), Point(row2, col2)) = rectangle.normalize();
        for row in row1..=row2 {
            for col in col1..=col2 {
                self[Point(row, col)] = x;
            }
        }
    }
}

impl<T> Index<Point> for Board<T> {
    type Output = T;
    fn index(&self, Point(row, col): Point) -> &Self::Output {
        &self.elems[row * self.cols + col]
    }
}

impl<T> IndexMut<Point> for Board<T> {
    fn index_mut(&mut self, Point(row, col): Point) -> &mut Self::Output {
        &mut self.elems[row * self.cols + col]
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
    // let p = Point(10, 10);
    let board = Board::<Cell>::new(10, 10, Cell::Floor);
    for row in board.rows_range() {
        for col in board.cols_range() {
            print!("{}", board[Point(row, col)].to_char());
        }
        println!();
    }
}
