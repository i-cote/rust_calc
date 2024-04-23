#![allow(non_camel_case_types)]


use std::fmt;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Symbol {
    OPEN_PAR,
    CLOSE_PAR,
    INT(u32),
    MULT,
    ADD,
    SUB,
    ERROR,
    END
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
        Symbol::OPEN_PAR => write!(f, "(")?,
        Symbol::CLOSE_PAR => write!(f, ")")?,
        Symbol::INT(a) => write!(f, "{}", a)?,
        Symbol::MULT => write!(f, "*")?,
        Symbol::ADD => write!(f, "+")?,
        Symbol::SUB => write!(f, "-")?,
        Symbol::ERROR => write!(f, "ERR")?,
        Symbol::END => write!(f, "$")?,
    }

    Ok(())
    } 
}