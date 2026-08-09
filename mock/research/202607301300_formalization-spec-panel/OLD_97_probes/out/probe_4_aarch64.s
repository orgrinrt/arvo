	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_cast_trunc
	.p2align	2
_cast_trunc:
	.cfi_startproc
	fcvtzs	x0, d0
	ret
	.cfi_endproc

.subsections_via_symbols
