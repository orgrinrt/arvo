	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w26_hand
	.p2align	2
_w26_hand:
	.cfi_startproc
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #6
	mov	w13, #1000
LBB0_1:
	ldur	w14, [x12, #-6]
	ldur	w15, [x12, #-3]
	ldr	w16, [x12]
	and	x14, x14, #0x3ffffff
	ubfx	x15, x15, #2, #26
	ldur	w17, [x12, #3]
	ubfx	x16, x16, #4, #26
	add	x8, x8, x14
	add	x9, x9, x15
	add	x10, x10, x16
	add	x11, x11, x17, lsr #6
	add	x12, x12, #13
	subs	x13, x13, #4
	b.ne	LBB0_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret
	.cfi_endproc

	.globl	_w26_native
	.p2align	2
_w26_native:
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
	bic.4s	v4, #252, lsl #24
	bic.4s	v5, #252, lsl #24
	bic.4s	v6, #252, lsl #24
	bic.4s	v7, #252, lsl #24
	uaddw.2d	v0, v0, v4
	uaddw2.2d	v0, v0, v4
	uaddw.2d	v1, v1, v5
	uaddw2.2d	v1, v1, v5
	uaddw.2d	v2, v2, v6
	uaddw2.2d	v2, v2, v6
	uaddw.2d	v3, v3, v7
	uaddw2.2d	v3, v3, v7
	subs	x9, x9, #16
	b.ne	LBB1_1
	add.2d	v0, v1, v0
	add.2d	v1, v3, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	ldr	q0, [x0, #3968]
	bic.4s	v0, #252, lsl #24
	uaddw.2d	v1, v1, v0
	uaddw2.2d	v0, v1, v0
	ldr	q1, [x0, #3984]
	bic.4s	v1, #252, lsl #24
	uaddw.2d	v0, v0, v1
	uaddw2.2d	v0, v0, v1
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.cfi_endproc

	.globl	_w26_typed
	.p2align	2
_w26_typed:
	.cfi_startproc
	mov	x9, #0
	mov	x8, #0
	mov	w10, #1000
LBB2_1:
	lsr	x11, x9, #3
	ldr	w11, [x0, x11]
	and	x12, x9, #0x6
	lsr	x11, x11, x12
	and	x11, x11, #0x3ffffff
	add	x8, x11, x8
	add	x9, x9, #26
	subs	x10, x10, #1
	b.ne	LBB2_1
	mov	x0, x8
	ret
	.cfi_endproc

.subsections_via_symbols
