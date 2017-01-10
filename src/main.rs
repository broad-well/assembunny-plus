// Main command-line interface to parse Assembunny files (*.asmb)
extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
extern crate clap;
use clap::{Arg, App, SubCommand};
use std::io;
pub mod parser;
pub mod interpret;
pub mod gen_c;

/// Main function for the CLI. Uses `clap` for args handling.
fn main() {
    let clap_matches = App::new("Assembunny-plus")
        .version("0.0.1")
        .author("Michael P. <michael@mcmoo.org>")
        .about("A C compiler and interpreter for Assembunny+, an ASM-like language extended from the Assembunny concept in Advent of Code 2016")
        .arg(Arg::with_name("interpret")
            .short("i")
            .long("config")
            .value_name("asmb file")
            .help("Launches the ASMB intepreter, or interprets a file if the filename is provided")
            .takes_value(true))
        .get_matches();
    if clap_matches.value_of("interpret").unwrap() == "test" {
        // Enter REPL
        println!("Welcome to the Assembunny-plus REPL.");
        println!("Use :help for help, :reg for registers and their values, and :unlicense for the unlicense.");
        println!("At the > prompt, enter your lines of Assembunny-plus.");
        let mut state = interpret::new_state();
        while true {
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Line read failed");
            input.trim();

            let tokens = parser::tokenize_line(input);

            if input.starts_with(":") {
                match tokens[0] {
                    ":help" => println!("Not available now"),
                    ":reg" => println!("{}", state.regs),
                    ":exit" => {
                        println!("Bye");
                        break
                    },
                    _ => {}
                }
            }
            if tokens[0].to_lowercase() == "jnz" {
                println!("The REPL does not support JNZ yet.");
                continue
            }

            interpret::execute(state, tokens);
        }
    }
}