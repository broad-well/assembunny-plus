// Main command-line interface to parse Assembunny files (*.asmb)
extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
extern crate clap;
extern crate ansi_term;
use clap::{Arg, App};
use std::io;
use std::io::Write;
use ansi_term::Colour::Red;
pub mod parser;
pub mod interpret;
pub mod gen_c;
pub mod loader;

/// Main function for the CLI. Uses `clap` for args handling.
fn main() {
	let clap_matches = App::new("Assembunny-plus")
		.version("0.0.1")
		.author("Michael P. <michael@mcmoo.org>")
		.about("A C compiler and interpreter for Assembunny+, an ASM-like language extended from the Assembunny concept in Advent of Code 2016")
		.arg(Arg::with_name("interpret")
			.short("i")
			.long("interpret")
			.value_name("asmb file")
			.help("Launches the ASMB intepreter, or interprets a file if the filename is provided")
			.required(false)
			.takes_value(true))
		.arg(Arg::with_name("compile")
			.short("c")
			.long("compile")
			.value_name("asmb file")
			.help("Compiles the given ASMB file to C source code and prints it to STDOUT")
			.takes_value(true))
		.get_matches();

	if clap_matches.is_present("interpret") {
		if let Err(errno) = loader::run_file(
			clap_matches.value_of("interpret").unwrap()) {
			println!("{} {}", Red.paint("Run file failed:"), errno);
		}
	} else if !clap_matches.is_present("compile") {
		// Enter REPL
		println!("Welcome to the Assembunny-plus REPL.");
		println!("Use :help for help, :reg for registers and their values, and :unlicense for the unlicense.");
		println!("At the > prompt, enter your lines of Assembunny-plus.");
		let mut state = interpret::new_state(0);
		let mut regs: Vec<String> = Vec::new();
		let mut show_raw_token = false;
		loop {
			print!("{}::>", state.ip);
			io::stdout().flush().expect("Stdout clogged");
			let mut input = String::new();
			io::stdin().read_line(&mut input)
				.expect("Line read failed");

			if input.trim().is_empty() { continue }
			let str_tokens = parser::tokenize_line(&input);

			if input.starts_with(":") {
				match str_tokens[0] {
					":help" => println!("Not available now"),
					":reg" => {
						for (index, val) in state.regs.vec.iter().enumerate() {
							println!("{} => {}", regs[index], val);
						}
					},
					":exit" => {
						println!("Bye");
						break
					},
					":rawtoken" => {
						show_raw_token = !show_raw_token;
						println!("Toggled show raw tokens before execution");
					},
					_ => {}
				}
				continue;
			}

			let tokens = match parser::to_tokens(&input, &mut regs) {
				Ok(opttok) => if opttok.is_none() {
					continue
				} else {
					opttok.unwrap()
				},
				Err(problem) => {
					println!("{} {}", Red.paint("Failed to tokenize:"), problem);
					continue
				}
			};


			if str_tokens[0].to_lowercase() == "jnz" {
				println!("{}", Red.paint("This REPL does not support JNZ."));
				continue;
			}

			if show_raw_token {
				println!("{}", tokens.iter().map(|token| token.to_string()).collect::<Vec<_>>().join(","));
			}

			// Since the interpreter is optimized for files, we have to dynamically allocate before `def` lines get executed.
			if str_tokens[0].to_lowercase() == "def" {
				state.regs.vec.push(0);
			}

			if let Err(errmsg) = interpret::execute(&mut state, &tokens) {
				println!("{} {}", Red.paint("Failed:"), errmsg);
			} else {
				state.ip += 1;
			}
		}
	} else {
		match loader::compile_file(
				clap_matches.value_of("compile").unwrap()) {

			Ok(c_code) => println!("{}", c_code),
			Err(errno) => println!("{} {}", Red.paint("Compile file failed:"), errno)
		}
	}
}
