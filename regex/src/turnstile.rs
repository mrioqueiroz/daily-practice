use std::io::{self, BufRead, Write};

const LOCKED: usize = 0;
const UNLOCKED: usize = 1;
const STATES_COUNT: usize = 2;

const PUSH: usize = 0;
const COIN: usize = 1;
const EVENTS_COUNT: usize = 2;

const FSM: [[usize; EVENTS_COUNT]; STATES_COUNT] = [[LOCKED, UNLOCKED], [LOCKED, UNLOCKED]];

fn next_state(state: usize, event: usize) -> usize {
    FSM[state][event]
}

#[test]
fn on_push_lock_if_locked() {
    assert_eq!(next_state(LOCKED, PUSH), LOCKED);
}

#[test]
fn on_push_lock_if_unlocked() {
    assert_eq!(next_state(UNLOCKED, PUSH), LOCKED);
}

#[test]
fn on_coin_unlock_if_locked() {
    assert_eq!(next_state(LOCKED, COIN), UNLOCKED);
}

#[test]
fn on_coin_unlock_if_unlocked() {
    assert_eq!(next_state(UNLOCKED, COIN), UNLOCKED);
}

fn prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn state_to_str(state: usize) -> &'static str {
    match state {
        LOCKED => "locked",
        UNLOCKED => "unlocked",
        _ => "unknown",
    }
}

fn main() {
    let mut state = LOCKED;
    prompt();
    for line in io::stdin().lock().lines() {
        match line.unwrap().as_str() {
            "c" | "coin" => state = next_state(state, COIN),
            "p" | "push" => state = next_state(state, PUSH),
            "q" | "quit" => return,
            unknown => {
                eprintln!("unknown event {}", unknown);
            }
        }
        println!("{}", state_to_str(state));
        prompt();
    }
}
