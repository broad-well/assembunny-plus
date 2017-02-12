use parser::{Token, TokenType};
use std::ops::Index;
use std::iter;
use std::iter::FromIterator;

/*
  This mod contains the interpreter part of Assembunny+. The abbreviated terminology for this mod is "ASMBI", for "ASseMBunny+ Interpreter".
  One of the functions here is meant to be called directly from main.rs so that file can focus on command line handling.
 */

pub struct AsmbiState {

    /// Register map (with its own type)
    pub regs: RegisterMap,

    /// Instruction Pointer, declared as u32 for ability to run more than 4 billion lines of ASMB.
    /// (I don't anticipate any combined ASMB program to have more than 4 billion lines!)
    pub ip: u32,

}

/// This struct/impl wraps the Register Vec in order to reduce boilerplate and redundancy on certain functions; It also makes code more readable.
pub struct RegisterMap {
    pub vec: Vec<i32>,
}
impl RegisterMap {
    pub fn index_set(&mut self, regindex: usize, val: i32) -> bool {
        if self.vec.len() <= regindex {
            return false;
        }
        self.vec[regindex] = val;
        true
    }

    pub fn set(&mut self, regtok: &Token, newval: i32) -> bool {
        self.index_set(regtok.val as usize, newval)
    }

    pub fn get(&self, index: usize) -> Option<&i32> {
        if self.vec.len() <= index {
            None
        } else {
            Some(self.vec.index(index))
        }
    }

    #[allow(unused_assignments)]
    pub fn index_modify<F>(&mut self, index: usize, modifier: F) -> bool
            where F: Fn(i32) -> i32 {
        let mut optval: i32 = 0;
        {
            match self.get(index) {
                Some(val) => optval = *val,
                None => return false
            }
        }
        self.index_set(index, modifier(optval))
    }

    pub fn modify<F>(&mut self, regtok: &Token, modifier: F) -> bool
            where F: Fn(i32) -> i32 {
        self.index_modify(regtok.val as usize, modifier)
    }

    pub fn parse_token(&self, tok: &Token) -> i32 {
        match tok.type_ {
            TokenType::LITERAL => tok.val,
            TokenType::REGISTER => *self.get(tok.val as usize).unwrap(),
            _ => panic!("parse_token does not parse keyword tokens.")
        }
    }

    pub fn new(capacity: usize) -> Self {
        RegisterMap {
            vec: Vec::from_iter(iter::repeat(0).take(capacity)),
        }
    }
}

/// Syntactic sugar for all return values in exec.
type Response = Result<(), String>;

/// Module consisting of executors for each keyword.
/// Each function has two arguments: mutable reference to AsmbiState and Vec<&str> tokens from the parser.
/// The tokens are expected to be passed by parser::line_valid. If an error that was supposed to be caught in that function is encountered here, the program will panic!, reminding the developer that parser::line_valid is not working properly.
mod exec {
    use std::char;
    use interpret::{AsmbiState, Response};
    use parser::Token;

    macro_rules! try_do {
        ( $fun:expr, $err:expr ) => (if $fun {
            Ok(())
        } else {
            Err($err)
        })
    }
    macro_rules! try_set {
        ( $fun:expr ) => (try_do!($fun, "Failed to set register value".to_owned()))
    }

    pub fn def(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
        // Syntax: def <new register index> <new value>
        // NOTE: All the separate `let` statements were adopted in order to prevent compiler errors regarding simultaneous mutable/immutable borrowing of state
        let newval = state.regs.parse_token(&toks[2]);
        try_set!(state.regs.set(&toks[1], newval))
    }

    pub fn inc(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
        // Syntax: inc <register index>
        try_set!(state.regs.modify(&toks[1], |v| v + 1))
    }

    pub fn inct(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
        // Syntax: inct <register index> <value to add>
        let adder = state.regs.parse_token(&toks[2]);
        try_set!(state.regs.modify(&toks[1], |v| v + adder))
    }

    pub fn dec(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
        // Syntax: dec <register name>
        try_set!(state.regs.modify(&toks[1], |v| v - 1))
    }

    pub fn dect(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
        // Syntax: dect <register name> <value to be eval'd>
        let subtractor = state.regs.parse_token(&toks[2]);
        try_set!(state.regs.modify(&toks[1], |v| v - subtractor))
    }

    pub fn mul(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
        // Syntax: mul <register name> <eval-ue>
        let multiplier = state.regs.parse_token(&toks[2]);
        try_set!(state.regs.modify(&toks[1], |v| v * multiplier))
    }

    pub fn div(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
        // Syntax: div <register name> <eval-ue>
        // Note: floor the result
        let quotient = state.regs.parse_token(&toks[2]);
        try_set!(state.regs.modify(&toks[1], |v| v / quotient))
    }

    pub fn cpy(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
        // Syntax: cpy <eval-ue> <register name>
        let newval = state.regs.parse_token(&toks[1]);
        try_set!(state.regs.set(&toks[2], newval))
    }

    pub fn jnz(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
        // Syntax: cpy <eval-ue> <literal>
        // Since IP is incremented after each line, go to relative line **minus 1** so the program works properly.
        if state.regs.parse_token(&toks[1]) != 0 {
            // TODO: add under/overflow checks
            // Ugly hack for u32 adding i32; hope this will be supported in future versions of Rust.
            let diff = state.regs.parse_token(&toks[2]) - 1;
            if diff < 0 {
                state.ip -= (-diff) as u32
            } else {
                state.ip += diff as u32
            }
        }
        Ok(())
    }

    pub fn out(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
        // Syntax: out <eval-ue>
        print!("{} ", state.regs.parse_token(&toks[1]));
        Ok(())
    }

    pub fn outn(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
        // Syntax: outn <eval-ue>
        println!("{}", state.regs.parse_token(&toks[1]));
        Ok(())
    }

    pub fn outc(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
        // Syntax: outc <eval-ue>
        let val = state.regs.parse_token(&toks[1]);
        if val < 0 {
            return Err(format!("Char code ({}) should not be less than zero", val));
        }
        match char::from_u32(val as u32) {
            Some(v) => print!("{}", v),
            _ => return Err(format!("Char code ({}) is invalid", val))
        }
        Ok(())
    }

    pub const INDEX: [fn(&mut AsmbiState, &Vec<Token>) -> Response; 12] = [def, inc, inct, dec, dect, mul, div, cpy, jnz, out, outn, outc];
}

pub fn execute(state: &mut AsmbiState, toks: &Vec<Token>) -> Response {
    assert_eq!(toks[0].type_, TokenType::KEYWORD);
    exec::INDEX[toks[0].val as usize](state, toks)
}

pub fn new_state(capacity: usize) -> AsmbiState {
    AsmbiState {
        regs: RegisterMap::new(capacity),
        ip: 0
    }
}
