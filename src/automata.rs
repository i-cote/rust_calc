use crate::state::State;
use crate::lexer::Lexer;
use State::*;


use crate::state::TransitionResult;

pub struct Automata<'a> {
    pub state_stack: Vec<State>,
    lexer: Lexer<'a> 
}

impl <'a>Automata<'a> {

    pub fn new(str: &'a String) -> Self {
        let mut _self = Self {
            state_stack: Vec::<State>::new(),
            lexer: Lexer::new(str)
        };
        _self.state_stack.push(STATE0);
        _self
    }

    pub fn step(&mut self) -> TransitionResult {
        self.state_stack.last().unwrap().clone().transition(self)
    }
}