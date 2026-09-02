	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w6_hand
	.p2align	2
_w6_hand:
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
	ldrb	w16, [x12], #3
	orr	w17, w14, w15, lsl #8
	orr	w15, w15, w16, lsl #8
	and	x14, x14, #0x3f
	ubfx	x17, x17, #6, #6
	ubfx	x15, x15, #4, #6
	add	x8, x8, x14
	add	x9, x17, x9
	add	x10, x15, x10
	add	x11, x11, x16, lsr #2
	subs	x13, x13, #4
	b.ne	LBB0_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret
	.cfi_endproc

	.globl	_w6_typed
_w6_typed = _w6_hand
.subsections_via_symbols
