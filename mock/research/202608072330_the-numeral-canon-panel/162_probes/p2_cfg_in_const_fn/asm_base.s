	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_lowered
	.p2align	2
_lowered:
	.cfi_startproc
	mov	w8, #8191
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret
	.cfi_endproc

.subsections_via_symbols
