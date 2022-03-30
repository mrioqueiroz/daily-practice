// Use Const Generics.
#[allow(dead_code)]
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
    #[allow(dead_code)]
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
    let _board = Board::<Cell>::new(10, 10, Cell::Floor);
    println!("{}", Cell::Passage.to_char());
}
