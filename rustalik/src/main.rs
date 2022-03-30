// Use Const Generics.
#[allow(dead_code)]
#[derive(Debug)]
struct Board<T, const ROWS: usize, const COLS: usize> {
    // 2D array.
    // The size will be customizable.
    elems: [[T; COLS]; ROWS],
}

impl<T, const ROWS: usize, const COLS: usize> Default for Board<T, ROWS, COLS>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            elems: [[T::default(); COLS]; ROWS],
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Cell {
    Empty,
    RoomFloor,
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
            Cell::RoomFloor => '.',
            Cell::VertWall => '|',
            Cell::HorzWall => '-',
            Cell::Passage => '#',
            Cell::Door => '+',
        }
    }
}

fn main() {
    let _board = Board::<Cell, 10, 10>::default();
    println!("{}", Cell::Passage.to_char());
}
