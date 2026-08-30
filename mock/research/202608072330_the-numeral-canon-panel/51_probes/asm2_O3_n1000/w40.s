	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w40_hand
	.p2align	2
_w40_hand:
	.cfi_startproc
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #9
	mov	w13, #1000
LBB0_1:
	ldur	w14, [x12, #-9]
	ldur	w15, [x12, #-4]
	ldur	w16, [x12, #1]
	ldur	w17, [x12, #6]
	ldurb	w0, [x12, #-5]
	ldrb	w1, [x12]
	ldrb	w2, [x12, #5]
	orr	x14, x14, x0, lsl #32
	ldrb	w0, [x12, #10]
	orr	x15, x15, x1, lsl #32
	orr	x16, x16, x2, lsl #32
	orr	x17, x17, x0, lsl #32
	add	x8, x14, x8
	add	x9, x15, x9
	add	x10, x16, x10
	add	x11, x17, x11
	add	x12, x12, #20
	subs	x13, x13, #4
	b.ne	LBB0_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret
	.cfi_endproc

	.globl	_w40_native
	.p2align	2
_w40_native:
	.cfi_startproc
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #1000
	movi.2d	v1, #0x0000ffffffffff
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
LBB1_1:
	ldp	q5, q6, [x8, #-32]
	ldp	q7, q16, [x8], #64
	and.16b	v5, v5, v1
	and.16b	v6, v6, v1
	and.16b	v7, v7, v1
	and.16b	v16, v16, v1
	add.2d	v0, v5, v0
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	add.2d	v4, v16, v4
	subs	x9, x9, #8
	b.ne	LBB1_1
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	add.2d	v0, v4, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.cfi_endproc

	.globl	_w40_typed
	.p2align	2
_w40_typed:
	.cfi_startproc
	mov	x8, x0
	mov	x0, #0
	add	x8, x8, #2
	mov	w9, #1000
LBB2_1:
	ldur	w10, [x8, #-2]
	ldrb	w11, [x8, #2]
	orr	x10, x10, x11, lsl #32
	add	x0, x10, x0
	add	x8, x8, #5
	subs	x9, x9, #1
	b.ne	LBB2_1
	ret
	.cfi_endproc

.subsections_via_symbols
