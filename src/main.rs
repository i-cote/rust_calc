mod state;
mod automata;
mod lexer;
mod symbol;
use automata::Automata;
use std::io::stdin;
fn main() {


    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
     input = input.trim().to_string();

    let mut automata = Automata::new(&input);

    let result = automata.compute();

    if result.is_some() {
        println!("{}",result.unwrap());
     } else {
        println!("Erroneous expression");
     }


}
