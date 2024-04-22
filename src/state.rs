#![allow(non_snake_case)]
use crate::automata::Automata;

use crate::symbol::Symbol;


use State::*;
use TransitionResult::*;

#[derive(PartialEq)]
pub enum TransitionResult {
    PROCEED,
    ERROR,
    END,
}

#[derive(Clone)]
pub enum State {
    STATE0,
    STATE1,
    STATE2,
    STATE3,
    STATE4,
    STATE5,
    STATE6,
    STATE7,
    STATE8,
    STATE9,
}

impl State {
    pub fn transition(&self, automata: &mut Automata) -> TransitionResult {
        match self {
            STATE0 => transitSTATE0(automata),
            STATE1 => transitSTATE1(automata),
            STATE2 => transitSTATE2(automata),
            STATE3 => transitSTATE3(automata),
            STATE4 => transitSTATE4(automata),
            STATE5 => transitSTATE5(automata),
            STATE6 => transitSTATE6(automata),
            STATE7 => transitSTATE7(automata),
            STATE8 => transitSTATE8(automata),
            STATE9 => transitSTATE9(automata),
        }
    }
}


fn transitSTATE0(automata: &mut Automata) -> TransitionResult {
    if automata.is_non_t {
        automata.state_stack.push(STATE1);
        automata.is_non_t = false;
        return PROCEED;
    }
    let sym = automata.lexer.consult();
    match sym {
        Symbol::INT(_) => {
            automata.state_stack.push(STATE3);
            automata.lexer.proceed();
            PROCEED
        },
        Symbol::OPEN_PAR => {
            automata.state_stack.push(STATE2);
            automata.lexer.proceed();
            PROCEED
        },
        _ => {
            ERROR
        }
    }
}

fn transitSTATE1(automata: &mut Automata) -> TransitionResult {
    let sym = automata.lexer.consult();
    match sym {
        Symbol::ADD => {
            automata.state_stack.push(STATE4);
            automata.lexer.proceed();
            PROCEED
        },
        Symbol::MULT => {
            automata.state_stack.push(STATE5);
            automata.lexer.proceed();
            PROCEED
        },
        Symbol::END => {
            END
        }
        _ => {
            ERROR
        }
    }
}


fn transitSTATE2(automata: &mut Automata) -> TransitionResult {
    if automata.is_non_t {
        automata.state_stack.push(STATE6);
        automata.is_non_t = false;
        return PROCEED;
    }
    let sym = automata.lexer.consult();
    match sym {
        Symbol::INT(_) => {
            automata.state_stack.push(STATE3);
            automata.lexer.proceed();
            PROCEED
        },
        Symbol::OPEN_PAR => {
            automata.state_stack.push(STATE2);
            automata.lexer.proceed();
            PROCEED
        },
        _ => {
            ERROR
        }
    }

}


fn transitSTATE3(automata: &mut Automata) -> TransitionResult {
    let sym = automata.lexer.consult();
    match sym {
       Symbol::ADD | Symbol::MULT | Symbol::CLOSE_PAR | Symbol::END => {
        automata.state_stack.pop();
        automata.is_non_t = true;
        PROCEED
       },
       _ => {ERROR} 
    }
}

fn transitSTATE4(automata: &mut Automata) -> TransitionResult {
    if automata.is_non_t {
        automata.state_stack.push(STATE7);
        automata.is_non_t = false;
        return PROCEED;
    }
    let sym = automata.lexer.consult();
    match sym {
        Symbol::INT(_) => {
            automata.state_stack.push(STATE3);
            automata.lexer.proceed();
            PROCEED
        },
        Symbol::OPEN_PAR => {
            automata.state_stack.push(STATE2);
            automata.lexer.proceed();
            PROCEED
        },
        _ => {
            ERROR
        }
    }
}


fn transitSTATE5(automata: &mut Automata) -> TransitionResult {
    if automata.is_non_t {
        automata.state_stack.push(STATE8);
        automata.is_non_t = false;
        return PROCEED;
    }
    let sym = automata.lexer.consult();
    match sym {
        Symbol::INT(_) => {
            automata.state_stack.push(STATE3);
            automata.lexer.proceed();
            PROCEED
        },
        Symbol::OPEN_PAR => {
            automata.state_stack.push(STATE2);
            automata.lexer.proceed();
            PROCEED
        },
        _ => {
            ERROR
        }
    }
}

fn transitSTATE6(automata: &mut Automata) -> TransitionResult {
    let sym = automata.lexer.consult();
    match sym {
        Symbol::ADD => {
            automata.state_stack.push(STATE4);
            automata.lexer.proceed();
            PROCEED
        },
        Symbol::MULT => {
            automata.state_stack.push(STATE5);
            automata.lexer.proceed();
            PROCEED
        },
        Symbol::CLOSE_PAR => {
            automata.state_stack.push(STATE9);
            automata.lexer.proceed();
            PROCEED
        },
        _ => {
            ERROR
        }
    }
}

fn transitSTATE7(automata: &mut Automata) -> TransitionResult {
    let sym = automata.lexer.consult();
    match sym {
        Symbol::MULT => {
            automata.state_stack.push(STATE5);
            automata.lexer.proceed();
            PROCEED
        },
        Symbol::ADD | Symbol::CLOSE_PAR | Symbol::END => {
            automata.state_stack.pop();
            automata.state_stack.pop();
            automata.state_stack.pop();
            automata.is_non_t = true;
            PROCEED
        },
       _ => {
            ERROR
        }, 
    }
}

fn transitSTATE8(automata: &mut Automata) -> TransitionResult {
    let sym = automata.lexer.consult();
    match sym {
        Symbol::ADD | Symbol::MULT | Symbol::CLOSE_PAR | Symbol::END => {
            automata.state_stack.pop();
            automata.state_stack.pop();
            automata.state_stack.pop();
            automata.is_non_t = true;
            PROCEED
        },
       _ => {
            ERROR
        }, 
    }
}

fn transitSTATE9(automata: &mut Automata) -> TransitionResult {
    let sym = automata.lexer.consult();
    match sym {
        Symbol::ADD | Symbol::MULT | Symbol::CLOSE_PAR | Symbol::END => {
            automata.state_stack.pop();
            automata.state_stack.pop();
            automata.state_stack.pop();
            automata.is_non_t = true;
            PROCEED
        },
       _ => {
            ERROR
        }, 
    }
}