	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_eager_w13_head_d1
	.p2align	2
_eager_w13_head_d1:
	.cfi_startproc
	cbz	x1, LBB0_4
	mov	x8, x0
	mov	w0, #0
	lsl	x9, x1, #2
LBB0_2:
	ldr	w10, [x8], #4
	add	w11, w2, w0
	add	w10, w10, w11
	and	w0, w10, #0x1fff
	subs	x9, x9, #4
	b.ne	LBB0_2
	ret
LBB0_4:
	mov	x0, #0
	ret
	.cfi_endproc

	.globl	_eager_w13_head_d8
	.p2align	2
_eager_w13_head_d8:
	.cfi_startproc
	cbz	x1, LBB1_4
	mov	x8, x0
	mov	w0, #0
	lsl	x9, x1, #2
	mov	w10, #1177
LBB1_2:
	ldr	w11, [x8], #4
	add	w12, w11, w2
	add	w11, w11, w12, lsl #1
	eor	w11, w11, w10
	add	w12, w11, w2
	add	w11, w11, w12, lsl #1
	eor	w11, w11, w10
	add	w11, w11, w0
	and	w0, w11, #0x1fff
	subs	x9, x9, #4
	b.ne	LBB1_2
	ret
LBB1_4:
	mov	x0, #0
	ret
	.cfi_endproc

	.globl	_eager_w13_min_d1
	.p2align	2
_eager_w13_min_d1:
	.cfi_startproc
	cbz	x1, LBB2_4
	mov	x8, x0
	mov	w0, #0
	lsl	x9, x1, #1
LBB2_2:
	ldrh	w10, [x8], #2
	add	w11, w2, w0
	add	w10, w10, w11
	and	w0, w10, #0x1fff
	subs	x9, x9, #2
	b.ne	LBB2_2
	ret
LBB2_4:
	mov	x0, #0
	ret
	.cfi_endproc

	.globl	_eager_w13_min_d8
	.p2align	2
_eager_w13_min_d8:
	.cfi_startproc
	cbz	x1, LBB3_4
	mov	x8, x0
	mov	w0, #0
	lsl	x9, x1, #1
	mov	w10, #1177
LBB3_2:
	ldrh	w11, [x8], #2
	add	w12, w11, w2
	add	w11, w11, w12, lsl #1
	eor	w11, w11, w10
	add	w12, w11, w2
	add	w11, w11, w12, lsl #1
	eor	w11, w11, w10
	add	w11, w11, w0
	and	w0, w11, #0x1fff
	subs	x9, x9, #2
	b.ne	LBB3_2
	ret
LBB3_4:
	mov	x0, #0
	ret
	.cfi_endproc

	.globl	_eager_w32_head_d3
	.p2align	2
_eager_w32_head_d3:
	.cfi_startproc
	cbz	x1, LBB4_4
	mov	x8, x0
	mov	x0, #0
	lsl	x9, x1, #3
LBB4_2:
	ldr	w10, [x8], #8
	add	w11, w10, w2
	add	w10, w10, w11, lsl #1
	add	w0, w10, w0
	subs	x9, x9, #8
	b.ne	LBB4_2
	ret
LBB4_4:
	mov	x0, #0
	ret
	.cfi_endproc

	.globl	_eager_w32_min_d3
	.p2align	2
_eager_w32_min_d3:
	.cfi_startproc
	cbz	x1, LBB5_3
	lsl	x8, x1, #2
	sub	x10, x8, #4
	cmp	x10, #12
	b.hs	LBB5_4
	mov	w12, #0
	mov	x11, x0
	b	LBB5_13
LBB5_3:
	mov	x0, #0
	ret
LBB5_4:
	lsr	x9, x10, #2
	add	x9, x9, #1
	dup.4s	v0, w2
	cmp	x10, #60
	b.hs	LBB5_6
	mov	x10, #0
	mov	w12, #0
	b	LBB5_10
LBB5_6:
	and	x11, x9, #0xc
	and	x10, x9, #0x7ffffffffffffff0
	add	x12, x0, #32
	movi.2d	v1, #0000000000000000
	movi.4s	v2, #3
	neg.4s	v3, v0
	and	x13, x9, #0x7ffffffffffffff0
	movi.2d	v4, #0000000000000000
	movi.2d	v5, #0000000000000000
	movi.2d	v6, #0000000000000000
