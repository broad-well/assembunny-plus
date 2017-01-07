mod parser;
mod macros;
use std::collections::HashMap;

/*
  This mod contains the interpreter part of Assembunny+. The abbreviated terminology for this mod is "ASMBI", for "ASseMBunny+ Interpreter".
  One of the functions here is meant to be called directly from main.rs so that file can focus on command line handling.
 */

struct AsmbiState {

    /// Register HashMap
    regs: HashMap<&str, i32>,

    /// Instruction Pointer, declared as u32 for ability to run more than 4 billion lines of ASMB.
    /// (I don't anticipate any combined ASMB program to have more than 4 billion lines!)
    ip: u32,

}

/// Module consisting of executors for each keyword.
/// Each function has two arguments: mutable reference to AsmbiState and Vec<&str> tokens from the parser.
/// The tokens are expected to be passed by parser::line_valid. If an error that was supposed to be caught in that function is encountered here, the program will panic!, reminding the developer that parser::line_valid is not working properly.
mod exec {
    type Response = Result<(), &str>;

    // Using macros here because it can dictate parent function returns
    macro_rules! expect_regname_exists {
        ( $name:expr, $state:expr ) => (if !$state.regs.contains_key($name) {
            return Err(format!("Register '{}' does not exist", $name));
        })
    }
    macro_rules! unexpect_regname_exists {
        ( $name:expr, $state:expr ) => (if $state.regs.contains_key($name) {
            return Err(format!("Register '{}' already exists", $name));
        })
    }


    fn def(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: def <new register name> <evaluate_val candidate>
        unexpect_regname_exists!(toks[1], state);
        match parser::evaluate_val(toks[2], &state.regs) {
            Err(why) => return Err(format!("2nd parameter evaluation failure: {}", why)),
            Ok(value) => {
                state.regs.insert(toks[1], value);
                return Ok();
            }
        }
    }

    fn inc(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: inc <register name>
        expect_regname_exists!(toks[1], state);
        state.regs[toks[1]]++;
        Ok()
    }

    fn inct(state: &mut AsmbiState, toks: Vec<&str>) -> Response {
        // Syntax: inct <register name> <evaluate_val candidate>
        expect_regname_exists!(toks[1], state);
        // TODO: finish
    }
}
