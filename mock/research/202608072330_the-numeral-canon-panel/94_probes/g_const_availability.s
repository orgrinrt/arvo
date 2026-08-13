	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCsbh3xbCK06wH_20g_const_availability7arm_seq
	.p2align	2
__RNvCsbh3xbCK06wH_20g_const_availability7arm_seq:
	cbz	x1, LBB0_3
	lsl	x9, x1, #2
	sub	x8, x9, #4
	cmp	x8, #12
	b.hs	LBB0_4
	mov	w8, #0
	mov	x12, x0
	b	LBB0_13
LBB0_3:
	mov	w8, #0
	mov	x0, x8
	ret
LBB0_4:
	lsr	x10, x8, #2
	add	x10, x10, #1
	cmp	x8, #60
	b.hs	LBB0_6
	mov	x11, #0
	mov	w8, #0
	b	LBB0_10
LBB0_6:
	and	x12, x10, #0xc
	and	x11, x10, #0x7ffffffffffffff0
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	and	x13, x10, #0x7ffffffffffffff0
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB0_7:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	add.4s	v0, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	subs	x13, x13, #16
	b.ne	LBB0_7
	add.4s	v0, v1, v0
	add.4s	v0, v2, v0
	add.4s	v0, v3, v0
	addv.4s	s0, v0
	fmov	w8, s0
	cmp	x10, x11
	b.eq	LBB0_15
	cbz	x12, LBB0_16
LBB0_10:
	and	x13, x10, #0x7ffffffffffffffc
	add	x12, x0, x13, lsl #2
	movi.2d	v0, #0000000000000000
	mov.s	v0[0], w8
	add	x8, x0, x11, lsl #2
	sub	x11, x11, x13
LBB0_11:
	ldr	q1, [x8], #16
	add.4s	v0, v1, v0
	adds	x11, x11, #4
	b.ne	LBB0_11
	addv.4s	s0, v0
	fmov	w8, s0
	cmp	x10, x13
	b.eq	LBB0_15
LBB0_13:
	add	x9, x0, x9
LBB0_14:
	ldr	w10, [x12], #4
	add	w8, w10, w8
	cmp	x12, x9
	b.ne	LBB0_14
LBB0_15:
	mov	x0, x8
	ret
LBB0_16:
	add	x12, x0, x11, lsl #2
	b	LBB0_13

	.globl	__RNvCsbh3xbCK06wH_20g_const_availability9arm_lanes
	.p2align	2
__RNvCsbh3xbCK06wH_20g_const_availability9arm_lanes:
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
	mov	w13, #0
	mov	w14, #0
	mov	w15, #0
	mov	w16, #0
	mov	x12, #0
	b	LBB1_7
LBB1_3:
	mov	w8, #0
	and	x10, x1, #0x1ffffffffffffffc
	cmp	x10, x1
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
	b.ne	LBB1_5
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
LBB1_7:
	sub	x17, x9, x12
	sub	x11, x11, x12
	sub	x10, x10, x12
	sub	x2, x8, x12
	lsl	x8, x12, #2
	add	x12, x0, x12, lsl #4
	add	x12, x12, #8
