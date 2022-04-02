use crate::board::{Board, Point, Rectangle};

mod board {
    use std::cmp::{max, min};
    use std::ops::{Index, IndexMut, Range};

    #[derive(Clone, Copy)]
    pub struct Point(pub usize, pub usize);

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

        #[allow(dead_code)]
        pub fn corner1(&self) -> &Point {
            &self.0
        }

        #[allow(dead_code)]
        pub fn corner2(&self) -> &Point {
            &self.1
        }
    }

    #[derive(Debug)]
    pub struct Board<T> {
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
        pub fn new(rows: usize, cols: usize, x: T) -> Self {
            Self {
                rows,
                cols,
                elems: vec![x; rows * cols],
            }
        }

        #[allow(dead_code)]
        pub fn rectangle(&self) -> Rectangle {
            Rectangle(Point(0, 0), Point(self.rows - 1, self.cols - 1))
        }

        pub fn rows_range(&self) -> Range<usize> {
            0..self.rows
        }

        pub fn cols_range(&self) -> Range<usize> {
            0..self.cols
        }

        #[allow(dead_code)]
        pub fn fill_rectangle(&mut self, rectangle: Rectangle, x: T) {
            let Rectangle(Point(row1, col1), Point(row2, col2)) = rectangle;
            for row in row1..=row2 {
                for col in col1..=col2 {
                    self[Point(row, col)] = x;
                }
            }
        }

        #[allow(dead_code)]
        pub fn contains(&self, Point(row, col): Point) -> bool {
            (0..self.rows).contains(&row) && (0..self.cols).contains(&col)
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

#[allow(dead_code)]
struct Rogalik {
    board: Board<Cell>,
    player_pos: Point,
}

impl Rogalik {
    #[allow(dead_code)]
    fn new(rows: usize, cols: usize) -> Self {
        Rogalik {
            board: Board::new(rows, cols, Cell::Floor),
            player_pos: Point(0, 0),
        }
    }

    fn render(&self, display: &mut Board<char>) {
        for row in self.board.rows_range() {
            for col in self.board.cols_range() {
                let point = Point(row, col);
                if display.contains(point) {
                    display[point] = self.board[point].to_char();
                }
            }
        }
    }
}

fn main() {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    let mut display = Board::new(WIDTH, HEIGHT, ' ');
    let mut rogalik = Rogalik::new(WIDTH, HEIGHT);

    rogalik.render(&mut display);

    for row in display.rows_range() {
        for col in display.cols_range() {
            print!("{}", display[Point(row, col)]);
        }
        println!();
    }
}
