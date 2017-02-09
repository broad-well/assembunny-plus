use parser;
/*
  This mod generates C code from Assembunny+.
  The conventional usage of gen_c is after the user has "checked" their code with the interpreter. Therefore, the C generator does not provide any checks except parser::line_valid.

  Example:
	(ASMB)
	1  def a 0
	2  def b 0
	3  def c 0
	4  def d 0
	5  inct c d
	6  inc a
	7  outn a

	  |
	  V

	(C)
	#include <stdio.h>
	#include <stdint.h>

	int main(void) {
	__asmb_line_1:
		int32_t __asmb_reg_a = 0;
	__asmb_line_2:
		int32_t __asmb_reg_b = 0;
	__asmb_line_3:
		int32_t __asmb_reg_c = 0;
	__asmb_line_4:
		int32_t __asmb_reg_d = 0;
	__asmb_line_5:
		__asmb_reg_c += __asmb_reg_d;
	__asmb_line_6:
		++__asmb_reg_a;
	__asmb_line_7:
		puts(__asmb_reg_a);
	}
 */

// C semantics
// NOTE: This is directly related to the "starting with '__'" check in parser.rs.

/// Prefix of a C variable representing a register
/// Example: "__asmb_reg_" means `int32_t __asmb_reg_rmta;` for register "rmta"
const REG_VARNAME_PREFIX: &'static str = "__asmb_reg_";

/// Prefix of a C label representing a line in the .asmb source
/// Example: "__asmb_line_" means `__asmb_line_41:` for line 41
/// This is required for `jnz` to work.
const LINE_LABEL_PREFIX: &'static str = "__asmb_line_";

/// Indentation characters
/// Choose between Tabs and Spaces (the battle is still on!)
/// TODO: Make the selection available as a command line option
const INDENT: &'static str = "\t";

/// Prototype of generated C code
/// Will be used during the final compilation of C source
const C_PROTOTYPE: &'static str = "#include <stdio.h>\n#include <stdint.h>\n\nint main(void) {\n##return 0;\n}";

macro_rules! eval {
	( $arg:expr ) => (match $arg.parse::<i32>() {
		Ok(_) => $arg.to_owned(),
		Err(_) => reg!($arg)
	});
}

macro_rules! reg {
	( $name:expr ) => (format!("{}{}", gen_c::REG_VARNAME_PREFIX, $name));
}

macro_rules! line {
	( $num:expr ) => (format!("{}{}", gen_c::LINE_LABEL_PREFIX, $num));
}

/// Closures called to make modifications to C source draft
//static DRAFT_MODIFIERS: HashMap<&'static str, Box<FnMut(&Vec<&str>) -> String>> = hashmap!(
//	"def" => Box::new(|&args| {
//		format!("int32_t {}{} = {}", REG_VARNAME_PREFIX, args[1], eval!(args[2]))
//	})
//);

/// Collection of functions that generate C code on-demand.
// XXX: Please inform me if there's a more efficient way using a static HashMap or something else.
pub mod generators {
	use gen_c;

	pub fn def(args: &Vec<&str>) -> String {
		// Syntax: def <new reg name> <eval>
		format!("int32_t {} = {};", reg!(args[1]), eval!(args[2]))
	}

	pub fn inc(args: &Vec<&str>) -> String {
		// Syntax: inc <reg name>
		format!("++{};", reg!(args[1]))
	}

	pub fn inct(args: &Vec<&str>) -> String {
		// Syntax: inct <reg name> <eval>
		format!("{} += {};", reg!(args[1]), reg!(args[2]))
	}

	pub fn dec(args: &Vec<&str>) -> String {
		// Syntax: dec <reg name>
		format!("--{};", reg!(args[1]))
	}

	pub fn dect(args: &Vec<&str>) -> String {
		// Syntax: dect <reg name> <eval>
		format!("{} -= {};", reg!(args[1]), eval!(args[2]))
	}

	pub fn mul(args: &Vec<&str>) -> String {
		// Syntax: mul <reg name> <eval>
		format!("{} *= {};", reg!(args[1]), eval!(args[2]))
	}

	pub fn div(args: &Vec<&str>) -> String {
		// Syntax: div <reg name> <eval>
		format!("{} /= {};", reg!(args[1]), eval!(args[2]))
	}

	pub fn cpy(args: &Vec<&str>) -> String {
		// Syntax: cpy <eval> <reg name>
		format!("{} = {};", reg!(args[2]), eval!(args[1]))
	}

	pub fn jnz(args: &Vec<&str>, linenum: u32) -> String {
		// Syntax: jnz <eval not 0> <literal>
		let offset = args[2].parse::<i32>().unwrap();
		format!("if ({} != 0) goto {};", eval!(args[1]), line!(if offset < 0 {
			linenum - (-offset as u32)
		} else {
			linenum + (offset as u32)
		}))
	}

	pub fn out(args: &Vec<&str>) -> String {
		// Syntax: out <eval>
		format!("printf(\"%d \", {});", eval!(args[1]))
	}

	pub fn outn(args: &Vec<&str>) -> String {
		// Syntax: outn <eval>
		format!("printf(\"%d\\n\", {});", eval!(args[1]))
	}

	pub fn outc(args: &Vec<&str>) -> String {
		// Syntax: outc <eval>
		// NOTE: Does not support Unicode, because C doesn't
		format!("printf(\"%c\", {});", eval!(args[1]))
	}
}

/// Returns a line of C source code from a line of ASMB+.
pub fn get_cline(toks: &Vec<&str>, linenum: u32) -> Result<String, String> {
	// Execution worth is already checked at compose().

	// Line checked and invalid
	if let Err(err) = parser::line_valid(&toks) {
		return Err(format!("Invalid line: {}", err));
	}

	match toks[0].to_lowercase().as_str() {
		"def" => Ok(generators::def(toks)),
		"inc" => Ok(generators::inc(toks)),
		"inct" => Ok(generators::inct(toks)),
		"dec" => Ok(generators::dec(toks)),
		"dect" => Ok(generators::dect(toks)),
		"mul" => Ok(generators::mul(toks)),
		"div" => Ok(generators::div(toks)),
		"cpy" => Ok(generators::cpy(toks)),
		"jnz" => Ok(generators::jnz(toks, linenum)),
		"out" => Ok(generators::out(toks)),
		"outn" => Ok(generators::outn(toks)),
		"outc" => Ok(generators::outc(toks)),
		_ => Err(format!("Unknown keyword: {}", toks[0]))
	}
}

/// Returns the entire C program, ready to be written to a file.
pub fn compose(clines: &Vec<&str>) -> Result<String, String> {
	let mut infix = String::new();
	let mut linenum = 1;
	for line in clines.iter() {
		let tokens = parser::tokenize_line(line);
		if parser::worth_execution(&tokens).is_ok() {
			infix += &format!("{}{}:;\n{}{}\n", LINE_LABEL_PREFIX, linenum, INDENT,
				try!(get_cline(&tokens, linenum as u32)));
			linenum += 1;
		}
	}
	Ok(C_PROTOTYPE.to_owned().replace("##", &infix))
}
