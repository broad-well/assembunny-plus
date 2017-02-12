// The loader of files for ASMBI. A function here is directly called from main.rs.
use std::io::{Read, Write};
use std::fs::{File, OpenOptions};
use std::ops::Index;
use interpret;
use parser;
use parser::Token;
use gen_c;
use bytecode;

macro_rules! try_do_res {
	( $fun:expr, $err:expr ) => (try_failsafe!($fun, $err.to_owned()));
}

macro_rules! index_option {
	( $vec:expr, $index:expr ) => (if $index >= $vec.len() {
		None
	} else {
		Some($vec.index($index))
	})
}

pub fn run_file(filename: &str) -> Result<u64, String> {
	let mut fstr = file_to_string!(filename);

	let mut regs: Vec<String> = Vec::new();
	let mut ftoks: Vec<Vec<Token>> = Vec::new();

	let mut line_count: u64 = 0;

	// ftoks: File tokens
	for line in fstr.lines() {
		if let Some(tokens) = try!(parser::to_tokens(line, &mut regs)) {
			ftoks.push(tokens);
		}
	}

	let mut state = interpret::new_state(regs.len());
	while let Some(line) = index_option!(ftoks, state.ip as usize) {
		if let Err(errno) = interpret::execute(&mut state, line) {
			return Err(format!("Interpretation of line {} failed: {}", state.ip, errno));
		}
		state.ip += 1;
		line_count += 1;
	}
	Ok(line_count)
}

pub fn compile_file(filename: &str) -> Result<String, String> {
	let fstr = file_to_string!(filename);
	gen_c::compose(&fstr.lines().collect::<Vec<_>>())
}

pub fn convert_to_bytecode(src_file: &str, target_file: &str) -> Result<(), String> {
	let src = file_to_string!(src_file);
	let mut outfile: File = try_do_res!(OpenOptions::new()
		.write(true)
		.create(true)
		.open(target_file), "Unable to create file");
	try_do_res!(
		outfile.write(
			&*try_err_fallthru!(bytecode::to_bytecode(&src.lines().collect::<Vec<_>>()), "Bytecode generation failed: ")),
			"Unable to write to bytecode output file"
	);
	Ok(())
}