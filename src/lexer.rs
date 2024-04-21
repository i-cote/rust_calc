use crate::symbol;

use symbol::Symbol;

pub struct Lexer<'a> {
    head: u32,
    stream: &'a String,
    buffer: Option<Symbol>,
} 

impl <'a>Lexer<'a> {
    pub fn new (stream: &'a String) -> Self {
        Self {
            head: 0,
            stream,
            buffer: None,
        }
    }

    pub fn consult(&mut self) -> Symbol {

        if self.buffer.is_some() {
            return self.buffer.clone().unwrap();
        }

        let c = self.stream.chars().nth(self.head as usize);

        if c.is_none() { 
            return Symbol::END;
        }

        let char = c.unwrap();

        let cur_sym: Symbol;

        cur_sym = match char {
         '(' => {
            self.head = self.head + 1;
            Symbol::OPEN_PAR
         }   
         ')' => {
            self.head = self.head + 1;
            Symbol::CLOSE_PAR
         }
         '+' => {
            self.head = self.head + 1;
            Symbol::ADD
         }
         '-' => {
            self.head = self.head + 1;
            Symbol::SUB
         } 
         '*' => {
            self.head = self.head + 1;
            Symbol::MULT
         }

         mut c if c.is_numeric() => {
            let mut result = 0; 
            while c.is_numeric() {
                result = result *10 + c.to_digit(10).unwrap();
                self.head = self.head+1;
                c = self.stream.chars().nth(self.head as usize).unwrap_or('_');
            }
            Symbol::INT(result)
         }
         _ => {
            Symbol::ERROR
         }

        };
         self.buffer = Some(cur_sym.clone());
         return cur_sym;
    }

    pub fn proceed(self: &mut Self) {
        self.buffer = None;
    }

}
