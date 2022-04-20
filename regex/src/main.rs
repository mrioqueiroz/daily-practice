const FSM_COLUMN_SIZE: usize = 130;
const FSM_NEW_LINE: usize = FSM_COLUMN_SIZE - 1;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
struct FsmAction {
    next: FsmIndex,
    // Tell if will go to the right or not.
    offset: i32,
}

// impl FsmAction {
//     fn default() -> Self {
//         Self { next: 0, offset: 0 }
//     }
// }

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
                    col.transition[FSM_NEW_LINE] = FsmAction {
                        next: fsm.columns.len() + 1,
                        offset: 1,
                    };
                    fsm.columns.push(col);
                }
                '.' => {
                    for i in 32..127 {
                        col.transition[i] = FsmAction {
                            next: fsm.columns.len() + 1,
                            offset: 1,
                        }
                    }
                    fsm.columns.push(col);
                }
                // Modifies the previous state.
                '*' => {
                    let n = fsm.columns.len();
                    for t in fsm.columns.last_mut().unwrap().transition.iter_mut() {
                        if t.next == n {
                            // Redirects to itself.
                            t.next = n - 1;
                        } else if t.next == 0 {
                            t.next = n;
                            t.offset = 0;
                        } else {
                            unreachable!();
                        }
                    }
                }
                _ => {
                    col.transition[c as usize] = FsmAction {
                        next: fsm.columns.len() + 1,
                        offset: 1,
                    };
                    fsm.columns.push(col);
                }
            }
        }
        fsm
    }

    fn match_str(&self, input: &str) -> bool {
        // Successful state.
        let mut state = 1;
        let mut head = 0;
        let chars = input.chars().collect::<Vec<_>>();

        while 0 < state && state < self.columns.len() {
            let action = self.columns[state].transition[chars[head] as usize];
            state = action.next;
            head = (head as i32 + action.offset) as usize;
        }
        // for c in input.chars() {
        //     if state == 0 || state >= self.columns.len() {
        //         break;
        //     }
        //     state = self.columns[state].transition[c as usize];
        // }
        if state == 0 {
            return false;
        }
        if state < self.columns.len() {
            let action = self.columns[state].transition[FSM_NEW_LINE];
            state = action.next;
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
                print!(
                    "({}, {}) ",
                    col.transition[row].next, col.transition[row].offset
                );
            }
            println!();
        }
    }
}

type FsmIndex = usize;

#[derive(Copy, Clone, Debug)]
struct FsmColumn {
    transition: [FsmAction; FSM_COLUMN_SIZE],
}

impl FsmColumn {
    fn new() -> Self {
        Self {
            transition: [Default::default(); FSM_COLUMN_SIZE],
        }
    }
}

fn main() {
    let regex = Regex::compile("a*bc");

    regex.dump();

    let inputs = vec![
        "Hello, World",
        "abc",
        "bc",
        "bbc",
        "cbc",
        "cbd",
        "cbd",
        "abcd",
        "aaabc",
    ];
    for input in inputs.iter() {
        println!("{:?} => {:?}", input, regex.match_str(input));
    }
}
