	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w10_hand
	.p2align	2
_w10_hand:
	.cfi_startproc
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #2
	mov	w13, #1000
LBB0_1:
	ldurb	w14, [x12, #-1]
	ldrb	w15, [x12]
	ldrb	w16, [x12, #1]
	ldrb	w17, [x12, #2]
	orr	w0, w14, w15, lsl #8
	orr	w15, w15, w16, lsl #8
	orr	x16, x16, x17, lsl #8
	ldurb	w17, [x12, #-2]
	bfi	x17, x14, #8, #2
	ubfx	x14, x0, #2, #10
	ubfx	x15, x15, #4, #10
	add	x8, x17, x8
	add	x9, x14, x9
	add	x10, x15, x10
	add	x11, x11, x16, lsr #6
	add	x12, x12, #5
	subs	x13, x13, #4
	b.ne	LBB0_1
	add	x8, x9, x8
	add	x8, x10, x8
	add	x0, x11, x8
	ret
	.cfi_endproc

	.globl	_w10_native
	.p2align	2
_w10_native:
	.cfi_startproc
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #992
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB1_1:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	bic.8h	v4, #252, lsl #8
	bic.8h	v5, #252, lsl #8
	bic.8h	v6, #252, lsl #8
	bic.8h	v7, #252, lsl #8
	ushll2.4s	v16, v4, #0
	ushll.4s	v4, v4, #0
	uaddw.2d	v0, v0, v4
	uaddw2.2d	v0, v0, v4
	uaddw.2d	v0, v0, v16
	uaddw2.2d	v0, v0, v16
	ushll2.4s	v4, v5, #0
	ushll.4s	v5, v5, #0
	uaddw.2d	v1, v1, v5
	uaddw2.2d	v1, v1, v5
	uaddw.2d	v1, v1, v4
	uaddw2.2d	v1, v1, v4
	ushll2.4s	v4, v6, #0
	ushll.4s	v5, v6, #0
	uaddw.2d	v2, v2, v5
	uaddw2.2d	v2, v2, v5
	uaddw.2d	v2, v2, v4
	uaddw2.2d	v2, v2, v4
	ushll2.4s	v4, v7, #0
	ushll.4s	v5, v7, #0
	uaddw.2d	v3, v3, v5
	uaddw2.2d	v3, v3, v5
	uaddw.2d	v3, v3, v4
	uaddw2.2d	v3, v3, v4
	subs	x9, x9, #32
	b.ne	LBB1_1
	add.2d	v0, v1, v0
	add.2d	v1, v3, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	ldr	q0, [x0, #1984]
	bic.8h	v0, #252, lsl #8
	ushll2.4s	v2, v0, #0
	ushll.4s	v0, v0, #0
	uaddw.2d	v1, v1, v0
	uaddw2.2d	v0, v1, v0
	uaddw.2d	v0, v0, v2
	uaddw2.2d	v0, v0, v2
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.cfi_endproc

	.globl	_w10_typed
_w10_typed = _w10_hand
.subsections_via_symbols
