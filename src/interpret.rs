use std::collections::HashMap;
use parser;

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

/// This struct/impl wraps the Register HashMap in order to reduce boilerplate and redundancy on certain functions; It also makes code more readable.
pub struct RegisterMap {
    pub map: HashMap<String, i32>,
}
impl RegisterMap {
    pub fn set(&mut self, reg_name: &str, val: i32) -> bool {
        if !self.map.contains_key(reg_name) {
            return false;
        }
        self.map.insert(reg_name.to_owned(), val);
        true
    }

    pub fn get(&self, name: &str) -> Option<&i32> {
        self.map.get(name)
    }

    pub fn add(&mut self, reg_name: &str, val: i32) -> bool {
        if self.map.contains_key(reg_name) {
            return false;
        }
        self.map.insert(reg_name.to_owned(), val);
        true
    }

    pub fn modify<F>(&mut self, name: &str, modifier: F) -> bool
            where F: Fn(i32) -> i32 {
		let mut optval: i32 = 0;
		{
			match self.get(name) {
				Some(val) => optval = *val,
				None => return false
			}

		}
		self.set(name, modifier(optval))
    }

    pub fn new() -> Self {
        RegisterMap {
            map: HashMap::new(),
        }
    }
}

/// Syntactic sugar for all return values in exec.
type Response = Result<(), String>;

/// Module consisting of executors for each keyword.
/// Each function has two arguments: mutable reference to AsmbiState and Vec<&str> tokens from the parser.
/// The tokens are expected to be passed by parser::line_valid. If an error that was supposed to be caught in that function is encountered here, the program will panic!, reminding the developer that parser::line_valid is not working properly.
mod exec {
    use interpret::{AsmbiState, Response};
    use parser;

    macro_rules! try_eval {
        ( $val:expr, $state:expr ) => (match parser::evaluate_val($val, &$state.regs.map) {
            Err(why) => return Err(format!("Parameter evaluation failed: {}", why)),
            Ok(val) => val
        })
    }
    macro_rules! try_do {
        ( $fun:expr, $err:expr ) => (if $fun {
            Ok(())
        } else {
            Err($err)
        })
    }
    macro_rules! err_nonexist {
        ( $reg:expr ) => (format!("Register by name '{}' does not exist", $reg))
    }

    pub fn def(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: def <new register name> <evaluate_val candidate>
		let val = try_eval!(toks[2], state);
        try_do!(
            state.regs.add(toks[1], val),
            format!("Register by name '{}' already exists", toks[1]))
    }

    pub fn inc(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: inc <register name>
        try_do!(
            state.regs.modify(toks[1], |v| v+1),
            err_nonexist!(toks[1]))
    }

    pub fn inct(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: inct <register name> <evaluate_val candidate>
		let add = try_eval!(toks[2], state);
        try_do!(
            state.regs.modify(toks[1], |v| v+add),
            err_nonexist!(toks[1]))
    }

    pub fn dec(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: dec <register name>
        try_do!(
            state.regs.modify(toks[1], |v| v-1),
            err_nonexist!(toks[1]))
    }

    pub fn dect(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: dect <register name> <value to be eval'd>
		let subt = try_eval!(toks[2], state);
        try_do!(
            state.regs.modify(toks[1], |v| v - subt),
            err_nonexist!(toks[1]))
    }

    pub fn mul(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: mul <register name> <eval-ue>
		let multiplier = try_eval!(toks[2], state);
        try_do!(
            state.regs.modify(toks[1], |v| v * multiplier),
            err_nonexist!(toks[1]))
    }

    pub fn div(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: div <register name> <eval-ue>
        // Note: floor the result
		let divisor = try_eval!(toks[2], state);
        try_do!(
            state.regs.modify(toks[1], |v| v / divisor),
            err_nonexist!(toks[1]))
    }

    pub fn cpy(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: cpy <eval-ue> <register name>
		let val: i32 = try_eval!(toks[1], state);
        try_do!(
            state.regs.set(toks[2], val),
            format!("Register by the name of '{}' does not exist. Perhaps use DEF instead?", toks[2]))
    }

    pub fn jnz(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: cpy <eval-ue> <eval-ue>
        // Since IP is incremented after each line, go to relative line **minus 1** so the program works properly.
        if try_eval!(toks[1], state) != 0 {
            // TODO: add under/overflow checks
            state.ip += (try_eval!(toks[2], state) - 1) as u32;
        }
        Ok(())
    }

    pub fn out(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: out <eval-ue>
        println!("{} ", try_eval!(toks[1], state));
        Ok(())
    }
}

pub fn execute(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
    // Redundancy can be solved with anonymous closures in HashMaps
	if let Err(err) = parser::line_valid(&toks) {
		return Err(format!("Invalid: {}", err));
	}
    match toks[0].to_lowercase().as_str() {
        "def" => exec::def(state, toks),
        "inc" => exec::inc(state, toks),
        "inct" => exec::inct(state, toks),
        "dec" => exec::dec(state, toks),
        "dect" => exec::dect(state, toks),
        "mul" => exec::mul(state, toks),
        "div" => exec::div(state, toks),
        "cpy" => exec::cpy(state, toks),
        "jnz" => exec::jnz(state, toks),
        "out" => exec::out(state, toks),
		_ => Err(format!("Unknown keyword: {}", toks[0]))
    }
}

pub fn new_state() -> AsmbiState {
    AsmbiState {
        regs: RegisterMap::new(),
        ip: 0
    }
}
