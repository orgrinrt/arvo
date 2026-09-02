	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_get_one_4_4
	.p2align	2
_get_one_4_4:
	.cfi_startproc
	mov	w0, #16
	ret
	.cfi_endproc

.subsections_via_symbols
