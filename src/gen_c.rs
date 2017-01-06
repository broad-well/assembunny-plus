
mod macros;

/*
  This mod generates C code from Assembunny+.

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
const REG_VARNAME_PREFIX = "__asmb_reg_";

/// Prefix of a C label representing a line in the .asmb source
/// Example: "__asmb_line_" means `__asmb_line_41:` for line 41
/// This is required for `jnz` to work.
const LINE_LABEL_PREFIX = "__asmb_line_";

/// Indentation characters
/// Choose between Tabs and Spaces (the battle is still on!)
/// TODO: Make the selection available as a command line option
const INDENT = "\t";

/// Prototype of generated C code
/// Will be used during the final compilation of C source
const C_PROTOTYPE = "#include <stdio.h>\n#include <stdint.h>\n\nint main(void) "
    "{\n##return 0;\n}";
