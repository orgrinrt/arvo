	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_a_size_first
	.p2align	2
_a_size_first:
	.cfi_startproc
	mov	x8, #0
	cbz	x1, LBB0_3
	mov	w9, #8191
LBB0_2:
	ldr	x10, [x0], #8
	add	x8, x10, x8
	cmp	x8, x9
	csel	x8, x8, x9, lo
	subs	x1, x1, #1
	b.ne	LBB0_2
LBB0_3:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_a_time_first
	.p2align	2
_a_time_first:
	.cfi_startproc
	cbz	x1, LBB1_4
	mov	x8, x0
	mov	x0, #0
	lsl	x9, x1, #3
	sub	x8, x8, #8
	mov	w10, #8191
LBB1_2:
	ldr	x11, [x8, x9]
	add	x11, x11, x0
	cmp	x11, x10
	csel	x0, x11, x10, lo
	subs	x9, x9, #8
	b.ne	LBB1_2
	ret
LBB1_4:
	mov	x0, #0
	ret
	.cfi_endproc

	.globl	_b_as_stored
_b_as_stored = _a_time_first
	.globl	_c_default
_c_default = _a_time_first
	.globl	_b_other_weighting
_b_other_weighting = _a_size_first
	.globl	_c_overridden
_c_overridden = _a_size_first
.subsections_via_symbols
