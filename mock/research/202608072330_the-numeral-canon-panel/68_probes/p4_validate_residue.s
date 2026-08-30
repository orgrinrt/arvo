	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_add_bare
	.p2align	2
_add_bare:
	.cfi_startproc
	ldrb	w8, [x0]
	ldrb	w9, [x1]
	add	w0, w9, w8
	ret
	.cfi_endproc

	.globl	_add_runtime_validated
	.p2align	2
_add_runtime_validated:
	.cfi_startproc
	orr	w8, w1, w0
	add	w9, w1, w0
	tst	w8, #0x80
	csel	w0, wzr, w9, ne
	ret
	.cfi_endproc

	.globl	_trusted_constant
	.p2align	2
_trusted_constant:
	.cfi_startproc
	mov	w0, #123
	ret
	.cfi_endproc

	.globl	_add_trusted
_add_trusted = _add_bare
.subsections_via_symbols
