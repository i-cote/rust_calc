use crate::state::State;
use State::*;

use crate::state::TransitionResult;

pub struct Automata {
    pub state_stack: Vec<State>,
}

impl Automata {

    pub fn new() -> Self {
        let mut _self = Self {
            state_stack: Vec::<State>::new(),
        };
        _self.state_stack.push(STATE0);
        _self
    }

    pub fn step(&mut self) -> TransitionResult {
        self.state_stack.last().unwrap().clone().transition(self)
    }
}