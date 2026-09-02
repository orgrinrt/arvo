	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w22_hand
	.p2align	2
_w22_hand:
	.cfi_startproc
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #5
	mov	w13, #1000
LBB0_1:
	ldur	w14, [x12, #-5]
	ldur	w15, [x12, #-3]
	ldr	w16, [x12]
	and	x14, x14, #0x3fffff
	ubfx	x15, x15, #6, #22
	ldur	w17, [x12, #3]
	ubfx	x16, x16, #4, #22
	ubfx	x17, x17, #2, #22
	add	x8, x8, x14
	add	x9, x9, x15
	add	x10, x10, x16
	add	x11, x11, x17
	add	x12, x12, #11
	subs	x13, x13, #4
	b.ne	LBB0_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret
	.cfi_endproc

	.globl	_w22_native
	.p2align	2
_w22_native:
	.cfi_startproc
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #992
	movi.4s	v1, #63, msl #16
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
	uaddw.2d	v0, v0, v5
	uaddw2.2d	v0, v0, v5
	uaddw.2d	v2, v2, v6
	uaddw2.2d	v2, v2, v6
	uaddw.2d	v3, v3, v7
	uaddw2.2d	v3, v3, v7
	uaddw.2d	v4, v4, v16
	uaddw2.2d	v4, v4, v16
	subs	x9, x9, #16
	b.ne	LBB1_1
	add.2d	v0, v2, v0
	add.2d	v1, v4, v3
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	ldr	q0, [x0, #3968]
	movi.4s	v2, #63, msl #16
	and.16b	v0, v0, v2
	uaddw.2d	v1, v1, v0
	uaddw2.2d	v0, v1, v0
	ldr	q1, [x0, #3984]
	and.16b	v1, v1, v2
	uaddw.2d	v0, v0, v1
	uaddw2.2d	v0, v0, v1
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.cfi_endproc

	.globl	_w22_typed
	.p2align	2
_w22_typed:
	.cfi_startproc
	mov	x9, #0
	mov	x8, #0
	mov	w10, #1000
LBB2_1:
	lsr	x11, x9, #3
	ldr	w11, [x0, x11]
	and	x12, x9, #0x6
	lsr	x11, x11, x12
	and	x11, x11, #0x3fffff
	add	x8, x11, x8
	add	x9, x9, #22
	subs	x10, x10, #1
	b.ne	LBB2_1
	mov	x0, x8
	ret
	.cfi_endproc

.subsections_via_symbols
