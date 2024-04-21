#![allow(non_camel_case_types)]

#[derive(Clone)]
#[derive(Debug)]
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
