#include <stdio.h>
#include <stdint.h>

int main(void) {
__asmb_line_1:;
	int32_t __asmb_reg_a = 0;
__asmb_line_2:;
	int32_t __asmb_reg_b = 0;
__asmb_line_3:;
	int32_t __asmb_reg_c = 100;
__asmb_line_4:;
	++__asmb_reg_b;
__asmb_line_5:;
	--__asmb_reg_c;
__asmb_line_6:;
	if (__asmb_reg_c != 0) goto __asmb_line_4;
__asmb_line_7:;
	printf("%d\n", __asmb_reg_c);
__asmb_line_8:;
	printf("%d\n", __asmb_reg_b);
__asmb_line_9:;
	__asmb_reg_b = 0;
__asmb_line_10:;
	__asmb_reg_c = 0;
__asmb_line_11:;
	__asmb_reg_a = -20;
__asmb_line_12:;
	--__asmb_reg_b;
__asmb_line_13:;
	++__asmb_reg_a;
__asmb_line_14:;
	if (__asmb_reg_a != 0) goto __asmb_line_12;
__asmb_line_15:;
	printf("%d\n", __asmb_reg_a);
__asmb_line_16:;
	printf("%d\n", __asmb_reg_b);
return 0;
}
