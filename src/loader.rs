// The loader of files for ASMBI. A function here is directly called from main.rs.
use std::io::Read;
use std::fs::File;
use interpret;
use parser;
use gen_c;

macro_rules! try_do_res {
	( $fun:expr, $err:expr ) => (match $fun {
		Ok(_) => $fun.unwrap(),
		Err(_) => return Err($err.to_owned())
	});
}

pub fn run_file(filename: &str) -> Result<(), String> {
	let mut file = try_do_res!(File::open(filename), "File not found");
	let mut fstr = String::new();
	try_do_res!(file.read_to_string(&mut fstr), "File unreadable");
	let mut state = interpret::new_state();

	while let Some(line) = fstr.lines().nth(state.ip as usize) {
		if let Err(errno) = interpret::execute(&mut state, parser::tokenize_line(line)) {
			return Err(format!("Interpretation of line {} failed: {}", state.ip, errno));
		}
		state.ip += 1;
	}
	Ok(())
}

pub fn compile_file(filename: &str) -> Result<String, String> {
	let mut file = try_do_res!(File::open(filename), "File not found");
	let mut fstr = String::new();
	try_do_res!(file.read_to_string(&mut fstr), "File unreadable");
	gen_c::compose(&fstr.lines().collect::<Vec<_>>())
}
