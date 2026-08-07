	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_under_cold
	.p2align	2
_under_cold:
	.cfi_startproc
	adds	w8, w0, w1
	csinv	w0, w8, wzr, lo
	ret
	.cfi_endproc

	.globl	_under_hot
	.p2align	2
_under_hot:
	.cfi_startproc
	add	w0, w1, w0
	ret
	.cfi_endproc

.subsections_via_symbols
