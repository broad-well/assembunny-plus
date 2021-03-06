// Main command-line interface to parse Assembunny files (*.asmb)
extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
#[macro_use] extern crate enum_primitive;
extern crate clap;
extern crate ansi_term;
extern crate byteorder;
use clap::{Arg, App};
use std::io;
use std::io::Write;
use ansi_term::Colour::Red;
#[macro_use] pub mod macros;
pub mod parser;
pub mod interpret;
pub mod gen_c;
pub mod loader;
pub mod bytecode;

/// Main function for the CLI. Uses `clap` for args handling.
fn main() {
	let clap_matches = App::new("Assembunny-plus")
		.version("0.0.1")
		.author("Michael P. <michael@mcmoo.org>")
		.about("A compiler, interpreter, and bytecode manager for Assembunny+, an ASM-like language extended from the Assembunny concept in Advent of Code 2016")
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
		.arg(Arg::with_name("to-bytecode")
			.short("b")
			.long("to-bytecode")
			.multiple(true)
			.value_name("ASMB+ source file")
			.value_name("Bytecode output file")
			.help("Converts the ASMB source file's contents to ASMBB and stores the binary data into the bytecode output file")
			.takes_value(true)
			.conflicts_with_all(&["interpret", "compile", "from-bytecode"]))
		.arg(Arg::with_name("from-bytecode")
			.short("e")
			.long("from-bytecode")
			.value_name("Bytecode input file")
			.help("Reads ASMBP bytecode from the specified input file and executes the instructions")
			.takes_value(true)
			.conflicts_with_all(&["interpret", "compile", "to-bytecode"]))
		.get_matches();

	if clap_matches.is_present("interpret") {
		if let Err(errno) = loader::run_file(
				clap_matches.value_of("interpret").unwrap()) {
			println!("{} {}", Red.paint("Run file failed:"), errno);
			abort!();
		}
	} else if clap_matches.is_present("to-bytecode") {
		// Convert to bytecode
		let fileinputs: Vec<_> = clap_matches.values_of("to-bytecode").unwrap().collect();
		if let Err(problem) = loader::convert_to_bytecode(fileinputs[0], fileinputs[1]) {
			println!("{} {}", Red.paint("Conversion to bytecode failed:"), problem);
			abort!();
		}
	} else if clap_matches.is_present("from-bytecode") {
		// Run bytecode
		if let Err(problem) = loader::run_bytecode(clap_matches.value_of("from-bytecode").unwrap()) {
			println!("{} {}", Red.paint("Execution of bytecode failed:"), problem);
			abort!();
			// TODO: a macro for the procedure above, repeated 3 times.
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
					continue;
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
			Err(errno) => {
				println!("{} {}", Red.paint("Compile file failed:"), errno);
				abort!();
			}
		}
	}
}
