	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w20_hand
	.p2align	2
_w20_hand:
	.cfi_startproc
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #4
	mov	w13, #1000
LBB0_1:
	ldurb	w14, [x12, #-2]
	ldrb	w15, [x12, #3]
	ldurb	w16, [x12, #-1]
	ldrb	w17, [x12, #4]
	ldurh	w0, [x12, #-4]
	orr	x16, x14, x16, lsl #8
	ldurh	w1, [x12, #1]
	orr	x17, x15, x17, lsl #8
	ldrb	w2, [x12]
	orr	x16, x16, x2, lsl #16
	ldrb	w2, [x12, #5]
	orr	x17, x17, x2, lsl #16
	bfi	x0, x14, #16, #4
	bfi	x1, x15, #16, #4
	add	x8, x0, x8
	add	x9, x9, x16, lsr #4
	add	x10, x1, x10
	add	x11, x11, x17, lsr #4
	add	x12, x12, #10
	subs	x13, x13, #4
	b.ne	LBB0_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret
	.cfi_endproc

	.globl	_w20_typed
_w20_typed = _w20_hand
.subsections_via_symbols
