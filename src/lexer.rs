use crate::symbol;

use symbol::Symbol;

pub struct Lexer<'a> {
    head: u32,
    stream: &'a String,
    buffer: Option<Symbol>,
    pub parsed_stream: Vec<Symbol>
} 


impl <'a>Lexer<'a> {
    pub fn new (stream: &'a String) -> Self {
        Self {
            head: 0,
            stream,
            buffer: None,
            parsed_stream: Vec::<Symbol>::new(),
        }
    }

    pub fn reduce(&mut self, sym: Symbol) {
        let position = self.parsed_stream.len() - 2;
        let t1 = self.parsed_stream[position-2].clone();
        let t2 = self.parsed_stream[position-1].clone();
        let t3 = self.parsed_stream[position].clone();

        let result = match sym {
            Symbol::ADD => {
                if let Symbol::INT(op1) = t1 {
                    if let Symbol::INT(op2) = t3 {
                        op1+op2
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
            },
            Symbol::MULT => {
                if let Symbol::INT(op1) = t1 {
                    if let Symbol::INT(op2) = t3 {
                        op1*op2
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
            },
            Symbol::CLOSE_PAR => {
                if let Symbol::INT(op1) = t2 {
                    op1
                } else {
                    panic!();
                }
            },
            _ => panic!()
        };
        self.parsed_stream.drain(self.parsed_stream.len()-4..self.parsed_stream.len()-1);
        self.parsed_stream.insert(self.parsed_stream.len()-1,Symbol::INT(result));
    }

    pub fn consult(&mut self) -> Symbol {

        if self.buffer.is_some() {
            return self.buffer.clone().unwrap();
        }

        let c = self.stream.chars().nth(self.head as usize);

        if c.is_none() { 
            if *self.parsed_stream.last().clone().unwrap() != Symbol::END
            {self.parsed_stream.push(Symbol::END);}
            return Symbol::END;
        }

        let char = c.unwrap();

        let cur_sym: Symbol;

        cur_sym = match char {
         '(' => {
            self.head = self.head + 1;
            self.parsed_stream.push(Symbol::OPEN_PAR);
            Symbol::OPEN_PAR
         }   
         ')' => {
            self.head = self.head + 1;
            self.parsed_stream.push(Symbol::CLOSE_PAR);
            Symbol::CLOSE_PAR
         }
         '+' => {
            self.head = self.head + 1;
            self.parsed_stream.push(Symbol::ADD);
            Symbol::ADD
         }
         '-' => {
            self.head = self.head + 1;
            self.parsed_stream.push(Symbol::SUB);
            Symbol::SUB
         } 
         '*' => {
            self.head = self.head + 1;
            self.parsed_stream.push(Symbol::MULT);
            Symbol::MULT
         }

         mut c if c.is_numeric() => {
            let mut result = 0; 
            while c.is_numeric() {
                result = result *10 + c.to_digit(10).unwrap();
                self.head = self.head+1;
                c = self.stream.chars().nth(self.head as usize).unwrap_or('_');
            }
            self.parsed_stream.push(Symbol::INT(result));
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
