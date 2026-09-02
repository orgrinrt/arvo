	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCsltXhV8SboNZ_15f_where_carried12arm_wide_sat
	.p2align	2
__RNvCsltXhV8SboNZ_15f_where_carried12arm_wide_sat:
	cbz	x1, LBB0_4
	lsl	x8, x1, #2
	subs	x10, x8, #4
	b.eq	LBB0_5
	lsr	x9, x10, #2
	add	x9, x9, #1
	cmp	x10, #60
	b.hs	LBB0_6
	mov	x10, #0
	mov	x11, #0
	b	LBB0_10
LBB0_4:
	mov	x11, #0
	mov	w8, w2
	cmp	x11, x8
	csel	x0, x11, x8, lo
	ret
LBB0_5:
	mov	x11, #0
	mov	x12, x0
	b	LBB0_13
LBB0_6:
	and	x12, x9, #0xe
	and	x10, x9, #0x7ffffffffffffff0
	add	x11, x0, #32
	movi.2d	v0, #0000000000000000
	and	x13, x9, #0x7ffffffffffffff0
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB0_7:
	ldp	q4, q5, [x11, #-32]
	ldp	q6, q7, [x11], #64
	uaddw.2d	v0, v0, v4
	uaddw2.2d	v0, v0, v4
	uaddw.2d	v1, v1, v5
	uaddw2.2d	v1, v1, v5
	uaddw.2d	v2, v2, v6
	uaddw2.2d	v2, v2, v6
	uaddw.2d	v3, v3, v7
	uaddw2.2d	v3, v3, v7
	subs	x13, x13, #16
	b.ne	LBB0_7
	add.2d	v0, v1, v0
	add.2d	v1, v3, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	fmov	x11, d0
	cmp	x9, x10
	b.eq	LBB0_15
	cbz	x12, LBB0_16
LBB0_10:
	and	x13, x9, #0x7ffffffffffffffe
	add	x12, x0, x13, lsl #2
	movi.2d	v0, #0000000000000000
	mov.d	v0[0], x11
	add	x11, x0, x10, lsl #2
	sub	x10, x10, x13
LBB0_11:
	ldr	d1, [x11], #8
	uaddw.2d	v0, v0, v1
	adds	x10, x10, #2
	b.ne	LBB0_11
	addp.2d	d0, v0
	fmov	x11, d0
	cmp	x9, x13
	b.eq	LBB0_15
LBB0_13:
	add	x8, x0, x8
LBB0_14:
	ldr	w9, [x12], #4
	add	x11, x11, x9
	cmp	x12, x8
	b.ne	LBB0_14
LBB0_15:
	mov	w8, w2
	cmp	x11, x8
	csel	x0, x11, x8, lo
	ret
LBB0_16:
	add	x12, x0, x10, lsl #2
	b	LBB0_13

	.globl	__RNvCsltXhV8SboNZ_15f_where_carried13arm_lanes_sat
	.p2align	2
__RNvCsltXhV8SboNZ_15f_where_carried13arm_lanes_sat:
	stp	d15, d14, [sp, #-80]!
	stp	d13, d12, [sp, #16]
	stp	d11, d10, [sp, #32]
	stp	d9, d8, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	lsr	x9, x1, #2
	cbz	x9, LBB1_3
	add	x8, x1, #3
	lsr	x8, x8, #2
	add	x10, x1, #2
	lsr	x10, x10, #2
	mov	w11, #2
	cmp	x1, #2
	csel	x11, x1, x11, hi
	add	x11, x11, #1
	lsr	x11, x11, #2
	cmp	x9, x8
	csel	x12, x9, x8, lo
	cmp	x12, x10
	csel	x12, x12, x10, lo
	cmp	x12, x11
	csel	x12, x12, x11, lo
	sub	x13, x9, #1
	cmp	x12, x13
	csel	x12, x12, x13, lo
	cmp	x12, #16
	b.hs	LBB1_4
	mov	x13, #0
	mov	x14, #0
	mov	x15, #0
	mov	x16, #0
	mov	x12, #0
	b	LBB1_7
LBB1_3:
	mov	x10, #0
	and	x8, x1, #0x1ffffffffffffffc
	cmp	x8, x1
	b.ne	LBB1_14
	b	LBB1_26
LBB1_4:
	movi.2d	v0, #0000000000000000
	add	x12, x12, #1
	movi.2d	v2, #0000000000000000
	ands	x13, x12, #0xf
	movi.2d	v7, #0000000000000000
	mov	w14, #16
	movi.2d	v17, #0000000000000000
	csel	x13, x14, x13, eq
	movi.2d	v3, #0000000000000000
	sub	x12, x12, x13
	movi.2d	v18, #0000000000000000
	add	x13, x0, #128
	movi.2d	v22, #0000000000000000
	mov	x14, x12
	movi.2d	v23, #0000000000000000
	movi.2d	v16, #0000000000000000
	movi.2d	v19, #0000000000000000
	movi.2d	v20, #0000000000000000
	movi.2d	v21, #0000000000000000
	movi.2d	v1, #0000000000000000
	movi.2d	v4, #0000000000000000
	movi.2d	v5, #0000000000000000
	movi.2d	v6, #0000000000000000
LBB1_5:
	sub	x15, x13, #128
	sub	x16, x13, #64
	ld4.4s	{ v24, v25, v26, v27 }, [x15]
	ld4.4s	{ v28, v29, v30, v31 }, [x16]
	mov	x15, x13
	ld4.4s	{ v8, v9, v10, v11 }, [x15], #64
	ld4.4s	{ v12, v13, v14, v15 }, [x15]
	uaddw.2d	v1, v1, v24
	uaddw2.2d	v1, v1, v24
	uaddw.2d	v4, v4, v28
	uaddw2.2d	v4, v4, v28
	uaddw.2d	v5, v5, v8
	uaddw2.2d	v5, v5, v8
	uaddw.2d	v6, v6, v12
	uaddw2.2d	v6, v6, v12
	uaddw.2d	v16, v16, v25
	uaddw2.2d	v16, v16, v25
	uaddw.2d	v19, v19, v29
	uaddw2.2d	v19, v19, v29
	uaddw.2d	v20, v20, v9
	uaddw2.2d	v20, v20, v9
	uaddw.2d	v21, v21, v13
	uaddw2.2d	v21, v21, v13
	uaddw.2d	v3, v3, v26
	uaddw2.2d	v3, v3, v26
	uaddw.2d	v18, v18, v30
	uaddw2.2d	v18, v18, v30
	uaddw.2d	v22, v22, v10
	uaddw2.2d	v22, v22, v10
	uaddw.2d	v23, v23, v14
	uaddw2.2d	v23, v23, v14
	uaddw.2d	v0, v0, v27
	uaddw2.2d	v0, v0, v27
	uaddw.2d	v2, v2, v31
	uaddw2.2d	v2, v2, v31
	uaddw.2d	v7, v7, v11
	uaddw2.2d	v7, v7, v11
	uaddw.2d	v17, v17, v15
	uaddw2.2d	v17, v17, v15
	add	x13, x13, #256
	subs	x14, x14, #16
	b.ne	LBB1_5
	add.2d	v0, v2, v0
	add.2d	v0, v7, v0
	add.2d	v0, v17, v0
	addp.2d	d0, v0
	fmov	x13, d0
	add.2d	v0, v18, v3
	add.2d	v0, v22, v0
	add.2d	v0, v23, v0
	addp.2d	d0, v0
	fmov	x14, d0
	add.2d	v0, v19, v16
	add.2d	v0, v20, v0
	add.2d	v0, v21, v0
	addp.2d	d0, v0
	fmov	x15, d0
	add.2d	v0, v4, v1
	add.2d	v0, v5, v0
	add.2d	v0, v6, v0
	addp.2d	d0, v0
	fmov	x16, d0
LBB1_7:
	sub	x17, x9, x12
	sub	x11, x11, x12
	sub	x10, x10, x12
	sub	x3, x8, x12
	lsl	x8, x12, #2
	add	x12, x0, x12, lsl #4
	add	x12, x12, #8
LBB1_8:
	cbz	x3, LBB1_30
	cbz	x10, LBB1_31
	cbz	x11, LBB1_28
	cbz	x17, LBB1_29
	ldp	w4, w5, [x12, #-8]
	add	x16, x16, x4
	add	x15, x15, x5
	ldp	w4, w5, [x12], #16
	add	x14, x14, x4
	add	x13, x13, x5
	sub	x11, x11, #1
	sub	x10, x10, #1
	sub	x3, x3, #1
	add	x8, x8, #4
	sub	x17, x17, #1
	cbnz	x17, LBB1_8
	add	x8, x15, x16
	add	x10, x14, x13
	add	x10, x8, x10
	and	x8, x1, #0x1ffffffffffffffc
	cmp	x8, x1
	b.eq	LBB1_26
LBB1_14:
	orr	x11, x8, #0x1
	cmp	x1, x11
	csinc	x11, x1, x8, hi
	sub	x12, x11, x8
	cmp	x12, #2
	b.hs	LBB1_16
	and	x12, x1, #0x1ffffffffffffffc
	b	LBB1_25
LBB1_16:
	cmp	x12, #16
	b.hs	LBB1_18
	mov	x13, #0
	b	LBB1_22
LBB1_18:
	and	x14, x12, #0xe
	and	x13, x12, #0xfffffffffffffff0
	movi.2d	v0, #0000000000000000
	mov.d	v0[0], x10
	movi.2d	v1, #0000000000000000
	add	x10, x0, x9, lsl #4
	add	x10, x10, #32
	and	x15, x12, #0xfffffffffffffff0
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB1_19:
	ldp	q4, q5, [x10, #-32]
	ldp	q6, q7, [x10], #64
	uaddw.2d	v0, v0, v4
	uaddw2.2d	v0, v0, v4
	uaddw.2d	v1, v1, v5
	uaddw2.2d	v1, v1, v5
	uaddw.2d	v2, v2, v6
	uaddw2.2d	v2, v2, v6
	uaddw.2d	v3, v3, v7
	uaddw2.2d	v3, v3, v7
	subs	x15, x15, #16
	b.ne	LBB1_19
	add.2d	v0, v1, v0
	add.2d	v1, v3, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	fmov	x10, d0
	cmp	x12, x13
	b.eq	LBB1_26
	cbz	x14, LBB1_27
LBB1_22:
	and	x14, x11, #0x1
	sub	x12, x12, x14
	add	x12, x8, x12
	movi.2d	v0, #0000000000000000
	mov.d	v0[0], x10
	lsl	x9, x9, #4
	add	x9, x9, x13, lsl #2
	add	x9, x0, x9
	sub	x10, x11, x13
	sub	x10, x10, x14
	sub	x8, x10, x8
LBB1_23:
	ldr	d1, [x9], #8
	uaddw.2d	v0, v0, v1
	subs	x8, x8, #2
	b.ne	LBB1_23
	addp.2d	d0, v0
	fmov	x10, d0
	cbz	x14, LBB1_26
LBB1_25:
	ldr	w8, [x0, x12, lsl #2]
	add	x10, x10, x8
	add	x12, x12, #1
	cmp	x12, x1
	b.lo	LBB1_25
LBB1_26:
	mov	w8, w2
	cmp	x10, x8
	csel	x0, x10, x8, lo
	ldp	x29, x30, [sp, #64]
	ldp	d9, d8, [sp, #48]
	ldp	d11, d10, [sp, #32]
	ldp	d13, d12, [sp, #16]
	ldp	d15, d14, [sp], #80
	ret
LBB1_27:
	add	x12, x8, x13
	b	LBB1_25
LBB1_28:
Lloh0:
	adrp	x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.3@PAGE
Lloh1:
	add	x2, x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.3@PAGEOFF
	add	x0, x8, #2
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB1_29:
Lloh2:
	adrp	x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.4@PAGE
Lloh3:
	add	x2, x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.4@PAGEOFF
	add	x0, x8, #3
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB1_30:
Lloh4:
	adrp	x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.1@PAGE
Lloh5:
	add	x2, x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.1@PAGEOFF
	mov	x0, x8
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB1_31:
Lloh6:
	adrp	x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.2@PAGE
Lloh7:
	add	x2, x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.2@PAGEOFF
	add	x0, x8, #1
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh0, Lloh1
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh6, Lloh7

	.globl	__RNvCsltXhV8SboNZ_15f_where_carried13arm_tight_sat
	.p2align	2
__RNvCsltXhV8SboNZ_15f_where_carried13arm_tight_sat:
	.cfi_startproc
	mov	w8, #0
	cbz	x1, LBB2_3
	lsl	x9, x1, #2
LBB2_2:
	ldr	w10, [x0], #4
	adds	w8, w8, w10
	csinv	w8, w8, wzr, lo
	cmp	w2, w8
	csel	w8, w2, w8, lo
	subs	x9, x9, #4
	b.ne	LBB2_2
LBB2_3:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	__RNvCsltXhV8SboNZ_15f_where_carried13arm_wide_wrap
	.p2align	2
__RNvCsltXhV8SboNZ_15f_where_carried13arm_wide_wrap:
	cbz	x1, LBB3_3
	lsl	x8, x1, #2
	sub	x10, x8, #4
	cmp	x10, #12
	b.hs	LBB3_4
	mov	w12, #0
	mov	x11, x0
	b	LBB3_13
LBB3_3:
	mov	w0, #0
	ret
LBB3_4:
	lsr	x9, x10, #2
	add	x9, x9, #1
	cmp	x10, #60
	b.hs	LBB3_6
	mov	x10, #0
	mov	w12, #0
	b	LBB3_10
LBB3_6:
	and	x11, x9, #0xc
	and	x10, x9, #0x7ffffffffffffff0
	add	x12, x0, #32
	movi.2d	v0, #0000000000000000
	and	x13, x9, #0x7ffffffffffffff0
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB3_7:
	ldp	q4, q5, [x12, #-32]
	ldp	q6, q7, [x12], #64
	add.4s	v0, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	subs	x13, x13, #16
	b.ne	LBB3_7
	add.4s	v0, v1, v0
	add.4s	v0, v2, v0
	add.4s	v0, v3, v0
	addv.4s	s0, v0
	fmov	w12, s0
	cmp	x9, x10
	b.eq	LBB3_15
	cbz	x11, LBB3_16
LBB3_10:
	and	x13, x9, #0x7ffffffffffffffc
	add	x11, x0, x13, lsl #2
	movi.2d	v0, #0000000000000000
	mov.s	v0[0], w12
	add	x12, x0, x10, lsl #2
	sub	x10, x10, x13
LBB3_11:
	ldr	q1, [x12], #16
	add.4s	v0, v1, v0
	adds	x10, x10, #4
	b.ne	LBB3_11
	addv.4s	s0, v0
	fmov	w12, s0
	cmp	x9, x13
	b.eq	LBB3_15
LBB3_13:
	add	x8, x0, x8
LBB3_14:
	ldr	w9, [x11], #4
	add	w12, w9, w12
	cmp	x11, x8
	b.ne	LBB3_14
LBB3_15:
	and	w0, w12, w2
	ret
LBB3_16:
	add	x11, x0, x10, lsl #2
	b	LBB3_13

	.globl	__RNvCsltXhV8SboNZ_15f_where_carried14arm_lanes_wrap
	.p2align	2
__RNvCsltXhV8SboNZ_15f_where_carried14arm_lanes_wrap:
	stp	d15, d14, [sp, #-80]!
	stp	d13, d12, [sp, #16]
	stp	d11, d10, [sp, #32]
	stp	d9, d8, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	lsr	x9, x1, #2
	cbz	x9, LBB4_3
	add	x8, x1, #3
	lsr	x8, x8, #2
	add	x10, x1, #2
	lsr	x10, x10, #2
	mov	w11, #2
	cmp	x1, #2
	csel	x11, x1, x11, hi
	add	x11, x11, #1
	lsr	x11, x11, #2
	cmp	x9, x8
	csel	x12, x9, x8, lo
	cmp	x12, x10
	csel	x12, x12, x10, lo
	cmp	x12, x11
	csel	x12, x12, x11, lo
	sub	x13, x9, #1
	cmp	x12, x13
	csel	x12, x12, x13, lo
	cmp	x12, #16
	b.hs	LBB4_4
	mov	w13, #0
	mov	w14, #0
	mov	w15, #0
	mov	w16, #0
	mov	x12, #0
	b	LBB4_7
LBB4_3:
	mov	w10, #0
	and	x8, x1, #0x1ffffffffffffffc
	cmp	x8, x1
	b.ne	LBB4_14
	b	LBB4_26
LBB4_4:
	movi.2d	v0, #0000000000000000
	add	x12, x12, #1
	movi.2d	v2, #0000000000000000
	ands	x13, x12, #0xf
	movi.2d	v7, #0000000000000000
	mov	w14, #16
	movi.2d	v17, #0000000000000000
	csel	x13, x14, x13, eq
	movi.2d	v3, #0000000000000000
	sub	x12, x12, x13
	movi.2d	v18, #0000000000000000
	add	x13, x0, #128
	movi.2d	v22, #0000000000000000
	mov	x14, x12
	movi.2d	v23, #0000000000000000
	movi.2d	v16, #0000000000000000
	movi.2d	v19, #0000000000000000
	movi.2d	v20, #0000000000000000
	movi.2d	v21, #0000000000000000
	movi.2d	v1, #0000000000000000
	movi.2d	v4, #0000000000000000
	movi.2d	v5, #0000000000000000
	movi.2d	v6, #0000000000000000
LBB4_5:
	sub	x15, x13, #128
	sub	x16, x13, #64
	ld4.4s	{ v24, v25, v26, v27 }, [x15]
	ld4.4s	{ v28, v29, v30, v31 }, [x16]
	mov	x15, x13
	ld4.4s	{ v8, v9, v10, v11 }, [x15], #64
	ld4.4s	{ v12, v13, v14, v15 }, [x15]
	add.4s	v1, v24, v1
	add.4s	v4, v28, v4
	add.4s	v5, v8, v5
	add.4s	v6, v12, v6
	add.4s	v16, v25, v16
	add.4s	v19, v29, v19
	add.4s	v20, v9, v20
	add.4s	v21, v13, v21
	add.4s	v3, v26, v3
	add.4s	v18, v30, v18
	add.4s	v22, v10, v22
	add.4s	v23, v14, v23
	add.4s	v0, v27, v0
	add.4s	v2, v31, v2
	add.4s	v7, v11, v7
	add.4s	v17, v15, v17
	add	x13, x13, #256
	subs	x14, x14, #16
	b.ne	LBB4_5
	add.4s	v0, v2, v0
	add.4s	v0, v7, v0
	add.4s	v0, v17, v0
	addv.4s	s0, v0
	fmov	w13, s0
	add.4s	v0, v18, v3
	add.4s	v0, v22, v0
	add.4s	v0, v23, v0
	addv.4s	s0, v0
	fmov	w14, s0
	add.4s	v0, v19, v16
	add.4s	v0, v20, v0
	add.4s	v0, v21, v0
	addv.4s	s0, v0
	fmov	w15, s0
	add.4s	v0, v4, v1
	add.4s	v0, v5, v0
	add.4s	v0, v6, v0
	addv.4s	s0, v0
	fmov	w16, s0
LBB4_7:
	sub	x17, x9, x12
	sub	x11, x11, x12
	sub	x10, x10, x12
	sub	x3, x8, x12
	lsl	x8, x12, #2
	add	x12, x0, x12, lsl #4
	add	x12, x12, #8
LBB4_8:
	cbz	x3, LBB4_30
	cbz	x10, LBB4_31
	cbz	x11, LBB4_28
	cbz	x17, LBB4_29
	ldp	w4, w5, [x12, #-8]
	add	w16, w4, w16
	add	w15, w5, w15
	ldp	w4, w5, [x12], #16
	add	w14, w4, w14
	add	w13, w5, w13
	sub	x11, x11, #1
	sub	x10, x10, #1
	sub	x3, x3, #1
	add	x8, x8, #4
	sub	x17, x17, #1
	cbnz	x17, LBB4_8
	add	w8, w15, w16
	add	w10, w14, w13
	add	w10, w8, w10
	and	x8, x1, #0x1ffffffffffffffc
	cmp	x8, x1
	b.eq	LBB4_26
LBB4_14:
	orr	x11, x8, #0x1
	cmp	x1, x11
	csinc	x11, x1, x8, hi
	sub	x12, x11, x8
	cmp	x12, #4
	b.hs	LBB4_16
	and	x9, x1, #0x1ffffffffffffffc
	b	LBB4_25
LBB4_16:
	cmp	x12, #16
	b.hs	LBB4_18
	mov	x13, #0
	b	LBB4_22
LBB4_18:
	and	x14, x12, #0xc
	and	x13, x12, #0xfffffffffffffff0
	movi.2d	v0, #0000000000000000
	mov.s	v0[0], w10
	movi.2d	v1, #0000000000000000
	add	x9, x0, x9, lsl #4
	add	x9, x9, #32
	and	x10, x12, #0xfffffffffffffff0
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB4_19:
	ldp	q4, q5, [x9, #-32]
	ldp	q6, q7, [x9], #64
	add.4s	v0, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	subs	x10, x10, #16
	b.ne	LBB4_19
	add.4s	v0, v1, v0
	add.4s	v0, v2, v0
	add.4s	v0, v3, v0
	addv.4s	s0, v0
	fmov	w10, s0
	cmp	x12, x13
	b.eq	LBB4_26
	cbz	x14, LBB4_27
LBB4_22:
	and	x14, x11, #0x3
	sub	x9, x12, x14
	add	x9, x8, x9
	movi.2d	v0, #0000000000000000
	mov.s	v0[0], w10
	lsl	x10, x1, #2
	and	x10, x10, #0xfffffffffffffff0
	add	x10, x10, x13, lsl #2
	add	x10, x0, x10
	sub	x11, x11, x13
	sub	x11, x11, x14
	sub	x8, x11, x8
LBB4_23:
	ldr	q1, [x10], #16
	add.4s	v0, v1, v0
	subs	x8, x8, #4
	b.ne	LBB4_23
	addv.4s	s0, v0
	fmov	w10, s0
	cbz	x14, LBB4_26
LBB4_25:
	ldr	w8, [x0, x9, lsl #2]
	add	w10, w8, w10
	add	x9, x9, #1
	cmp	x9, x1
	b.lo	LBB4_25
LBB4_26:
	and	w0, w10, w2
	ldp	x29, x30, [sp, #64]
	ldp	d9, d8, [sp, #48]
	ldp	d11, d10, [sp, #32]
	ldp	d13, d12, [sp, #16]
	ldp	d15, d14, [sp], #80
	ret
LBB4_27:
	add	x9, x8, x13
	b	LBB4_25
LBB4_28:
Lloh8:
	adrp	x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.7@PAGE
Lloh9:
	add	x2, x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.7@PAGEOFF
	add	x0, x8, #2
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB4_29:
Lloh10:
	adrp	x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.8@PAGE
Lloh11:
	add	x2, x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.8@PAGEOFF
	add	x0, x8, #3
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB4_30:
Lloh12:
	adrp	x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.5@PAGE
Lloh13:
	add	x2, x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.5@PAGEOFF
	mov	x0, x8
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB4_31:
Lloh14:
	adrp	x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.6@PAGE
Lloh15:
	add	x2, x2, l_anon.0c7f7db17baa0d39f8fc92e12be9d613.6@PAGEOFF
	add	x0, x8, #1
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh14, Lloh15

	.globl	__RNvCsltXhV8SboNZ_15f_where_carried14arm_tight_wrap
	.p2align	2
__RNvCsltXhV8SboNZ_15f_where_carried14arm_tight_wrap:
	mov	w8, #0
	cbz	x1, LBB5_3
	lsl	x9, x1, #2
LBB5_2:
	ldr	w10, [x0], #4
	add	w8, w10, w8
	and	w8, w8, w2
	subs	x9, x9, #4
	b.ne	LBB5_2
LBB5_3:
	mov	x0, x8
	ret

	.globl	_site_faithful
	.p2align	2
_site_faithful:
	mov	w2, #8191
	b	__RNvCsltXhV8SboNZ_15f_where_carried12arm_wide_sat

	.globl	_site_fast
	.p2align	2
_site_fast:
	mov	w2, #8191
	b	__RNvCsltXhV8SboNZ_15f_where_carried13arm_lanes_sat

	.globl	_site_fast_wrapping
	.p2align	2
_site_fast_wrapping:
	mov	w2, #8191
	b	__RNvCsltXhV8SboNZ_15f_where_carried14arm_lanes_wrap

	.globl	_site_tight
	.p2align	2
_site_tight:
	mov	w2, #8191
	b	__RNvCsltXhV8SboNZ_15f_where_carried13arm_tight_sat

	.section	__TEXT,__cstring,cstring_literals
l_anon.0c7f7db17baa0d39f8fc92e12be9d613.0:
	.asciz	"f_where_carried.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.0c7f7db17baa0d39f8fc92e12be9d613.1:
	.quad	l_anon.0c7f7db17baa0d39f8fc92e12be9d613.0
	.asciz	"\022\000\000\000\000\000\000\000\211\000\000\000\021\000\000"

	.p2align	3, 0x0
l_anon.0c7f7db17baa0d39f8fc92e12be9d613.2:
	.quad	l_anon.0c7f7db17baa0d39f8fc92e12be9d613.0
	.asciz	"\022\000\000\000\000\000\000\000\212\000\000\000\021\000\000"

	.p2align	3, 0x0
l_anon.0c7f7db17baa0d39f8fc92e12be9d613.3:
	.quad	l_anon.0c7f7db17baa0d39f8fc92e12be9d613.0
	.asciz	"\022\000\000\000\000\000\000\000\213\000\000\000\021\000\000"

	.p2align	3, 0x0
l_anon.0c7f7db17baa0d39f8fc92e12be9d613.4:
	.quad	l_anon.0c7f7db17baa0d39f8fc92e12be9d613.0
	.asciz	"\022\000\000\000\000\000\000\000\214\000\000\000\021\000\000"

	.p2align	3, 0x0
l_anon.0c7f7db17baa0d39f8fc92e12be9d613.5:
	.quad	l_anon.0c7f7db17baa0d39f8fc92e12be9d613.0
	.asciz	"\022\000\000\000\000\000\000\000\233\000\000\000\021\000\000"

	.p2align	3, 0x0
l_anon.0c7f7db17baa0d39f8fc92e12be9d613.6:
	.quad	l_anon.0c7f7db17baa0d39f8fc92e12be9d613.0
	.asciz	"\022\000\000\000\000\000\000\000\234\000\000\000\021\000\000"

	.p2align	3, 0x0
l_anon.0c7f7db17baa0d39f8fc92e12be9d613.7:
	.quad	l_anon.0c7f7db17baa0d39f8fc92e12be9d613.0
	.asciz	"\022\000\000\000\000\000\000\000\235\000\000\000\021\000\000"

	.p2align	3, 0x0
l_anon.0c7f7db17baa0d39f8fc92e12be9d613.8:
	.quad	l_anon.0c7f7db17baa0d39f8fc92e12be9d613.0
	.asciz	"\022\000\000\000\000\000\000\000\236\000\000\000\021\000\000"

.subsections_via_symbols
