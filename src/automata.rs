use crate::state::State;
use crate::lexer::Lexer;
use crate::symbol::Symbol;
use State::*;


use crate::state::TransitionResult;
use TransitionResult::*;

pub struct Automata<'a> {
    pub state_stack: Vec<State>,
    pub lexer: Lexer<'a>,
    pub is_non_t: bool,
}

impl <'a>Automata<'a> {

    pub fn new(str: &'a String) -> Self {
        let mut _self = Self {
            state_stack: Vec::<State>::new(),
            lexer: Lexer::new(str),
            is_non_t: false,
        };
        _self.state_stack.push(STATE0);
        _self
    }

    pub fn step(&mut self) -> TransitionResult {
        self.state_stack.last().unwrap().clone().transition(self)
    }

    pub fn compute(&mut self) -> Option<u32> {
        while self.step() == RUNNING {}

        if self.step() == ACCEPT {
            if let Symbol::INT(result) = self.lexer.parsed_stream[0] {
                return Some(result);
            }
            else {
                return None;
            }
        } else {
            return None
        }
    }
}