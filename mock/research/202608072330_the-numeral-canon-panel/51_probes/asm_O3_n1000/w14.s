	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w14_hand
	.p2align	2
_w14_hand:
	.cfi_startproc
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #3
	mov	w13, #1000
LBB0_1:
	ldurb	w14, [x12, #-2]
	ldrb	w15, [x12]
	ldrb	w16, [x12, #2]
	ldurb	w17, [x12, #-1]
	ldrb	w0, [x12, #1]
	orr	w17, w14, w17, lsl #8
	ldrb	w1, [x12, #3]
	orr	w0, w15, w0, lsl #8
	orr	x1, x16, x1, lsl #8
	orr	w15, w17, w15, lsl #16
	orr	w16, w0, w16, lsl #16
	ldurb	w17, [x12, #-3]
	bfi	x17, x14, #8, #6
	ubfx	x14, x15, #6, #14
	ubfx	x15, x16, #4, #14
	add	x8, x17, x8
	add	x9, x14, x9
	add	x10, x15, x10
	add	x11, x11, x1, lsr #2
	add	x12, x12, #7
	subs	x13, x13, #4
	b.ne	LBB0_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret
	.cfi_endproc

	.globl	_w14_typed
_w14_typed = _w14_hand
.subsections_via_symbols
