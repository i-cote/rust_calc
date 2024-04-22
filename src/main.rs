mod state;
mod automata;
mod lexer;
mod symbol;
use automata::Automata;
use state::TransitionResult;
use std::io::stdin;
fn main() {


    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
     input = input.trim().to_string();

    let mut automata = Automata::new(&input);

    while automata.step() ==  TransitionResult::PROCEED {}

    if automata.step() == TransitionResult::END {
        println!("The automata executed successfully");
    } else {
        println!("The automata encountered an error");
    }
}