LBB5_7:
	ldp	q7, q16, [x12, #-32]
	ldp	q17, q18, [x12], #64
	add.4s	v7, v7, v0
	add.4s	v16, v16, v0
	add.4s	v17, v17, v0
	add.4s	v18, v18, v0
	mov.16b	v19, v3
	mla.4s	v19, v7, v2
	mov.16b	v7, v3
	mla.4s	v7, v16, v2
	mov.16b	v16, v3
	mla.4s	v16, v17, v2
	mov.16b	v17, v3
	mla.4s	v17, v18, v2
	add.4s	v1, v19, v1
	add.4s	v4, v7, v4
	add.4s	v5, v16, v5
	add.4s	v6, v17, v6
	subs	x13, x13, #16
	b.ne	LBB5_7
	add.4s	v1, v4, v1
	add.4s	v1, v5, v1
	add.4s	v1, v6, v1
	addv.4s	s1, v1
	fmov	w12, s1
	cmp	x9, x10
	b.eq	LBB5_15
	cbz	x11, LBB5_16
LBB5_10:
	and	x13, x9, #0x7ffffffffffffffc
	add	x11, x0, x13, lsl #2
	movi.2d	v1, #0000000000000000
	mov.s	v1[0], w12
	sub	x12, x10, x13
	add	x10, x0, x10, lsl #2
	movi.4s	v2, #3
	neg.4s	v3, v0
LBB5_11:
	ldr	q4, [x10], #16
	add.4s	v4, v4, v0
	mov.16b	v5, v3
	mla.4s	v5, v4, v2
	add.4s	v1, v5, v1
	adds	x12, x12, #4
	b.ne	LBB5_11
	addv.4s	s0, v1
	fmov	w12, s0
	cmp	x9, x13
	b.eq	LBB5_15
LBB5_13:
	add	x8, x0, x8
LBB5_14:
	ldr	w9, [x11], #4
	add	w10, w9, w2
	add	w9, w9, w10, lsl #1
	add	w12, w9, w12
	cmp	x11, x8
	b.ne	LBB5_14
LBB5_15:
	mov	w0, w12
	ret
LBB5_16:
	add	x11, x0, x10, lsl #2
	b	LBB5_13
	.cfi_endproc

	.globl	_eager_w64_head_d1
	.p2align	2
_eager_w64_head_d1:
	.cfi_startproc
	cbz	x1, LBB6_3
	stp	x26, x25, [sp, #-64]!
	.cfi_def_cfa_offset 64
	stp	x24, x23, [sp, #16]
	stp	x22, x21, [sp, #32]
	stp	x20, x19, [sp, #48]
	.cfi_offset w19, -8
	.cfi_offset w20, -16
	.cfi_offset w21, -24
	.cfi_offset w22, -32
	.cfi_offset w23, -40
	.cfi_offset w24, -48
	.cfi_offset w25, -56
	.cfi_offset w26, -64
	.cfi_remember_state
	lsl	x9, x1, #4
	sub	x10, x9, #16
	cmp	x10, #48
	b.hs	LBB6_4
	mov	x8, #0
	mov	x11, #0
	mov	x10, x0
	b	LBB6_7
LBB6_3:
	.cfi_def_cfa wsp, 0
	.cfi_same_value w19
	.cfi_same_value w20
	.cfi_same_value w21
	.cfi_same_value w22
	.cfi_same_value w23
	.cfi_same_value w24
	.cfi_same_value w25
	.cfi_same_value w26
	mov	x0, #0
	ret
LBB6_4:
	.cfi_restore_state
	mov	x8, #0
	mov	x11, #0
	mov	x13, #0
	mov	x14, #0
	mov	x15, #0
	mov	x16, #0
	mov	x17, #0
	mov	x1, #0
	lsr	x10, x10, #4
	add	x12, x10, #1
	and	x4, x12, #0x1ffffffffffffffc
	add	x10, x0, x4, lsl #4
	add	x5, x0, #32
	and	x6, x12, #0x1ffffffffffffffc
LBB6_5:
	ldp	x19, x7, [x5, #-32]
	ldp	x21, x20, [x5, #-16]
	ldp	x23, x22, [x5]
	ldp	x25, x24, [x5, #16]
	adds	x19, x19, x2
	adc	x7, x7, x3
	adds	x21, x21, x2
	adc	x20, x20, x3
	adds	x23, x23, x2
	adc	x22, x22, x3
	adds	x25, x25, x2
	adc	x24, x24, x3
	adds	x8, x19, x8
	adc	x11, x7, x11
	adds	x13, x21, x13
	adc	x14, x20, x14
	adds	x15, x23, x15
	adc	x16, x22, x16
	adds	x17, x25, x17
	adc	x1, x24, x1
	add	x5, x5, #64
	subs	x6, x6, #4
	b.ne	LBB6_5
	adds	x8, x13, x8
	adc	x11, x14, x11
	adds	x8, x15, x8
	adc	x11, x16, x11
	adds	x8, x17, x8
	adc	x11, x1, x11
	cmp	x12, x4
	b.eq	LBB6_9
LBB6_7:
	add	x9, x0, x9
LBB6_8:
	ldp	x13, x12, [x10], #16
	adds	x13, x13, x2
	adc	x12, x12, x3
	adds	x8, x13, x8
	adc	x11, x12, x11
	cmp	x10, x9
	b.ne	LBB6_8
LBB6_9:
	ldp	x20, x19, [sp, #48]
	ldp	x22, x21, [sp, #32]
	ldp	x24, x23, [sp, #16]
	ldp	x26, x25, [sp], #64
	.cfi_def_cfa_offset 0
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_eager_w64_head_d8
	.p2align	2
_eager_w64_head_d8:
	.cfi_startproc
	cbz	x1, LBB7_4
	mov	x8, x0
	mov	x0, #0
	mov	x9, #0
	lsl	x10, x1, #4
	mov	x11, #9369
	movk	x11, #37449, lsl #16
	movk	x11, #18724, lsl #32
	movk	x11, #9362, lsl #48
	mov	w12, #3
LBB7_2:
	ldp	x14, x13, [x8], #16
	adds	x14, x14, x2
	adc	x13, x13, x3
	umulh	x15, x14, x12
	add	x13, x13, x13, lsl #1
	add	x13, x15, x13
	add	x14, x14, x14, lsl #1
	subs	x14, x14, x2
	eor	x14, x14, x11
	sbc	x13, x13, x3
	adds	x14, x14, x2
	adc	x13, x13, x3
	add	x13, x13, x13, lsl #1
	umulh	x15, x14, x12
	add	x13, x15, x13
	add	x14, x14, x14, lsl #1
	subs	x14, x14, x2
	eor	x14, x14, x11
	sbc	x13, x13, x3
	adds	x0, x14, x0
	adc	x9, x13, x9
	subs	x10, x10, #16
	b.ne	LBB7_2
	ret
LBB7_4:
	mov	x0, #0
	ret
	.cfi_endproc

	.globl	_eager_w64_min_d1
	.p2align	2
_eager_w64_min_d1:
	.cfi_startproc
	cbz	x1, LBB8_3
	lsl	x10, x1, #3
	sub	x8, x10, #8
	cmp	x8, #56
	b.hs	LBB8_4
	mov	x8, #0
	mov	x9, x0
	b	LBB8_7
LBB8_3:
	mov	x8, #0
	mov	x0, x8
	ret
LBB8_4:
	lsr	x8, x8, #3
	add	x11, x8, #1
	and	x12, x11, #0x3ffffffffffffff8
	add	x9, x0, x12, lsl #3
	dup.2d	v0, x2
	add	x8, x0, #32
	movi.2d	v1, #0000000000000000
	and	x13, x11, #0x3ffffffffffffff8
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
LBB8_5:
	ldp	q5, q6, [x8, #-32]
	ldp	q7, q16, [x8], #64
	add.2d	v5, v0, v5
	add.2d	v6, v0, v6
	add.2d	v7, v0, v7
	add.2d	v16, v0, v16
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	add.2d	v4, v16, v4
	subs	x13, x13, #8
	b.ne	LBB8_5
	add.2d	v0, v2, v1
	add.2d	v0, v3, v0
	add.2d	v0, v4, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x11, x12
	b.eq	LBB8_9
LBB8_7:
	add	x10, x0, x10
LBB8_8:
	ldr	x11, [x9], #8
	add	x8, x2, x8
	add	x8, x8, x11
	cmp	x9, x10
	b.ne	LBB8_8
LBB8_9:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_eager_w64_min_d8
	.p2align	2
_eager_w64_min_d8:
	.cfi_startproc
	cbz	x1, LBB9_3
	stp	x22, x21, [sp, #-32]!
	.cfi_def_cfa_offset 32
	stp	x20, x19, [sp, #16]
	.cfi_offset w19, -8
	.cfi_offset w20, -16
	.cfi_offset w21, -24
	.cfi_offset w22, -32
	.cfi_remember_state
	lsl	x11, x1, #3
	mov	x9, #9369
	movk	x9, #37449, lsl #16
	movk	x9, #18724, lsl #32
	movk	x9, #9362, lsl #48
	sub	x10, x11, #8
	cmp	x10, #24
	b.hs	LBB9_4
	mov	x8, #0
	mov	x10, x0
	b	LBB9_7
LBB9_3:
	.cfi_def_cfa wsp, 0
	.cfi_same_value w19
	.cfi_same_value w20
	.cfi_same_value w21
	.cfi_same_value w22
	mov	x0, #0
	ret
LBB9_4:
	.cfi_restore_state
	mov	x8, #0
	mov	x13, #0
	mov	x14, #0
	mov	x16, #0
	lsr	x10, x10, #3
	add	x12, x10, #1
	and	x15, x12, #0x3ffffffffffffffc
	add	x10, x0, x15, lsl #3
	add	x17, x0, #16
	and	x1, x12, #0x3ffffffffffffffc
LBB9_5:
	ldp	x3, x4, [x17, #-16]
	add	x5, x3, x2
	add	x6, x4, x2
	ldp	x7, x19, [x17], #32
	add	x20, x7, x2
	add	x3, x3, x5, lsl #1
	add	x5, x19, x2
	add	x4, x4, x6, lsl #1
	add	x6, x7, x20, lsl #1
	add	x5, x19, x5, lsl #1
	eor	x3, x3, x9
	eor	x4, x4, x9
	eor	x6, x6, x9
	eor	x5, x5, x9
	add	x7, x3, x2
	add	x19, x4, x2
	add	x20, x6, x2
	add	x21, x5, x2
	add	x3, x3, x7, lsl #1
	add	x4, x4, x19, lsl #1
	add	x6, x6, x20, lsl #1
	add	x5, x5, x21, lsl #1
	eor	x3, x3, x9
	eor	x4, x4, x9
	eor	x6, x6, x9
	eor	x5, x5, x9
	add	x8, x3, x8
	add	x13, x4, x13
	add	x14, x6, x14
	add	x16, x5, x16
	subs	x1, x1, #4
	b.ne	LBB9_5
	add	x8, x13, x8
	add	x13, x16, x14
	add	x8, x13, x8
	cmp	x12, x15
	b.eq	LBB9_9
LBB9_7:
	add	x11, x0, x11
LBB9_8:
	ldr	x12, [x10], #8
	add	x13, x12, x2
	add	x12, x12, x13, lsl #1
	eor	x12, x12, x9
	add	x13, x12, x2
	add	x12, x12, x13, lsl #1
	eor	x12, x12, x9
	add	x8, x12, x8
	cmp	x10, x11
	b.ne	LBB9_8
LBB9_9:
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #32
	.cfi_def_cfa_offset 0
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_ew_eager_w13_head_d4
	.p2align	2
_ew_eager_w13_head_d4:
	.cfi_startproc
	cbz	x2, LBB10_14
	mov	x8, #0
	cmp	x2, #4
	b.lo	LBB10_12
	sub	x9, x1, x0
	cmp	x9, #63
	b.ls	LBB10_12
	dup.4s	v0, w3
	dup.4s	v1, w4
	cmp	x2, #16
	b.hs	LBB10_5
	mov	x8, #0
	b	LBB10_9
LBB10_5:
	and	x9, x2, #0xc
	and	x8, x2, #0xfffffffffffffff0
	add	x10, x0, #32
	add	x11, x1, #32
	movi.4s	v2, #3
	neg.4s	v3, v0
	movi.4s	v4, #31, msl #8
	and	x12, x2, #0xfffffffffffffff0
LBB10_6:
	ldp	q5, q6, [x10, #-32]
	ldp	q7, q16, [x10], #64
	add.4s	v5, v5, v0
	add.4s	v6, v6, v0
	add.4s	v7, v7, v0
	add.4s	v16, v16, v0
	mov.16b	v17, v3
	mla.4s	v17, v5, v2
	mov.16b	v5, v3
	mla.4s	v5, v6, v2
	mov.16b	v6, v3
	mla.4s	v6, v7, v2
	mov.16b	v7, v3
	mla.4s	v7, v16, v2
	eor.16b	v16, v17, v1
	eor.16b	v5, v5, v1
	eor.16b	v6, v6, v1
	eor.16b	v7, v7, v1
	and.16b	v16, v16, v4
	and.16b	v5, v5, v4
	and.16b	v6, v6, v4
	stp	q16, q5, [x11, #-32]
	and.16b	v5, v7, v4
	stp	q6, q5, [x11], #64
	subs	x12, x12, #16
	b.ne	LBB10_6
	cmp	x2, x8
	b.eq	LBB10_14
	cbz	x9, LBB10_12
LBB10_9:
	mov	x10, x8
	and	x8, x2, #0xfffffffffffffffc
	sub	x9, x10, x8
	lsl	x11, x10, #2
	add	x10, x1, x11
	add	x11, x0, x11
	movi.4s	v2, #3
	neg.4s	v3, v0
	movi.4s	v4, #31, msl #8
LBB10_10:
	ldr	q5, [x11], #16
	add.4s	v5, v5, v0
	mov.16b	v6, v3
	mla.4s	v6, v5, v2
	eor.16b	v5, v6, v1
	and.16b	v5, v5, v4
	str	q5, [x10], #16
	adds	x9, x9, #4
	b.ne	LBB10_10
	cmp	x2, x8
	b.eq	LBB10_14
LBB10_12:
	lsl	x10, x8, #2
	add	x9, x1, x10
	add	x10, x0, x10
	sub	x8, x2, x8
LBB10_13:
	ldr	w11, [x10], #4
	add	w12, w11, w3
	add	w11, w11, w12, lsl #1
	eor	w11, w11, w4
	and	w11, w11, #0x1fff
	str	w11, [x9], #4
	subs	x8, x8, #1
	b.ne	LBB10_13
LBB10_14:
	ret
	.cfi_endproc

	.globl	_ew_eager_w13_min_d4
	.p2align	2
_ew_eager_w13_min_d4:
	.cfi_startproc
	cbz	x2, LBB11_14
	mov	x8, #0
	cmp	x2, #4
	b.lo	LBB11_12
	sub	x9, x1, x0
	cmp	x9, #63
	b.ls	LBB11_12
	cmp	x2, #32
	b.hs	LBB11_5
	mov	x8, #0
	b	LBB11_9
LBB11_5:
	and	x9, x2, #0x1c
	dup.8h	v0, w3
	dup.8h	v1, w4
	and	x8, x2, #0xffffffffffffffe0
	add	x10, x0, #32
	add	x11, x1, #32
	movi.8h	v2, #3
	neg.8h	v3, v0
	and	x12, x2, #0xffffffffffffffe0
LBB11_6:
	ldp	q4, q5, [x10, #-32]
	ldp	q6, q7, [x10], #64
	add.8h	v4, v4, v0
	add.8h	v5, v5, v0
	add.8h	v6, v6, v0
	add.8h	v7, v7, v0
	mov.16b	v16, v3
	mla.8h	v16, v4, v2
	mov.16b	v4, v3
	mla.8h	v4, v5, v2
	mov.16b	v5, v3
	mla.8h	v5, v6, v2
	mov.16b	v6, v3
	mla.8h	v6, v7, v2
	eor.16b	v7, v16, v1
	eor.16b	v4, v4, v1
	eor.16b	v5, v5, v1
	eor.16b	v6, v6, v1
	bic.8h	v7, #224, lsl #8
	bic.8h	v4, #224, lsl #8
	bic.8h	v5, #224, lsl #8
	stp	q7, q4, [x11, #-32]
	bic.8h	v6, #224, lsl #8
	stp	q5, q6, [x11], #64
	subs	x12, x12, #32
	b.ne	LBB11_6
	cmp	x2, x8
	b.eq	LBB11_14
	cbz	x9, LBB11_12
LBB11_9:
	mov	x10, x8
	and	x8, x2, #0xfffffffffffffffc
	dup.4h	v0, w3
	dup.4h	v1, w4
	sub	x9, x10, x8
	lsl	x11, x10, #1
	add	x10, x1, x11
	add	x11, x0, x11
	movi.4h	v2, #3
	neg.4h	v3, v0
LBB11_10:
	ldr	d4, [x11], #8
	add.4h	v4, v4, v0
	mov.16b	v5, v3
	mla.4h	v5, v4, v2
	eor.8b	v4, v5, v1
	bic.4h	v4, #224, lsl #8
	str	d4, [x10], #8
	adds	x9, x9, #4
	b.ne	LBB11_10
	cmp	x2, x8
	b.eq	LBB11_14
LBB11_12:
	lsl	x10, x8, #1
	add	x9, x1, x10
	add	x10, x0, x10
	sub	x8, x2, x8
LBB11_13:
	ldrh	w11, [x10], #2
	add	w12, w11, w3
	add	w11, w11, w12, lsl #1
	eor	w11, w11, w4
	and	w11, w11, #0x1fff
	strh	w11, [x9], #2
	subs	x8, x8, #1
	b.ne	LBB11_13
LBB11_14:
	ret
	.cfi_endproc

	.globl	_ew_eager_w64_head_d4
	.p2align	2
_ew_eager_w64_head_d4:
	.cfi_startproc
	cbz	x2, LBB12_2
LBB12_1:
	ldr	x8, [x0], #16
	add	x9, x8, x3
	add	x8, x8, x9, lsl #1
	eor	x8, x8, x5
	stp	x8, xzr, [x1], #16
	subs	x2, x2, #1
	b.ne	LBB12_1
LBB12_2:
	ret
	.cfi_endproc

	.globl	_ew_eager_w64_min_d4
	.p2align	2
_ew_eager_w64_min_d4:
	.cfi_startproc
	cbz	x2, LBB13_2
LBB13_1:
	ldr	x8, [x0], #8
	add	x9, x8, x3
	add	x8, x8, x9, lsl #1
	eor	x8, x8, x4
	str	x8, [x1], #8
	subs	x2, x2, #1
	b.ne	LBB13_1
LBB13_2:
	ret
	.cfi_endproc

	.globl	_ew_lazy_w13_min_d4
	.p2align	2
_ew_lazy_w13_min_d4:
	.cfi_startproc
	cbz	x2, LBB14_14
	mov	x8, #0
	cmp	x2, #4
	b.lo	LBB14_12
	sub	x9, x1, x0
	cmp	x9, #63
	b.ls	LBB14_12
	cmp	x2, #32
	b.hs	LBB14_5
	mov	x8, #0
	b	LBB14_9
LBB14_5:
	and	x9, x2, #0x1c
	dup.8h	v0, w3
	dup.8h	v1, w4
	and	x8, x2, #0xffffffffffffffe0
	add	x10, x0, #32
	add	x11, x1, #32
	movi.8h	v2, #3
	neg.8h	v3, v0
	and	x12, x2, #0xffffffffffffffe0
LBB14_6:
	ldp	q4, q5, [x10, #-32]
	ldp	q6, q7, [x10], #64
	add.8h	v4, v4, v0
	add.8h	v5, v5, v0
	add.8h	v6, v6, v0
	add.8h	v7, v7, v0
	mov.16b	v16, v3
	mla.8h	v16, v4, v2
	mov.16b	v4, v3
	mla.8h	v4, v5, v2
	mov.16b	v5, v3
	mla.8h	v5, v6, v2
	mov.16b	v6, v3
	mla.8h	v6, v7, v2
	eor.16b	v7, v16, v1
	eor.16b	v4, v4, v1
	eor.16b	v5, v5, v1
	eor.16b	v6, v6, v1
	bic.8h	v7, #224, lsl #8
	bic.8h	v4, #224, lsl #8
	bic.8h	v5, #224, lsl #8
	stp	q7, q4, [x11, #-32]
	bic.8h	v6, #224, lsl #8
	stp	q5, q6, [x11], #64
	subs	x12, x12, #32
	b.ne	LBB14_6
	cmp	x2, x8
	b.eq	LBB14_14
	cbz	x9, LBB14_12
LBB14_9:
	mov	x10, x8
	and	x8, x2, #0xfffffffffffffffc
	dup.4h	v0, w3
	dup.4h	v1, w4
	sub	x9, x10, x8
	lsl	x11, x10, #1
	add	x10, x1, x11
	add	x11, x0, x11
	movi.4h	v2, #3
	neg.4h	v3, v0
LBB14_10:
	ldr	d4, [x11], #8
	add.4h	v4, v4, v0
	mov.16b	v5, v3
	mla.4h	v5, v4, v2
	eor.8b	v4, v5, v1
	bic.4h	v4, #224, lsl #8
	str	d4, [x10], #8
	adds	x9, x9, #4
	b.ne	LBB14_10
	cmp	x2, x8
	b.eq	LBB14_14
LBB14_12:
	lsl	x10, x8, #1
	add	x9, x1, x10
	add	x10, x0, x10
	sub	x8, x2, x8
LBB14_13:
	ldrh	w11, [x10], #2
	add	w12, w11, w3
	add	w11, w11, w12, lsl #1
	eor	w11, w11, w4
	and	w11, w11, #0x1fff
	strh	w11, [x9], #2
	subs	x8, x8, #1
	b.ne	LBB14_13
LBB14_14:
	ret
	.cfi_endproc

	.globl	_lazy_w13_head_d1
	.p2align	2
_lazy_w13_head_d1:
	.cfi_startproc
	cbz	x1, LBB15_3
	lsl	x8, x1, #2
	sub	x10, x8, #4
	cmp	x10, #12
	b.hs	LBB15_4
	mov	w12, #0
	mov	x11, x0
	b	LBB15_13
LBB15_3:
	mov	x0, #0
	ret
LBB15_4:
	lsr	x9, x10, #2
	add	x9, x9, #1
	dup.4s	v0, w2
	cmp	x10, #60
	b.hs	LBB15_6
	mov	x10, #0
	mov	w12, #0
	b	LBB15_10
LBB15_6:
	and	x11, x9, #0xc
	and	x10, x9, #0x7ffffffffffffff0
	add	x12, x0, #32
	movi.2d	v1, #0000000000000000
	and	x13, x9, #0x7ffffffffffffff0
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
LBB15_7:
	ldp	q5, q6, [x12, #-32]
	ldp	q7, q16, [x12], #64
	add.4s	v5, v0, v5
	add.4s	v6, v0, v6
	add.4s	v7, v0, v7
	add.4s	v16, v0, v16
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	add.4s	v4, v16, v4
	subs	x13, x13, #16
	b.ne	LBB15_7
	add.4s	v1, v2, v1
	add.4s	v1, v3, v1
	add.4s	v1, v4, v1
	addv.4s	s1, v1
	fmov	w12, s1
	cmp	x9, x10
	b.eq	LBB15_15
	cbz	x11, LBB15_16
LBB15_10:
	and	x13, x9, #0x7ffffffffffffffc
	add	x11, x0, x13, lsl #2
	movi.2d	v1, #0000000000000000
	mov.s	v1[0], w12
	sub	x12, x10, x13
	add	x10, x0, x10, lsl #2
LBB15_11:
	ldr	q2, [x10], #16
	add.4s	v2, v0, v2
	add.4s	v1, v2, v1
	adds	x12, x12, #4
	b.ne	LBB15_11
	addv.4s	s0, v1
	fmov	w12, s0
	cmp	x9, x13
	b.eq	LBB15_15
LBB15_13:
	add	x8, x0, x8
LBB15_14:
	ldr	w9, [x11], #4
	add	w10, w2, w12
	add	w12, w10, w9
	cmp	x11, x8
	b.ne	LBB15_14
LBB15_15:
	and	w0, w12, #0x1fff
	ret
LBB15_16:
	add	x11, x0, x10, lsl #2
	b	LBB15_13
	.cfi_endproc

	.globl	_lazy_w13_head_d8
	.p2align	2
_lazy_w13_head_d8:
	.cfi_startproc
	cbz	x1, LBB16_3
	lsl	x8, x1, #2
	sub	x10, x8, #4
	cmp	x10, #12
	b.hs	LBB16_4
	mov	w12, #0
	mov	x11, x0
	b	LBB16_13
LBB16_3:
	mov	x0, #0
	ret
LBB16_4:
	lsr	x9, x10, #2
	add	x9, x9, #1
	dup.4s	v0, w2
	cmp	x10, #60
	b.hs	LBB16_6
	mov	x10, #0
	mov	w12, #0
	b	LBB16_10
LBB16_6:
	and	x11, x9, #0xc
	and	x10, x9, #0x7ffffffffffffff0
	add	x12, x0, #32
	movi.2d	v1, #0000000000000000
	movi.4s	v2, #3
	neg.4s	v3, v0
	mov	w13, #1177
	dup.4s	v4, w13
	and	x13, x9, #0x7ffffffffffffff0
	movi.2d	v5, #0000000000000000
	movi.2d	v6, #0000000000000000
	movi.2d	v7, #0000000000000000
LBB16_7:
	ldp	q16, q17, [x12, #-32]
	ldp	q18, q19, [x12], #64
	add.4s	v16, v16, v0
	add.4s	v17, v17, v0
	add.4s	v18, v18, v0
	add.4s	v19, v19, v0
	mov.16b	v20, v3
	mla.4s	v20, v16, v2
	mov.16b	v16, v3
	mla.4s	v16, v17, v2
	mov.16b	v17, v3
	mla.4s	v17, v18, v2
	mov.16b	v18, v3
	mla.4s	v18, v19, v2
	eor.16b	v19, v20, v4
	eor.16b	v16, v16, v4
	eor.16b	v17, v17, v4
	eor.16b	v18, v18, v4
	add.4s	v19, v19, v0
	add.4s	v16, v16, v0
	add.4s	v17, v17, v0
	add.4s	v18, v18, v0
	mov.16b	v20, v3
	mla.4s	v20, v19, v2
	mov.16b	v19, v3
	mla.4s	v19, v16, v2
	mov.16b	v16, v3
	mla.4s	v16, v17, v2
	mov.16b	v17, v3
	mla.4s	v17, v18, v2
	eor.16b	v18, v20, v4
	eor.16b	v19, v19, v4
	eor.16b	v16, v16, v4
	eor.16b	v17, v17, v4
	add.4s	v1, v18, v1
	add.4s	v5, v19, v5
	add.4s	v6, v16, v6
	add.4s	v7, v17, v7
	subs	x13, x13, #16
	b.ne	LBB16_7
	add.4s	v1, v5, v1
	add.4s	v1, v6, v1
	add.4s	v1, v7, v1
	addv.4s	s1, v1
	fmov	w12, s1
	cmp	x9, x10
	b.eq	LBB16_15
	cbz	x11, LBB16_16
LBB16_10:
	and	x13, x9, #0x7ffffffffffffffc
	add	x11, x0, x13, lsl #2
	movi.2d	v1, #0000000000000000
	mov.s	v1[0], w12
	sub	x12, x10, x13
	add	x10, x0, x10, lsl #2
	movi.4s	v2, #3
	neg.4s	v3, v0
	mov	w14, #1177
	dup.4s	v4, w14
LBB16_11:
	ldr	q5, [x10], #16
	add.4s	v5, v5, v0
	mov.16b	v6, v3
	mla.4s	v6, v5, v2
	eor.16b	v5, v6, v4
	add.4s	v5, v5, v0
	mov.16b	v6, v3
	mla.4s	v6, v5, v2
	eor.16b	v5, v6, v4
	add.4s	v1, v5, v1
	adds	x12, x12, #4
	b.ne	LBB16_11
	addv.4s	s0, v1
	fmov	w12, s0
	cmp	x9, x13
	b.eq	LBB16_15
LBB16_13:
	add	x8, x0, x8
	mov	w9, #1177
LBB16_14:
	ldr	w10, [x11], #4
	add	w13, w10, w2
	add	w10, w10, w13, lsl #1
	eor	w10, w10, w9
	add	w13, w10, w2
	add	w10, w10, w13, lsl #1
	eor	w10, w10, w9
	add	w12, w10, w12
	cmp	x11, x8
	b.ne	LBB16_14
LBB16_15:
	and	w0, w12, #0x1fff
	ret
LBB16_16:
	add	x11, x0, x10, lsl #2
	b	LBB16_13
	.cfi_endproc

	.globl	_lazy_w13_min_d1
	.p2align	2
_lazy_w13_min_d1:
	.cfi_startproc
	cbz	x1, LBB17_3
	lsl	x8, x1, #1
	sub	x10, x8, #2
	cmp	x10, #6
	b.hs	LBB17_4
	mov	w12, #0
	mov	x11, x0
	b	LBB17_13
LBB17_3:
	mov	x0, #0
	ret
LBB17_4:
	lsr	x9, x10, #1
	add	x9, x9, #1
	cmp	x10, #62
	b.hs	LBB17_6
	mov	x10, #0
	mov	w12, #0
	b	LBB17_10
LBB17_6:
	and	x11, x9, #0x1c
	and	x10, x9, #0xffffffffffffffe0
	dup.8h	v0, w2
	add	x12, x0, #32
	movi.2d	v1, #0000000000000000
	and	x13, x9, #0xffffffffffffffe0
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
LBB17_7:
	ldp	q5, q6, [x12, #-32]
	ldp	q7, q16, [x12], #64
	add.8h	v5, v0, v5
	add.8h	v6, v0, v6
	add.8h	v7, v0, v7
	add.8h	v16, v0, v16
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
	add.8h	v4, v16, v4
	subs	x13, x13, #32
	b.ne	LBB17_7
	add.8h	v0, v2, v1
	add.8h	v0, v3, v0
	add.8h	v0, v4, v0
	addv.8h	h0, v0
	fmov	w12, s0
	cmp	x9, x10
	b.eq	LBB17_15
	cbz	x11, LBB17_16
LBB17_10:
	and	x13, x9, #0xfffffffffffffffc
	add	x11, x0, x13, lsl #1
	movi.2d	v0, #0000000000000000
	mov.h	v0[0], w12
	dup.4h	v1, w2
	sub	x12, x10, x13
	add	x10, x0, x10, lsl #1
LBB17_11:
	ldr	d2, [x10], #8
	add.4h	v2, v1, v2
	add.4h	v0, v2, v0
	adds	x12, x12, #4
	b.ne	LBB17_11
	addv.4h	h0, v0
	fmov	w12, s0
	cmp	x9, x13
	b.eq	LBB17_15
LBB17_13:
	add	x8, x0, x8
LBB17_14:
	ldrh	w9, [x11], #2
	add	w10, w2, w12
	add	w12, w10, w9
	cmp	x11, x8
	b.ne	LBB17_14
LBB17_15:
	and	x0, x12, #0x1fff
	ret
LBB17_16:
	add	x11, x0, x10, lsl #1
	b	LBB17_13
	.cfi_endproc

	.globl	_lazy_w13_min_d8
	.p2align	2
_lazy_w13_min_d8:
	.cfi_startproc
	cbz	x1, LBB18_3
	lsl	x8, x1, #1
	sub	x10, x8, #2
	cmp	x10, #6
	b.hs	LBB18_4
	mov	w12, #0
	mov	x11, x0
	b	LBB18_13
LBB18_3:
	mov	x0, #0
	ret
LBB18_4:
	lsr	x9, x10, #1
	add	x9, x9, #1
	cmp	x10, #62
	b.hs	LBB18_6
	mov	x10, #0
	mov	w12, #0
	b	LBB18_10
LBB18_6:
	and	x11, x9, #0x1c
	dup.8h	v0, w2
	and	x10, x9, #0xffffffffffffffe0
	add	x12, x0, #32
	movi.2d	v1, #0000000000000000
	movi.8h	v2, #3
	neg.8h	v3, v0
	mov	w13, #1177
	dup.8h	v4, w13
	and	x13, x9, #0xffffffffffffffe0
	movi.2d	v5, #0000000000000000
	movi.2d	v6, #0000000000000000
	movi.2d	v7, #0000000000000000
LBB18_7:
	ldp	q16, q17, [x12, #-32]
	ldp	q18, q19, [x12], #64
	add.8h	v16, v16, v0
	add.8h	v17, v17, v0
	add.8h	v18, v18, v0
	add.8h	v19, v19, v0
	mov.16b	v20, v3
	mla.8h	v20, v16, v2
	mov.16b	v16, v3
	mla.8h	v16, v17, v2
	mov.16b	v17, v3
	mla.8h	v17, v18, v2
	mov.16b	v18, v3
	mla.8h	v18, v19, v2
	eor.16b	v19, v20, v4
	eor.16b	v16, v16, v4
	eor.16b	v17, v17, v4
	eor.16b	v18, v18, v4
	add.8h	v19, v19, v0
	add.8h	v16, v16, v0
	add.8h	v17, v17, v0
	add.8h	v18, v18, v0
	mov.16b	v20, v3
	mla.8h	v20, v19, v2
	mov.16b	v19, v3
	mla.8h	v19, v16, v2
	mov.16b	v16, v3
	mla.8h	v16, v17, v2
	mov.16b	v17, v3
	mla.8h	v17, v18, v2
	eor.16b	v18, v20, v4
	eor.16b	v19, v19, v4
	eor.16b	v16, v16, v4
	eor.16b	v17, v17, v4
	add.8h	v1, v18, v1
	add.8h	v5, v19, v5
	add.8h	v6, v16, v6
	add.8h	v7, v17, v7
	subs	x13, x13, #32
	b.ne	LBB18_7
	add.8h	v0, v5, v1
	add.8h	v0, v6, v0
	add.8h	v0, v7, v0
	addv.8h	h0, v0
	fmov	w12, s0
	cmp	x9, x10
	b.eq	LBB18_15
	cbz	x11, LBB18_16
LBB18_10:
	and	x13, x9, #0xfffffffffffffffc
	add	x11, x0, x13, lsl #1
	movi.2d	v0, #0000000000000000
	mov.h	v0[0], w12
	dup.4h	v1, w2
	sub	x12, x10, x13
	add	x10, x0, x10, lsl #1
	movi.4h	v2, #3
	neg.4h	v3, v1
	mov	w14, #1177
	dup.4h	v4, w14
LBB18_11:
	ldr	d5, [x10], #8
	add.4h	v5, v5, v1
	mov.16b	v6, v3
	mla.4h	v6, v5, v2
	eor.8b	v5, v6, v4
	add.4h	v5, v5, v1
	mov.16b	v6, v3
	mla.4h	v6, v5, v2
	eor.8b	v5, v6, v4
	add.4h	v0, v5, v0
	adds	x12, x12, #4
	b.ne	LBB18_11
	addv.4h	h0, v0
	fmov	w12, s0
	cmp	x9, x13
	b.eq	LBB18_15
LBB18_13:
	add	x8, x0, x8
	mov	w9, #1177
LBB18_14:
	ldrh	w10, [x11], #2
	add	w13, w10, w2
	add	w10, w10, w13, lsl #1
	eor	w10, w10, w9
	add	w13, w10, w2
	add	w10, w10, w13, lsl #1
	eor	w10, w10, w9
	add	w12, w10, w12
	cmp	x11, x8
	b.ne	LBB18_14
LBB18_15:
	and	x0, x12, #0x1fff
	ret
LBB18_16:
	add	x11, x0, x10, lsl #1
	b	LBB18_13
	.cfi_endproc

	.globl	_lazy_w32_head_d3
	.p2align	2
_lazy_w32_head_d3:
	.cfi_startproc
	cbz	x1, LBB19_3
	stp	x20, x19, [sp, #-16]!
	.cfi_def_cfa_offset 16
	.cfi_offset w19, -8
	.cfi_offset w20, -16
	.cfi_remember_state
	lsl	x9, x1, #3
	sub	x8, x9, #8
	cmp	x8, #24
	b.hs	LBB19_4
	mov	x10, #0
	mov	x8, x0
	b	LBB19_7
LBB19_3:
	.cfi_def_cfa wsp, 0
	.cfi_same_value w19
	.cfi_same_value w20
	mov	x0, #0
	ret
LBB19_4:
	.cfi_restore_state
	mov	x10, #0
	mov	x12, #0
	mov	x13, #0
	mov	x15, #0
	lsr	x8, x8, #3
	add	x11, x8, #1
	and	x14, x11, #0x3ffffffffffffffc
	add	x8, x0, x14, lsl #3
	add	x16, x0, #16
	and	x17, x11, #0x3ffffffffffffffc
LBB19_5:
	ldp	x1, x3, [x16, #-16]
	add	x4, x1, x2
	ldp	x5, x6, [x16], #32
	add	x7, x3, x2
	add	x19, x5, x2
	add	x1, x1, x4, lsl #1
	add	x4, x6, x2
	add	x3, x3, x7, lsl #1
	add	x5, x5, x19, lsl #1
	add	x4, x6, x4, lsl #1
	add	x10, x1, x10
	add	x12, x3, x12
	add	x13, x5, x13
	add	x15, x4, x15
	subs	x17, x17, #4
	b.ne	LBB19_5
	add	x10, x12, x10
	add	x12, x15, x13
	add	x10, x12, x10
	cmp	x11, x14
	b.eq	LBB19_9
LBB19_7:
	add	x9, x0, x9
LBB19_8:
	ldr	x11, [x8], #8
	add	x12, x11, x2
	add	x11, x11, x12, lsl #1
	add	x10, x11, x10
	cmp	x8, x9
	b.ne	LBB19_8
LBB19_9:
	mov	w0, w10
	ldp	x20, x19, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w19
	.cfi_restore w20
	ret
	.cfi_endproc

	.globl	_lazy_w64_head_d1
	.p2align	2
_lazy_w64_head_d1:
	.cfi_startproc
	cbz	x1, LBB20_3
	stp	x26, x25, [sp, #-64]!
	.cfi_def_cfa_offset 64
	stp	x24, x23, [sp, #16]
	stp	x22, x21, [sp, #32]
	stp	x20, x19, [sp, #48]
	.cfi_offset w19, -8
	.cfi_offset w20, -16
	.cfi_offset w21, -24
	.cfi_offset w22, -32
	.cfi_offset w23, -40
	.cfi_offset w24, -48
	.cfi_offset w25, -56
	.cfi_offset w26, -64
	.cfi_remember_state
	lsl	x10, x1, #4
	sub	x9, x10, #16
	cmp	x9, #48
	b.hs	LBB20_4
	mov	x8, #0
	mov	x11, #0
	mov	x9, x0
	b	LBB20_7
LBB20_3:
	.cfi_def_cfa wsp, 0
	.cfi_same_value w19
	.cfi_same_value w20
	.cfi_same_value w21
	.cfi_same_value w22
	.cfi_same_value w23
	.cfi_same_value w24
	.cfi_same_value w25
	.cfi_same_value w26
	mov	x0, #0
	ret
LBB20_4:
	.cfi_restore_state
	mov	x8, #0
	mov	x11, #0
	mov	x13, #0
	mov	x14, #0
	mov	x15, #0
	mov	x16, #0
	mov	x17, #0
	mov	x1, #0
	lsr	x9, x9, #4
	add	x12, x9, #1
	and	x4, x12, #0x1ffffffffffffffc
	add	x9, x0, x4, lsl #4
	add	x5, x0, #32
	and	x6, x12, #0x1ffffffffffffffc
LBB20_5:
	ldp	x19, x7, [x5, #-32]
	ldp	x21, x20, [x5, #-16]
	ldp	x23, x22, [x5]
	ldp	x25, x24, [x5, #16]
	adds	x19, x2, x19
	adc	x7, x3, x7
	adds	x21, x2, x21
	adc	x20, x3, x20
	adds	x23, x2, x23
	adc	x22, x3, x22
	adds	x25, x2, x25
	adc	x24, x3, x24
	adds	x8, x19, x8
	adc	x11, x7, x11
	adds	x13, x21, x13
	adc	x14, x20, x14
	adds	x15, x23, x15
	adc	x16, x22, x16
	adds	x17, x25, x17
	adc	x1, x24, x1
	add	x5, x5, #64
	subs	x6, x6, #4
	b.ne	LBB20_5
	adds	x8, x13, x8
	adc	x11, x14, x11
	adds	x8, x15, x8
	adc	x11, x16, x11
	adds	x8, x17, x8
	adc	x11, x1, x11
	cmp	x12, x4
	b.eq	LBB20_9
LBB20_7:
	add	x10, x0, x10
LBB20_8:
	ldp	x13, x12, [x9], #16
	adds	x13, x2, x13
	adc	x12, x3, x12
	adds	x8, x13, x8
	adc	x11, x12, x11
	cmp	x9, x10
	b.ne	LBB20_8
LBB20_9:
	ldp	x20, x19, [sp, #48]
	ldp	x22, x21, [sp, #32]
	ldp	x24, x23, [sp, #16]
	ldp	x26, x25, [sp], #64
	.cfi_def_cfa_offset 0
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_op_addxor_u16
	.p2align	2
_op_addxor_u16:
	.cfi_startproc
	cbz	x2, LBB21_14
	mov	x8, #0
	cmp	x2, #4
	b.lo	LBB21_12
	sub	x9, x1, x0
	cmp	x9, #63
	b.ls	LBB21_12
	cmp	x2, #32
	b.hs	LBB21_5
	mov	x8, #0
	b	LBB21_9
LBB21_5:
	and	x9, x2, #0x1c
	and	x8, x2, #0xffffffffffffffe0
	dup.8h	v0, w3
	add	x10, x0, #32
	add	x11, x1, #32
	and	x12, x2, #0xffffffffffffffe0
LBB21_6:
	ldp	q1, q2, [x10, #-32]
	ldp	q3, q4, [x10], #64
	add.8h	v1, v1, v0
	add.8h	v2, v2, v0
	add.8h	v3, v3, v0
	add.8h	v4, v4, v0
	eor.16b	v1, v1, v0
	eor.16b	v2, v2, v0
	eor.16b	v3, v3, v0
	eor.16b	v4, v4, v0
	stp	q1, q2, [x11, #-32]
	stp	q3, q4, [x11], #64
	subs	x12, x12, #32
	b.ne	LBB21_6
	cmp	x2, x8
	b.eq	LBB21_14
	cbz	x9, LBB21_12
LBB21_9:
	mov	x10, x8
	and	x8, x2, #0xfffffffffffffffc
	dup.4h	v0, w3
	sub	x9, x10, x8
	lsl	x11, x10, #1
	add	x10, x1, x11
	add	x11, x0, x11
LBB21_10:
	ldr	d1, [x11], #8
	add.4h	v1, v1, v0
	eor.8b	v1, v1, v0
	str	d1, [x10], #8
	adds	x9, x9, #4
	b.ne	LBB21_10
	cmp	x2, x8
	b.eq	LBB21_14
LBB21_12:
	lsl	x10, x8, #1
	add	x9, x1, x10
	add	x10, x0, x10
	sub	x8, x2, x8
LBB21_13:
	ldrh	w11, [x10], #2
	add	w11, w11, w3
	eor	w11, w11, w3
	strh	w11, [x9], #2
	subs	x8, x8, #1
	b.ne	LBB21_13
LBB21_14:
	ret
	.cfi_endproc

	.globl	_op_addxor_u64
	.p2align	2
_op_addxor_u64:
	.cfi_startproc
	cbz	x2, LBB22_8
	mov	x8, #0
	cmp	x2, #8
	b.lo	LBB22_6
	sub	x9, x1, x0
	cmp	x9, #63
	b.ls	LBB22_6
	and	x8, x2, #0xfffffffffffffff8
	dup.2d	v0, x3
	add	x9, x0, #32
	add	x10, x1, #32
	and	x11, x2, #0xfffffffffffffff8
LBB22_4:
	ldp	q1, q2, [x9, #-32]
	ldp	q3, q4, [x9], #64
	add.2d	v1, v1, v0
	add.2d	v2, v2, v0
	add.2d	v3, v3, v0
	add.2d	v4, v4, v0
	eor.16b	v1, v1, v0
	eor.16b	v2, v2, v0
	eor.16b	v3, v3, v0
	eor.16b	v4, v4, v0
	stp	q1, q2, [x10, #-32]
	stp	q3, q4, [x10], #64
	subs	x11, x11, #8
	b.ne	LBB22_4
	cmp	x2, x8
	b.eq	LBB22_8
LBB22_6:
	lsl	x10, x8, #3
	add	x9, x1, x10
	add	x10, x0, x10
	sub	x8, x2, x8
LBB22_7:
	ldr	x11, [x10], #8
	add	x11, x11, x3
	eor	x11, x11, x3
	str	x11, [x9], #8
	subs	x8, x8, #1
	b.ne	LBB22_7
LBB22_8:
	ret
	.cfi_endproc

	.globl	_op_mul3_u16
	.p2align	2
_op_mul3_u16:
	.cfi_startproc
	cbz	x2, LBB23_14
	mov	x8, #0
	cmp	x2, #4
	b.lo	LBB23_12
	sub	x9, x1, x0
	cmp	x9, #63
	b.ls	LBB23_12
	cmp	x2, #32
	b.hs	LBB23_5
	mov	x8, #0
	b	LBB23_9
LBB23_5:
	and	x9, x2, #0x1c
	and	x8, x2, #0xffffffffffffffe0
	add	x10, x0, #32
	add	x11, x1, #32
	movi.8h	v0, #3
	and	x12, x2, #0xffffffffffffffe0
LBB23_6:
	ldp	q1, q2, [x10, #-32]
	ldp	q3, q4, [x10], #64
	mul.8h	v1, v1, v0
	mul.8h	v2, v2, v0
	mul.8h	v3, v3, v0
	mul.8h	v4, v4, v0
	stp	q1, q2, [x11, #-32]
	stp	q3, q4, [x11], #64
	subs	x12, x12, #32
	b.ne	LBB23_6
	cmp	x2, x8
	b.eq	LBB23_14
	cbz	x9, LBB23_12
LBB23_9:
	mov	x10, x8
	and	x8, x2, #0xfffffffffffffffc
	sub	x9, x10, x8
	lsl	x11, x10, #1
	add	x10, x1, x11
	add	x11, x0, x11
	movi.4h	v0, #3
LBB23_10:
	ldr	d1, [x11], #8
	mul.4h	v1, v1, v0
	str	d1, [x10], #8
	adds	x9, x9, #4
	b.ne	LBB23_10
	cmp	x2, x8
	b.eq	LBB23_14
LBB23_12:
	lsl	x10, x8, #1
	add	x9, x1, x10
	add	x10, x0, x10
	sub	x8, x2, x8
LBB23_13:
	ldrh	w11, [x10], #2
	add	w11, w11, w11, lsl #1
	strh	w11, [x9], #2
	subs	x8, x8, #1
	b.ne	LBB23_13
LBB23_14:
	ret
	.cfi_endproc

	.globl	_op_mul3_u64
	.p2align	2
_op_mul3_u64:
	.cfi_startproc
	cbz	x2, LBB24_8
	and	x8, x2, #0x7
	cmp	x2, #8
	b.hs	LBB24_3
	mov	x9, #0
	b	LBB24_6
LBB24_3:
	mov	x9, #0
	and	x10, x2, #0xfffffffffffffff8
	add	x11, x0, #32
	add	x12, x1, #32
LBB24_4:
	ldur	x13, [x11, #-32]
	add	x13, x13, x13, lsl #1
	stur	x13, [x12, #-32]
	ldur	x13, [x11, #-24]
	add	x13, x13, x13, lsl #1
	stur	x13, [x12, #-24]
	ldur	x13, [x11, #-16]
	add	x13, x13, x13, lsl #1
	stur	x13, [x12, #-16]
	ldur	x13, [x11, #-8]
	add	x13, x13, x13, lsl #1
	stur	x13, [x12, #-8]
	ldr	x13, [x11]
	add	x13, x13, x13, lsl #1
	str	x13, [x12]
	ldr	x13, [x11, #8]
	add	x13, x13, x13, lsl #1
	str	x13, [x12, #8]
	ldr	x13, [x11, #16]
	add	x13, x13, x13, lsl #1
	str	x13, [x12, #16]
	add	x9, x9, #8
	ldr	x13, [x11, #24]
	add	x13, x13, x13, lsl #1
	str	x13, [x12, #24]
	add	x11, x11, #64
	add	x12, x12, #64
	cmp	x10, x9
	b.ne	LBB24_4
	cbz	x8, LBB24_8
LBB24_6:
	lsl	x10, x9, #3
	add	x9, x1, x10
	add	x10, x0, x10
LBB24_7:
	ldr	x11, [x10], #8
	add	x11, x11, x11, lsl #1
	str	x11, [x9], #8
	subs	x8, x8, #1
	b.ne	LBB24_7
LBB24_8:
	ret
	.cfi_endproc

	.globl	_op_mulk_u16
	.p2align	2
_op_mulk_u16:
	.cfi_startproc
	cbz	x2, LBB25_14
	mov	x8, #0
	cmp	x2, #4
	b.lo	LBB25_12
	sub	x9, x1, x0
	cmp	x9, #63
	b.ls	LBB25_12
	cmp	x2, #32
	b.hs	LBB25_5
	mov	x8, #0
	b	LBB25_9
LBB25_5:
	and	x9, x2, #0x1c
	and	x8, x2, #0xffffffffffffffe0
	fmov	s0, w3
	add	x10, x0, #32
	add	x11, x1, #32
	and	x12, x2, #0xffffffffffffffe0
LBB25_6:
	ldp	q1, q2, [x10, #-32]
	ldp	q3, q4, [x10], #64
	mul.8h	v1, v1, v0[0]
	mul.8h	v2, v2, v0[0]
	mul.8h	v3, v3, v0[0]
	mul.8h	v4, v4, v0[0]
	stp	q1, q2, [x11, #-32]
	stp	q3, q4, [x11], #64
	subs	x12, x12, #32
	b.ne	LBB25_6
	cmp	x2, x8
	b.eq	LBB25_14
	cbz	x9, LBB25_12
LBB25_9:
	mov	x10, x8
	and	x8, x2, #0xfffffffffffffffc
	fmov	s0, w3
	sub	x9, x10, x8
	lsl	x11, x10, #1
	add	x10, x1, x11
	add	x11, x0, x11
LBB25_10:
	ldr	d1, [x11], #8
	mul.4h	v1, v1, v0[0]
	str	d1, [x10], #8
	adds	x9, x9, #4
	b.ne	LBB25_10
	cmp	x2, x8
	b.eq	LBB25_14
LBB25_12:
	lsl	x10, x8, #1
	add	x9, x1, x10
	add	x10, x0, x10
	sub	x8, x2, x8
LBB25_13:
	ldrh	w11, [x10], #2
	mul	w11, w11, w3
	strh	w11, [x9], #2
	subs	x8, x8, #1
	b.ne	LBB25_13
LBB25_14:
	ret
	.cfi_endproc

	.globl	_op_mulk_u64
	.p2align	2
_op_mulk_u64:
	.cfi_startproc
	cbz	x2, LBB26_8
	and	x8, x2, #0x7
	cmp	x2, #8
	b.hs	LBB26_3
	mov	x9, #0
	b	LBB26_6
LBB26_3:
	mov	x9, #0
	and	x10, x2, #0xfffffffffffffff8
	add	x11, x0, #32
	add	x12, x1, #32
LBB26_4:
	ldur	x13, [x11, #-32]
	mul	x13, x13, x3
	stur	x13, [x12, #-32]
	ldur	x13, [x11, #-24]
	mul	x13, x13, x3
	stur	x13, [x12, #-24]
	ldur	x13, [x11, #-16]
	mul	x13, x13, x3
	stur	x13, [x12, #-16]
	ldur	x13, [x11, #-8]
	mul	x13, x13, x3
	stur	x13, [x12, #-8]
	ldr	x13, [x11]
	mul	x13, x13, x3
	str	x13, [x12]
	ldr	x13, [x11, #8]
	mul	x13, x13, x3
	str	x13, [x12, #8]
	ldr	x13, [x11, #16]
	mul	x13, x13, x3
	str	x13, [x12, #16]
	add	x9, x9, #8
	ldr	x13, [x11, #24]
	mul	x13, x13, x3
	str	x13, [x12, #24]
	add	x11, x11, #64
	add	x12, x12, #64
	cmp	x10, x9
	b.ne	LBB26_4
	cbz	x8, LBB26_8
LBB26_6:
	lsl	x10, x9, #3
	add	x9, x1, x10
	add	x10, x0, x10
LBB26_7:
	ldr	x11, [x10], #8
	mul	x11, x11, x3
	str	x11, [x9], #8
	subs	x8, x8, #1
	b.ne	LBB26_7
LBB26_8:
	ret
	.cfi_endproc

	.globl	_op_mulk_u64_neon
	.p2align	2
_op_mulk_u64_neon:
	.cfi_startproc
	lsr	x12, x2, #1
	cbz	x12, LBB27_8
	mov	x11, #0
	dup.2d	v0, x3
	sub	x8, x12, #1
	mov	x9, #-3689348814741910324
	movk	x9, #52429
	umulh	x9, x8, x9
	lsr	x9, x9, #2
	add	x9, x9, x9, lsl #2
	sub	x8, x8, x9
	add	x10, x8, #1
	cmp	x10, #5
	csinc	x8, xzr, x8, eq
	mov.d	x9, v0[1]
	cmp	x2, #10
	b.lo	LBB27_6
	mov	x11, #0
	sub	x12, x8, x12
	add	x13, x0, #32
	add	x14, x1, #32
	fmov	x15, d0
LBB27_3:
	ldp	x16, x17, [x13, #-32]
	mul	x17, x17, x9
	mul	x16, x16, x15
	fmov	d1, x16
	mov.d	v1[1], x17
	stur	q1, [x14, #-32]
	ldp	x16, x17, [x13, #-16]
	mul	x17, x17, x9
	mul	x16, x16, x15
	fmov	d1, x16
	mov.d	v1[1], x17
	stur	q1, [x14, #-16]
	ldp	x16, x17, [x13]
	mul	x17, x17, x9
	mul	x16, x16, x15
	fmov	d1, x16
	mov.d	v1[1], x17
	str	q1, [x14]
	ldp	x16, x17, [x13, #16]
	mul	x17, x17, x9
	mul	x16, x16, x15
	fmov	d1, x16
	mov.d	v1[1], x17
	str	q1, [x14, #16]
	ldp	x16, x17, [x13, #32]
	mul	x17, x17, x9
	mul	x16, x16, x15
	fmov	d1, x16
	mov.d	v1[1], x17
	str	q1, [x14, #32]
	sub	x11, x11, #5
	add	x13, x13, #80
	add	x14, x14, #80
	cmp	x12, x11
	b.ne	LBB27_3
	cmp	x10, #5
	b.eq	LBB27_8
	neg	x11, x11
LBB27_6:
	lsl	x11, x11, #4
	add	x10, x1, x11
	add	x13, x0, x11
	fmov	x11, d0
	mov	x12, x13
LBB27_7:
	ldr	x14, [x12], #16
	ldr	x13, [x13, #8]
	mul	x13, x13, x9
	mul	x14, x14, x11
	fmov	d0, x14
	mov.d	v0[1], x13
	str	q0, [x10], #16
	mov	x13, x12
	subs	x8, x8, #1
	b.ne	LBB27_7
LBB27_8:
	and	x8, x2, #0xfffffffffffffffe
	cmp	x8, x2
	b.eq	LBB27_10
LBB27_9:
	ldr	x9, [x0, x8, lsl #3]
	mul	x9, x9, x3
	str	x9, [x1, x8, lsl #3]
	add	x8, x8, #1
	cmp	x8, x2
	b.lo	LBB27_9
LBB27_10:
	ret
	.cfi_endproc

	.globl	_sat_w13_head_d3
	.p2align	2
_sat_w13_head_d3:
	.cfi_startproc
	cbz	x1, LBB28_4
	mov	x8, x0
	mov	w0, #0
	lsl	x9, x1, #2
	mov	w10, #8191
LBB28_2:
	ldr	w11, [x8], #4
	adds	w11, w11, w2
	csinv	w11, w11, wzr, lo
	cmp	w11, w10
	csel	w11, w11, w10, lo
	subs	w11, w11, w2
	csel	w11, wzr, w11, lo
	adds	w11, w11, w2
	csinv	w11, w11, wzr, lo
	cmp	w11, w10
	csel	w11, w11, w10, lo
	add	w11, w0, w11
	cmp	w11, w10
	csel	w0, w11, w10, lo
	subs	x9, x9, #4
	b.ne	LBB28_2
	ret
LBB28_4:
	mov	x0, #0
	ret
	.cfi_endproc

	.globl	_sat_w13_min_d3
	.p2align	2
_sat_w13_min_d3:
	.cfi_startproc
	cbz	x1, LBB29_4
	mov	x8, x0
	mov	w0, #0
	lsl	x9, x1, #1
	mov	w10, #8191
LBB29_2:
	ldrh	w11, [x8], #2
	add	w11, w11, w2
	cmp	w11, w10
	csel	w11, w11, w10, lo
	subs	w11, w11, w2
	csel	w11, wzr, w11, lo
	add	w11, w11, w2
	cmp	w11, w10
	csel	w11, w11, w10, lo
	add	w11, w0, w11
	and	w11, w11, #0xffff
	cmp	w11, w10
	csel	w0, w11, w10, lo
	subs	x9, x9, #2
	b.ne	LBB29_2
	ret
LBB29_4:
	mov	x0, #0
	ret
	.cfi_endproc

	.globl	_sat_w64_head_d3
	.p2align	2
_sat_w64_head_d3:
	.cfi_startproc
	cbz	x1, LBB30_4
	mov	x8, x0
	mov	x0, #0
	mov	x10, #0
	lsl	x9, x1, #4
LBB30_2:
	ldp	x12, x11, [x8], #16
	adds	x12, x12, x2
	adcs	x11, x11, x3
	csinv	x12, x12, xzr, lo
	csinv	x11, x11, xzr, lo
	cmp	x11, #0
	csinv	x11, x12, xzr, eq
	subs	x11, x11, x2
	ngcs	x12, x3
	csel	x12, xzr, x12, lo
	csel	x11, xzr, x11, lo
	adds	x11, x11, x2
	adcs	x12, x12, x3
	csinv	x12, x12, xzr, lo
	csinv	x11, x11, xzr, lo
	cmp	x12, #0
	csinv	x11, x11, xzr, eq
	adds	x11, x0, x11
	cinc	x10, x10, hs
	cmp	x10, #0
	csinv	x0, x11, xzr, eq
	mov	x10, #0
	subs	x9, x9, #16
	b.ne	LBB30_2
	ret
LBB30_4:
	mov	x0, #0
	ret
	.cfi_endproc

	.globl	_sat_w64_min_d3
	.p2align	2
_sat_w64_min_d3:
	.cfi_startproc
	mov	x8, #0
	cbz	x1, LBB31_3
	lsl	x9, x1, #3
LBB31_2:
	ldr	x10, [x0], #8
	adds	x10, x10, x2
	csinv	x10, x10, xzr, lo
	subs	x10, x10, x2
	csel	x10, xzr, x10, lo
	adds	x10, x10, x2
	csinv	x10, x10, xzr, lo
	adds	x8, x8, x10
	csinv	x8, x8, xzr, lo
	subs	x9, x9, #8
	b.ne	LBB31_2
LBB31_3:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_op_mulk_u64_split
_op_mulk_u64_split = _op_mulk_u64
	.globl	_lazy_w64_min_d1
_lazy_w64_min_d1 = _eager_w64_min_d1
	.globl	_lazy_w64_head_d8
_lazy_w64_head_d8 = _eager_w64_head_d8
	.globl	_lazy_w32_min_d3
_lazy_w32_min_d3 = _eager_w32_min_d3
	.globl	_lazy_w64_min_d8
_lazy_w64_min_d8 = _eager_w64_min_d8
.subsections_via_symbols
