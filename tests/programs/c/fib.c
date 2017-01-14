#include <stdio.h>
#include <stdint.h>

int main(void) {
__asmb_line_1:;
	int32_t __asmb_reg_i = 20;
__asmb_line_2:;
	int32_t __asmb_reg_a = 1;
__asmb_line_3:;
	int32_t __asmb_reg_b = 1;
__asmb_line_4:;
	printf("%d ", __asmb_reg_a);
__asmb_line_5:;
	printf("%d ", __asmb_reg_b);
__asmb_line_6:;
	__asmb_reg_a += __asmb_reg_b;
__asmb_line_7:;
	__asmb_reg_b += __asmb_reg_a;
__asmb_line_8:;
	--__asmb_reg_i;
__asmb_line_9:;
	if (__asmb_reg_i != 0) goto __asmb_line_4;
return 0;
}
