	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_proved_add
	.p2align	2
_proved_add:
	.cfi_startproc
	add	w8, w1, w0
	and	w0, w8, #0xff
	ret
	.cfi_endproc

	.globl	_unproved_add_checked
	.p2align	2
_unproved_add_checked:
	.cfi_startproc
	add	w8, w0, w1
	mov	w9, #255
	cmp	w8, #255
	csel	w0, w8, w9, lo
	ret
	.cfi_endproc

	.globl	_unproved_add_saturating
_unproved_add_saturating = _unproved_add_checked
.subsections_via_symbols
