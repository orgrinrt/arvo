	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w12_hand
	.p2align	2
_w12_hand:
	.cfi_startproc
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #2
	mov	w13, #1000
LBB0_1:
	ldurb	w14, [x12, #-2]
	ldurb	w15, [x12, #-1]
	ldrb	w16, [x12, #2]
	ldrb	w17, [x12]
	ldrb	w0, [x12, #3]
	orr	x17, x15, x17, lsl #8
	orr	x0, x16, x0, lsl #8
	bfi	x14, x15, #8, #4
	ldrb	w15, [x12, #1]
	bfi	x15, x16, #8, #4
	add	x8, x14, x8
	add	x9, x9, x17, lsr #4
	add	x10, x15, x10
	add	x11, x11, x0, lsr #4
	add	x12, x12, #6
	subs	x13, x13, #4
	b.ne	LBB0_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret
	.cfi_endproc

	.globl	_w12_typed
_w12_typed = _w12_hand
.subsections_via_symbols
