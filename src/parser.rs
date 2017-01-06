// Parser of Assembunny code, part of assembunny_extended
extern crate regex;
use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

/* Available keywords:

 * DEF = Define *new* register (let)
     Usage: DEF <register name> <value>
     Note: A register name is case-sensitive and should only contain letters, numbers, and underscore; it should not start with a number; and it should not start with two underscores.
           Do not use DEF                 if regname_valid(tok) {
                    return Err("Reg")
                }
to set an existing register to a value. use CPY instead.
           Each register is actually stored as a 32-bit integer.

 * INC = Increment register's value (++)
     Usage: INC <register name>

 * INCT = Add to register (+=)
     Usage: INCT <register name> <value (can also be name of another register)>

 * DEC = Decrement register's value (--)
     Usage: DEC <register name>

 * DECT = Subtract from register (-=)
     Usage: DECT <register name> <value (can also be name of another register)>

 * MUL = Multiply register's value (*=)
     Usage: MUL <register name> <multiplier>
     Example:
       def bnr 5
       mul bnr -2
       ---
       Register BNR now has a value of `5 * -2`, or -10.
     Note: Eric Wastl expected Assembunny to be good at multiplying. (please correct me if I made a mistake here)

 * DIV = Divide register's value (/=)
     Usage: DIV <register name> <divisor>
     Example:
       def mb 52
       div mb 5
       ---
       Register MB now has a value of `52 / 5`, or 10 when floored.
     Note: DIV floors results because registers only store 32-bit integers.

 * CPY = Copy value to register (value can be name of a register)
     Usage: CPY <value> <register>
     Example 1: CPY 4 MyRegister
     Example 2: CPY RegA RegB

 * JNZ = Jump to instruction relative to itself
     Explanation: This keyword causes a jump to the line that's _Y_ lines away from this instruction *if _X_ is not zero*
     Usage: JNZ <X> <Y>

     Example:
       125  inc ej
       126  dec eh
       127  dect ms 14
       128  cpy 14 mt
       129  cpy mt qr
       130  jnz qr -2
       ---
       In this example, when the program reaches line 130 it jumps to line 128 (or 130 + (-2)) because qr has a value of 14
 
 * OUT = Write value to STDOUT, with trailing whitespace
     Usage: OUT <value (can be register name or literal)>
     Example:
       def rm 42
       def rn 43
       def rb 52
       cpy -41 rn
       inc rn
       out rn
       out 40
       ---
       STDOUT will be: "-40 40 "

 * OUTN = Write value to STDOUT, with trailing newline
     Usage: OUTN <value (can be register name or literal)>
     Example:
       def mj 42
       def mo 41
       inct mj mo
       outn 13
       outn mj
       ---
       STDOUT will be: "13\n83\n"

 * OUTC = Write character to STDOUT with value as codepoint (chr)
     Usage: OUTC <value (can be register name or literal)>
     Example:
       def rm 42
       def tm 0
       cpy rm tm
       inc tm
       outc tm
       ---
       STDOUT will be: "+", since tm's value is 43 and `+` has an ASCII codepoint of 43.

 */

const KEYWORDS: HashMap<&str, &'static str> = hashmap!(
    "def" => "RB", "inc" => "R", "inct" => "RB", "dec" => "R", "dect" => "RB",
    "mul" => "RB", "div" => "RB", "cpy" => "BR", "jnz" => "BB", "out" => "B",
    "outn" => "B", "outc" => "B"
);

/// Tokenizes the given string by whitespaces and returns the tokens in a Vec.
fn tokenize_line(line: &str) -> Vec<&str> {
    line.split_whitespace().collect::<Vec<_>>()
}

/// Checks if the given register name is valid.
pub fn regname_valid(name: &str) -> Result<(), &'static str> {

    lazy_static! {
        static ref CHAR_RE: Regex = Regex::new(r"[^0-9a-zA-Z_]").unwrap();
        static ref ISDIGIT: Regex = Regex::new(r"[0-9]").unwrap();
    }

    // Regex match 1: forbidden characters
    if CHAR_RE.is_match(name) {
        return Err("Forbidden characters in register name '" + name + "'");
    }
    // Regex match 2: starting with a number
    if ISDIGIT.is_match(name.char_at(0)) {
        return Err("Register name '" + name + "' should not start with a digit");
    }
    // Method match: starting with "__"
    if name.starts_with("__") {
        return Err(
            "Register name should not start with two underscores; "
            "this is occupied for C code generation purposes.");
    }
    Ok()
}

/// Checks whether the given token is an integer literal by attempting to convert it to an i32.
/// If it is, return Ok(integer value of token)
/// Otherwise return Err()
fn is_literal(tok: &str) -> Result<i32, ()> {
    match i32::from_str(tok) {
        Ok(val) => val,
        Err(_) => Err()
    }
}

/// Checks if the given line of ASMB is valid.
/// This function checks the keyword, parameter count, and parameter types (literal/register name)
fn line_valid(line: &str) -> Result<(), &'static str> {
    let toks = tokenize_line(line);
    // Check 1: keyword
    if !KEYWORDS.contains_key(toks[0].to_lowercase()) {
        return Err("Unknown keyword");
    }
    let param_rule = KEYWORDS[toks[0].to_lowercase()];
    // Check 2: param count
    if param_rule.len() != toks.len() - 1 {
        return Err(format!(
            "Expected {} parameters, received {}", param_rule.len(), toks.len() - 1));
    }
    // Check 3: param type
    for (index, rule) in param_rule.chars().enumerate() {
        // index+1!
        // rule can be 'R', 'L', or 'B'
        let is_litparam = is_literal(toks[index+1]).is_ok();
        if (is_litparam && rule == 'R') || (!is_litparam && rule == 'L') {
            return Err(format!(
                "Parameter '{}' does not comply with the parameter rules of keyword '{}' ({})",
                toks[index+1], toks[0], rule));
        }
    }
    Ok()
}

/// Attempts to evaluate the given token and return the numeric value.
/// Also borrows the registers HashMap for lookups.
/// Example: evaluate_val("mny", {"t5" => 42, "mny" => -3}) returns -3
/// Example: evaluate_val("-41", {"irr" => 0}) returns -41
/// Note: For interpreter only
pub fn evaluate_val(tok: &str, regs: &HashMap<&str, i32>)
                    -> Result<i32, &'static str> {
    match i32::from_str(tok) {
        Ok(literal) => return Ok(literal),
        Err(_) => {
            let validate_result = regname_valid(tok);
            if validate_result.is_err() {
                return Err("'" + tok + "' is an invalid register name: " +
                            validate_result.err().unwrap());
            }
            let register_val = regs.get(tok);
            if register_val.is_none() {
                return Err(format!(
                    "'{}' is an unknown register name in this context", tok));
            }
            return Ok(register_val.unwrap());
        }
    }
}

///
