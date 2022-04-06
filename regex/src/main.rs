use std::io::{self, BufRead, Write};

#[derive(Debug, PartialEq)]
enum State {
    Locked,
    Unlocked,
}

enum Event {
    Push,
    Coin,
}

fn next_state(event: Event) -> State {
    match event {
        Event::Push => State::Locked,
        Event::Coin => State::Unlocked,
    }
}

#[test]
fn on_push_lock() {
    assert_eq!(next_state(Event::Push), State::Locked);
}

#[test]
fn on_coin_unlock() {
    assert_eq!(next_state(Event::Coin), State::Unlocked);
}

fn prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut state = State::Locked;
    prompt();
    for line in io::stdin().lock().lines() {
        match line.unwrap().as_str() {
            "coin" => state = next_state(Event::Coin),
            "push" => state = next_state(Event::Push),
            unknown => {
                eprintln!("unknown event {}", unknown);
            }
        }
        println!("{:?}", state);
        prompt();
    }
}
