#![allow(non_snake_case)]
use crate::automata::Automata;


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
    println!("In state0 going to state1");
    automata.state_stack.push(STATE1);
    return PROCEED;

}

fn transitSTATE1(automata: &mut Automata) -> TransitionResult {
    println!("In state1 going to state2");
    automata.state_stack.push(STATE2);
    return PROCEED;

}

fn transitSTATE2(automata: &mut Automata) -> TransitionResult {
    println!("In state2 going to state3");
    automata.state_stack.push(STATE3);
    return PROCEED;

}

fn transitSTATE3(automata: &mut Automata) -> TransitionResult {
    println!("In state3 going to state4");
    automata.state_stack.push(STATE4);
    return PROCEED;

}

fn transitSTATE4(automata: &mut Automata) -> TransitionResult {
    println!("In state4 going to state5");
    automata.state_stack.push(STATE5);
    return PROCEED;

}

fn transitSTATE5(automata: &mut Automata) -> TransitionResult {
    println!("In state5 going to state6");
    automata.state_stack.push(STATE6);
    return PROCEED;

}

fn transitSTATE6(automata: &mut Automata) -> TransitionResult {
    println!("In state6 going to state7");
    automata.state_stack.push(STATE7);
    return PROCEED;

}

fn transitSTATE7(automata: &mut Automata) -> TransitionResult {
    println!("In state7 going to state8");
    automata.state_stack.push(STATE8);
    return PROCEED;

}

fn transitSTATE8(automata: &mut Automata) -> TransitionResult {
    println!("In state8 going to state9");
    automata.state_stack.push(STATE9);
    return PROCEED;

}

fn transitSTATE9(automata: &mut Automata) -> TransitionResult {
    println!("In state9 final state");
    return END;

}