use std::io::Write;

use board::Vec2;

use crate::board::{Board, Point};

mod board {
    use std::cmp::{max, min};
    use std::ops::{Add, Index, IndexMut, Range};

    #[derive(Clone, Copy)]
    pub struct Point(pub usize, pub usize);

    impl Add<Vec2> for Point {
        type Output = Point;
        fn add(self, Vec2(drow, dcol): Vec2) -> Self::Output {
            let Point(row, col) = self;
            // drow can be negative.
            Point(
                (row as isize + drow) as usize,
                (col as isize + dcol) as usize,
            )
        }
    }

    #[derive(Clone, Copy)]
    pub struct Vec2(pub isize, pub isize);

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

        pub fn get(&self, point: Point) -> Option<&T> {
            if self.contains(point) {
                Some(&self[point])
            } else {
                None
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

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    #[allow(dead_code)]
    fn to_vec2(self) -> Vec2 {
        match self {
            Self::N => Vec2(-1, 0),
            Self::S => Vec2(1, 0),
            Self::E => Vec2(0, 1),
            Self::W => Vec2(0, -1),
        }
    }

    fn from_key(key: char) -> Option<Self> {
        match key {
            'k' => Some(Self::N),
            'j' => Some(Self::S),
            'l' => Some(Self::E),
            'h' => Some(Self::W),
            _ => None,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

impl Cell {
    #[allow(dead_code)]
    fn is_walkable(&self) -> bool {
        match self {
            Cell::Empty => false, // the void of the game.
            Cell::Floor => true,
            Cell::VertWall => false,
            Cell::HorzWall => false,
            Cell::Passage => true,
            Cell::Door => true,
        }
    }

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
    quit: bool,
}

impl Rogalik {
    #[allow(dead_code)]
    fn new(rows: usize, cols: usize) -> Self {
        Rogalik {
            board: Board::new(rows, cols, Cell::Floor),
            player_pos: Point(0, 0),
            quit: false,
        }
    }

    fn render(&self, display: &mut Board<char>) {
        display.fill_rectangle(display.rectangle(), ' ');
        for row in self.board.rows_range() {
            for col in self.board.cols_range() {
                let point = Point(row, col);
                if display.contains(point) {
                    display[point] = self.board[point].to_char();
                }
            }
        }
        if display.contains(self.player_pos) {
            display[self.player_pos] = '@';
        }
    }

    #[allow(dead_code)]
    fn move_to(&mut self, dir: Direction) {
        let next_pos = self.player_pos + dir.to_vec2();
        if let Some(cell) = self.board.get(next_pos) {
            if cell.is_walkable() {
                self.player_pos = next_pos;
            }
        }
        if self.board.contains(next_pos) && self.board[next_pos].is_walkable() {}
    }

    #[allow(dead_code)]
    fn quit(&mut self) {
        self.quit = true;
    }
}

#[allow(dead_code)]
fn print_display(display: &Board<char>) {
    for row in display.rows_range() {
        for col in display.cols_range() {
            print!("{}", display[Point(row, col)]);
        }
        println!();
    }
}

fn main() {
    use std::io;

    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    let mut display = Board::new(WIDTH, HEIGHT, ' ');
    let mut rogalik = Rogalik::new(WIDTH, HEIGHT);
    let mut line = String::new();

    rogalik.render(&mut display);
    print_display(&display);
    while !rogalik.quit {
        print!("> ");
        io::stdout().flush().unwrap();
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        for key in line.chars() {
            if let Some(dir) = Direction::from_key(key) {
                rogalik.move_to(dir);
            } else {
                println!("unknown key {}", key);
            }
        }
        rogalik.render(&mut display);
        print_display(&display);
    }
}
