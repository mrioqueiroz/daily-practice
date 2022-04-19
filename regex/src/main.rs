const FSM_COLUMN_SIZE: usize = 130;
const FSM_NEW_LINE: usize = FSM_COLUMN_SIZE - 1;

struct Regex {
    columns: Vec<FsmColumn>,
}

impl Regex {
    fn compile(src: &str) -> Self {
        let mut fsm = Self {
            columns: Vec::new(),
        };
        fsm.push(FsmColumn::new());

        for c in src.chars() {
            let mut col = FsmColumn::new();
            match c {
                '$' => {
                    col.transition[FSM_NEW_LINE] = fsm.columns.len() + 1;
                }
                '.' => {
                    for i in 32..127 {
                        col.transition[i] = fsm.columns.len() + 1;
                    }
                }
                _ => {
                    col.transition[c as usize] = fsm.columns.len() + 1;
                }
            }
            fsm.columns.push(col)
        }
        fsm
    }

    fn match_str(&self, input: &str) -> bool {
        // Successful state.
        let mut state = 1;
        for c in input.chars() {
            if state == 0 || state >= self.columns.len() {
                break;
            }
            state = self.columns[state].transition[c as usize];
        }
        if state == 0 {
            return false;
        }
        if state < self.columns.len() {
            state = self.columns[state].transition[FSM_NEW_LINE]
        }
        state >= self.columns.len()
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
}

fn main() {
    let regex = Regex::compile(".bc$");

    regex.dump();

    let inputs = vec!["Hello, World", "abc", "bbc", "cbc", "cbd", "cbd", "abcd"];
    for input in inputs.iter() {
        println!("{:?} => {:?}", input, regex.match_str(input));
    }
}
