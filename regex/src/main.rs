use std::ops::Range;

const FSM_COLUMN_SIZE: usize = 127;

struct Fsm {
    columns: Vec<FsmColumn>,
}

impl Fsm {
    fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    fn push(&mut self, column: FsmColumn) {
        self.columns.push(column);
    }

    fn dump(&self) {
        for row in 0..FSM_COLUMN_SIZE {
            print!("{:03} = ", row);
            for col in &self.columns {
                print!("{:?} ", col.transition[row]);
            }
            println!();
        }
    }
}

type FsmIndex = usize;

#[derive(Debug)]
struct FsmColumn {
    transition: [FsmIndex; FSM_COLUMN_SIZE],
}

impl FsmColumn {
    fn new() -> Self {
        Self {
            transition: [0; FSM_COLUMN_SIZE],
        }
    }

    fn fill_range(&mut self, range: Range<char>, state: FsmIndex) {
        for i in range {
            self.transition[i as usize] = state;
        }
    }
}

fn main() {
    let mut fsm = Fsm::new();

    // FsmColumn 0
    {
        let mut col = FsmColumn::new();
        col.fill_range('a'..'b', 1);
        fsm.push(col);
    }
    fsm.dump();
}
