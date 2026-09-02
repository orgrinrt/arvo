	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w47_hand
	.p2align	2
_w47_hand:
	.cfi_startproc
	mov	x9, #0
	mov	x8, #0
	mov	w10, #1000
LBB0_1:
	and	x11, x9, #0x7
	add	x12, x0, x9, lsr #3
	ldr	w13, [x12]
	ldrb	w14, [x12, #4]
	orr	x13, x13, x14, lsl #32
	ldrb	w14, [x12, #5]
	orr	x13, x13, x14, lsl #40
	ldrb	w12, [x12, #6]
	orr	x12, x13, x12, lsl #48
	lsr	x11, x12, x11
	and	x11, x11, #0x7fffffffffff
	add	x8, x11, x8
	add	x9, x9, #47
	subs	x10, x10, #1
	b.ne	LBB0_1
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_w47_native
	.p2align	2
_w47_native:
	.cfi_startproc
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #1000
	mov	x10, #140737488355327
	dup.2d	v1, x10
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

	.globl	_w47_typed
	.p2align	2
_w47_typed:
	.cfi_startproc
	mov	x9, #0
	mov	x8, #0
	mov	w10, #1000
LBB2_1:
	add	x11, x0, x9, lsr #3
	ldr	w12, [x11]
	ldrb	w13, [x11, #4]
	orr	x12, x12, x13, lsl #32
	ldrb	w13, [x11, #5]
	orr	x12, x12, x13, lsl #40
	ldrb	w11, [x11, #6]
	orr	x11, x12, x11, lsl #48
	and	x12, x9, #0x7
	lsr	x11, x11, x12
	and	x11, x11, #0x7fffffffffff
	add	x8, x11, x8
	add	x9, x9, #47
	subs	x10, x10, #1
	b.ne	LBB2_1
	mov	x0, x8
	ret
	.cfi_endproc

.subsections_via_symbols
