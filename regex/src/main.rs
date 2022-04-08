use std::ops::Range;

const FSM_COLUMN_SIZE: usize = 130;
const FSM_NEW_LINE: usize = FSM_COLUMN_SIZE - 1;

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
            print!("{:03} => ", row);
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

    let events = vec!['a' as usize, 'b' as usize, 'c' as usize, FSM_NEW_LINE];

    // Failed state
    fsm.push(FsmColumn::new());

    for event in events {
        let mut col = FsmColumn::new();
        // Keep track of the state
        col.transition[event] = fsm.columns.len() + 1;
        fsm.push(col);
    }

    fsm.dump();
}
