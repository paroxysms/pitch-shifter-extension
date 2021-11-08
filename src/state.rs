use std::sync::{MutexGuard, Mutex};
use lazy_static::lazy_static;

pub struct State {
    pub pitch_change: i32,
    pub id: String,
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State {
        pitch_change: 80,
        id: "".to_string(),
    });
}

impl State {
    pub fn get<'a>() -> MutexGuard<'a, Self> {
        STATE.lock().unwrap()
    }
}