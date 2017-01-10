use std::collections::HashMap;
use parser;

/*
  This mod contains the interpreter part of Assembunny+. The abbreviated terminology for this mod is "ASMBI", for "ASseMBunny+ Interpreter".
  One of the functions here is meant to be called directly from main.rs so that file can focus on command line handling.
 */

pub struct AsmbiState {

    /// Register HashMap
    regs: HashMap<&'static str, i32>,

    /// Instruction Pointer, declared as u32 for ability to run more than 4 billion lines of ASMB.
    /// (I don't anticipate any combined ASMB program to have more than 4 billion lines!)
    ip: u32,

}

/// Syntactic sugar for all return values in exec.
type Response = Result<(), String>;

/// Module consisting of executors for each keyword.
/// Each function has two arguments: mutable reference to AsmbiState and Vec<&str> tokens from the parser.
/// The tokens are expected to be passed by parser::line_valid. If an error that was supposed to be caught in that function is encountered here, the program will panic!, reminding the developer that parser::line_valid is not working properly.
mod exec {
    use interpret::{AsmbiState, Response};
    use parser;

    // Using macros here because it can dictate parent function returns
    macro_rules! getmut_reg_by_name {
        ( $name:expr, $state:expr ) => (match $state.regs.get_mut($name) {
            Some(val) => val,
            _ => return Err(format!("Register '{}' does not exist", $name))
        })
    }
    macro_rules! get_reg_by_name {
        ( $name:expr, $state:expr ) => (match $state.regs.get($name) {
            Some(val) => val,
            _ => return Err(format!("Register '{}' does not exist", $name))
        })
    }
    macro_rules! unexpect_regname_exists {
        ( $name:expr, $state:expr ) => (if $state.regs.contains_key($name) {
            return Err(format!("Register '{}' already exists", $name));
        })
    }
    macro_rules! try_eval {
        ( $val:expr, $state:expr ) => (match parser::evaluate_val($val, &$state.regs) {
            Err(why) => return Err(format!("Parameter evaluation failed: {}", why)),
            Ok(val) => val
        })
    }


    pub fn def(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: def <new register name> <evaluate_val candidate>
        unexpect_regname_exists!(toks[1], state);
        state.regs.insert(toks[1], try_eval!(toks[2], state));
        Ok()
    }

    pub fn inc(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: inc <register name>
        getmut_reg_by_name!(toks[1], state) += 1;
        Ok()
    }

    pub fn inct(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: inct <register name> <evaluate_val candidate>
        getmut_reg_by_name!(toks[1], state) += try_eval!(toks[2], state);
        Ok()
    }

    pub fn dec(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: dec <register name>
        getmut_reg_by_name!(toks[1], state) -= 1;
        Ok()
    }

    pub fn dect(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: dect <register name> <value to be eval'd>
        getmut_reg_by_name!(toks[1], state) -= try_eval!(toks[2], state);
        Ok()
    }

    pub fn mul(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: mul <register name> <eval-ue>
        getmut_reg_by_name!(toks[1], state) *= try_eval!(toks[2], state);
        Ok()
    }

    pub fn div(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: div <register name> <eval-ue>
        // Note: floor the result
        getmut_reg_by_name!(toks[1], state) /= try_eval!(toks[2], state);
        Ok()
    }

    pub fn cpy(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: cpy <eval-ue> <register name>
        getmut_reg_by_name!(toks[2], state) = try_eval!(toks[1], state);
        Ok()
    }

    pub fn jnz(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: cpy <eval-ue> <eval-ue>
        // Since IP is incremented after each line, go to relative line **minus 1** so the program works properly.
        if try_eval!(toks[1], state) != 0 {
            // TODO: add under/overflow checks
            state.ip += try_eval!(toks[2], state) - 1;
        }
        Ok()
    }

    pub fn out(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: out <eval-ue>
        println!("{} ", try_eval!(toks[1], state));
        Ok()
    }
}

pub fn execute(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
    // Redundancy can be solved with anonymous closures in HashMaps
    match toks[0].to_lowercase() {
        "def" => exec::def(state, toks),
        "inc" => exec::inc(state, toks),
        "inct" => exec::inct(state, toks),
        "dec" => exec::dec(state, toks),
        "dect" => exec::dect(state, toks),
        "mul" => exec::mul(state, toks),
        "div" => exec::div(state, toks),
        "cpy" => exec::cpy(state, toks),
        "jnz" => exec::jnz(state, toks),
        "out" => exec::out(state, toks)
    }
}

pub fn new_state() -> AsmbiState {
    AsmbiState {
        regs: HashMap::new(),
        ip: 0
    }
}