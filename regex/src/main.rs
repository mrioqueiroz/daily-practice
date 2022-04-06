#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum State {
    Locked,
    Unlocked,
}

#[allow(dead_code)]
enum Event {
    Push,
    Coin,
}

#[allow(dead_code)]
fn next_state(state: State, event: Event) -> State {
    match state {
        State::Locked => match event {
            Event::Push => State::Locked,
            Event::Coin => State::Unlocked,
        },
        State::Unlocked => match event {
            Event::Push => State::Locked,
            Event::Coin => State::Unlocked,
        },
    }
}

#[test]
fn on_push_if_locked_stay_locked() {
    assert_eq!(next_state(State::Locked, Event::Push), State::Locked);
}

#[test]
fn on_push_if_unlocked_move_to_locked() {
    assert_eq!(next_state(State::Unlocked, Event::Push), State::Locked);
}

#[test]
fn on_coin_if_locked_move_to_unlocked() {
    assert_eq!(next_state(State::Locked, Event::Coin), State::Unlocked);
}

#[test]
fn on_coin_if_unlocked_stay_unlocked() {
    assert_eq!(next_state(State::Unlocked, Event::Coin), State::Unlocked);
}

fn main() {}
