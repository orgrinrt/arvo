	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w40_hand
	.p2align	2
_w40_hand:
	.cfi_startproc
	mov	x8, x0
	mov	x0, #0
	add	x8, x8, #2
	mov	w9, #1000
LBB0_1:
	ldur	w10, [x8, #-2]
	ldrb	w11, [x8, #2]
	orr	x10, x10, x11, lsl #32
	add	x0, x10, x0
	add	x8, x8, #5
	subs	x9, x9, #1
	b.ne	LBB0_1
	ret
	.cfi_endproc

	.globl	_w40_typed
_w40_typed = _w40_hand
.subsections_via_symbols