LBB1_8:
	cbz	x2, LBB1_30
	cbz	x10, LBB1_31
	cbz	x11, LBB1_28
	cbz	x17, LBB1_29
	ldp	w3, w4, [x12, #-8]
	add	w16, w3, w16
	add	w15, w4, w15
	ldp	w3, w4, [x12], #16
	add	w14, w3, w14
	add	w13, w4, w13
	sub	x11, x11, #1
	sub	x10, x10, #1
	sub	x2, x2, #1
	add	x8, x8, #4
	sub	x17, x17, #1
	cbnz	x17, LBB1_8
	add	w8, w15, w16
	add	w10, w14, w13
	add	w8, w8, w10
	and	x10, x1, #0x1ffffffffffffffc
	cmp	x10, x1
	b.eq	LBB1_26
LBB1_14:
	orr	x11, x10, #0x1
	cmp	x1, x11
	csinc	x11, x1, x10, hi
	sub	x12, x11, x10
	cmp	x12, #4
	b.hs	LBB1_16
	and	x9, x1, #0x1ffffffffffffffc
	b	LBB1_25
LBB1_16:
	cmp	x12, #16
	b.hs	LBB1_18
	mov	x13, #0
	b	LBB1_22
LBB1_18:
	and	x14, x12, #0xc
	and	x13, x12, #0xfffffffffffffff0
	movi.2d	v0, #0000000000000000
	mov.s	v0[0], w8
	movi.2d	v1, #0000000000000000
	add	x8, x0, x9, lsl #4
	add	x8, x8, #32
	and	x9, x12, #0xfffffffffffffff0
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB1_19:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	add.4s	v0, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	subs	x9, x9, #16
	b.ne	LBB1_19
	add.4s	v0, v1, v0
	add.4s	v0, v2, v0
	add.4s	v0, v3, v0
	addv.4s	s0, v0
	fmov	w8, s0
	cmp	x12, x13
	b.eq	LBB1_26
	cbz	x14, LBB1_27
LBB1_22:
	and	x14, x11, #0x3
	sub	x9, x12, x14
	add	x9, x10, x9
	movi.2d	v0, #0000000000000000
	mov.s	v0[0], w8
	lsl	x8, x1, #2
	and	x8, x8, #0xfffffffffffffff0
	add	x8, x8, x13, lsl #2
	add	x8, x0, x8
	sub	x11, x11, x13
	sub	x11, x11, x14
	sub	x10, x11, x10
LBB1_23:
	ldr	q1, [x8], #16
	add.4s	v0, v1, v0
	subs	x10, x10, #4
	b.ne	LBB1_23
	addv.4s	s0, v0
	fmov	w8, s0
	cbz	x14, LBB1_26
LBB1_25:
	ldr	w10, [x0, x9, lsl #2]
	add	w8, w10, w8
	add	x9, x9, #1
	cmp	x9, x1
	b.lo	LBB1_25
LBB1_26:
	mov	x0, x8
	ldp	x29, x30, [sp, #64]
	ldp	d9, d8, [sp, #48]
	ldp	d11, d10, [sp, #32]
	ldp	d13, d12, [sp, #16]
	ldp	d15, d14, [sp], #80
	ret
LBB1_27:
	add	x9, x10, x13
	b	LBB1_25
LBB1_28:
Lloh0:
	adrp	x2, l_anon.5876d41bca3d2fe8e2f798f523efd5a7.3@PAGE
Lloh1:
	add	x2, x2, l_anon.5876d41bca3d2fe8e2f798f523efd5a7.3@PAGEOFF
	add	x0, x8, #2
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB1_29:
Lloh2:
	adrp	x2, l_anon.5876d41bca3d2fe8e2f798f523efd5a7.4@PAGE
Lloh3:
	add	x2, x2, l_anon.5876d41bca3d2fe8e2f798f523efd5a7.4@PAGEOFF
	add	x0, x8, #3
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB1_30:
Lloh4:
	adrp	x2, l_anon.5876d41bca3d2fe8e2f798f523efd5a7.1@PAGE
Lloh5:
	add	x2, x2, l_anon.5876d41bca3d2fe8e2f798f523efd5a7.1@PAGEOFF
	mov	x0, x8
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB1_31:
Lloh6:
	adrp	x2, l_anon.5876d41bca3d2fe8e2f798f523efd5a7.2@PAGE
Lloh7:
	add	x2, x2, l_anon.5876d41bca3d2fe8e2f798f523efd5a7.2@PAGEOFF
	add	x0, x8, #1
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh0, Lloh1
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh6, Lloh7

	.globl	_site_const_long
	.p2align	2
_site_const_long:
	mov	w1, #64
	b	__RNvCsbh3xbCK06wH_20g_const_availability9arm_lanes

	.globl	_site_const_short
	.p2align	2
_site_const_short:
	mov	w1, #4
	b	__RNvCsbh3xbCK06wH_20g_const_availability7arm_seq

	.globl	_site_runtime
	.p2align	2
_site_runtime:
	cmp	x1, #15
	b.ls	LBB4_2
	b	__RNvCsbh3xbCK06wH_20g_const_availability9arm_lanes
LBB4_2:
	b	__RNvCsbh3xbCK06wH_20g_const_availability7arm_seq

	.section	__TEXT,__cstring,cstring_literals
l_anon.5876d41bca3d2fe8e2f798f523efd5a7.0:
	.asciz	"g_const_availability.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.5876d41bca3d2fe8e2f798f523efd5a7.1:
	.quad	l_anon.5876d41bca3d2fe8e2f798f523efd5a7.0
	.asciz	"\027\000\000\000\000\000\000\000\037\000\000\000\"\000\000"

	.p2align	3, 0x0
l_anon.5876d41bca3d2fe8e2f798f523efd5a7.2:
	.quad	l_anon.5876d41bca3d2fe8e2f798f523efd5a7.0
	.asciz	"\027\000\000\000\000\000\000\000 \000\000\000\"\000\000"

	.p2align	3, 0x0
l_anon.5876d41bca3d2fe8e2f798f523efd5a7.3:
	.quad	l_anon.5876d41bca3d2fe8e2f798f523efd5a7.0
	.asciz	"\027\000\000\000\000\000\000\000!\000\000\000\"\000\000"

	.p2align	3, 0x0
l_anon.5876d41bca3d2fe8e2f798f523efd5a7.4:
	.quad	l_anon.5876d41bca3d2fe8e2f798f523efd5a7.0
	.asciz	"\027\000\000\000\000\000\000\000\"\000\000\000\"\000\000"

.subsections_via_symbols
