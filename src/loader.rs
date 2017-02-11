// The loader of files for ASMBI. A function here is directly called from main.rs.
use std::io::Read;
use std::fs::File;
use std::ops::Index;
use interpret;
use parser;
use parser::Token;
use gen_c;

macro_rules! try_do_res {
	( $fun:expr, $err:expr ) => (match $fun {
		Ok(x) => x,
		Err(_) => return Err($err.to_owned())
	});
}

macro_rules! index_option {
	( $vec:expr, $index:expr ) => (if $index >= $vec.len() {
		None
	} else {
		Some($vec.index($index))
	})
}
pub fn run_file(filename: &str) -> Result<u64, String> {
	let mut file = try_do_res!(File::open(filename), "File not found");
	let mut fstr = String::new();
	try_do_res!(file.read_to_string(&mut fstr), "File unreadable");

	let mut regs: Vec<String> = Vec::new();
	let mut ftoks: Vec<Vec<Token>> = Vec::new();

	let mut line_count: u64 = 0;

	// ftoks: File tokens
	for line in fstr.lines() {
		if let Some(tokens) = try!(parser::to_tokens(line, &mut regs)) {
			// Trick to convert Tokens to references
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
	let mut file = try_do_res!(File::open(filename), "File not found");
	let mut fstr = String::new();
	try_do_res!(file.read_to_string(&mut fstr), "File unreadable");
	gen_c::compose(&fstr.lines().collect::<Vec<_>>())
}
