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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        let (expr, expected) = (String::from("1+22*4*4+(1+2)*3"), 1+22*4*4+(1+2)*3);
        let mut automata = Automata::new(&expr);
        let result = automata.compute();
        assert_eq!(result.unwrap(), expected);
    }


    #[test]
    fn test_compute1() {
        let (expr, expected) = 
        
            (String::from("5+12+4*3+2*3"), 5+12+4*3+2*3)
        ; 
        let mut automata = Automata::new(&expr);
        let result = automata.compute();
        assert_eq!(result.unwrap(), expected);
    }


    #[test]
    fn test_compute2() {
        let (expr, expected) = 
        
            (String::from("3*3+1*(10*2*3)"), 3*3+1*(10*2*3))
        ; 
        let mut automata = Automata::new(&expr);
        let result = automata.compute();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_compute3() {
        let (expr, expected) = 
        
            (String::from("((15*4))+3*(22)"), ((15)*4)+3*(22))
        ; 
        let mut automata = Automata::new(&expr);
        let result = automata.compute();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_compute4() {
        let (expr, expected) = 
        
            (String::from("100*(5*2)+3"), 100*(5*2)+3)
        ; 
        let mut automata = Automata::new(&expr);
        let result = automata.compute();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_compute5() {
        let (expr, expected) = 
        
            (String::from("2*(3+5*2)+4"), 2*(3+5*2)+4)
        ; 
        let mut automata = Automata::new(&expr);
        let result = automata.compute();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_compute6() {
        let (expr, expected) = 
        
            (String::from("(4*4+3*3)*5"), (4*4+3*3)*5)
        ; 
        let mut automata = Automata::new(&expr);
        let result = automata.compute();
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_compute7() {
        let (expr, expected) = 
        
            (String::from("7*5+20+(4+1)"), 7*5+20+(4+1))
        ; 
        let mut automata = Automata::new(&expr);
        let result = automata.compute();
        assert_eq!(result.unwrap(), expected);
    }
    
}
