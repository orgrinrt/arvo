	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCs2HiZyOMri21_17a_choice_function10arm_accfit
	.p2align	2
__RNvCs2HiZyOMri21_17a_choice_function10arm_accfit:
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

	.globl	__RNvCs2HiZyOMri21_17a_choice_function11arm_minimum
	.p2align	2
__RNvCs2HiZyOMri21_17a_choice_function11arm_minimum:
	mov	w8, #0
	cbz	x1, LBB1_3
	lsl	x9, x1, #2
LBB1_2:
	ldr	w10, [x0], #4
	adds	w11, w10, w8
	cmp	w11, w2
	csel	w11, w11, w2, lo
	cmn	w10, w8
	csel	w8, w2, w11, hs
	subs	x9, x9, #4
	b.ne	LBB1_2
LBB1_3:
	mov	x0, x8
	ret

	.globl	__RNvCs2HiZyOMri21_17a_choice_function16arm_accfit_lanes
	.p2align	2
__RNvCs2HiZyOMri21_17a_choice_function16arm_accfit_lanes:
	stp	d15, d14, [sp, #-80]!
	stp	d13, d12, [sp, #16]
	stp	d11, d10, [sp, #32]
	stp	d9, d8, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	lsr	x9, x1, #2
	cbz	x9, LBB2_3
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
	b.hs	LBB2_4
	mov	x13, #0
	mov	x14, #0
	mov	x15, #0
	mov	x16, #0
	mov	x12, #0
	b	LBB2_7
LBB2_3:
	mov	x10, #0
	and	x8, x1, #0x1ffffffffffffffc
	cmp	x8, x1
	b.ne	LBB2_14
	b	LBB2_26
LBB2_4:
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
LBB2_5:
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
	b.ne	LBB2_5
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
LBB2_7:
	sub	x17, x9, x12
	sub	x11, x11, x12
	sub	x10, x10, x12
	sub	x3, x8, x12
	lsl	x8, x12, #2
	add	x12, x0, x12, lsl #4
	add	x12, x12, #8
LBB2_8:
	cbz	x3, LBB2_30
	cbz	x10, LBB2_31
	cbz	x11, LBB2_28
	cbz	x17, LBB2_29
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
	cbnz	x17, LBB2_8
	add	x8, x15, x16
	add	x10, x14, x13
	add	x10, x8, x10
	and	x8, x1, #0x1ffffffffffffffc
	cmp	x8, x1
	b.eq	LBB2_26
LBB2_14:
	orr	x11, x8, #0x1
	cmp	x1, x11
	csinc	x11, x1, x8, hi
	sub	x12, x11, x8
	cmp	x12, #2
	b.hs	LBB2_16
	and	x12, x1, #0x1ffffffffffffffc
	b	LBB2_25
LBB2_16:
	cmp	x12, #16
	b.hs	LBB2_18
	mov	x13, #0
	b	LBB2_22
LBB2_18:
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
LBB2_19:
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
	b.ne	LBB2_19
	add.2d	v0, v1, v0
	add.2d	v1, v3, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	fmov	x10, d0
	cmp	x12, x13
	b.eq	LBB2_26
	cbz	x14, LBB2_27
LBB2_22:
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
LBB2_23:
	ldr	d1, [x9], #8
	uaddw.2d	v0, v0, v1
	subs	x8, x8, #2
	b.ne	LBB2_23
	addp.2d	d0, v0
	fmov	x10, d0
	cbz	x14, LBB2_26
LBB2_25:
	ldr	w8, [x0, x12, lsl #2]
	add	x10, x10, x8
	add	x12, x12, #1
	cmp	x12, x1
	b.lo	LBB2_25
LBB2_26:
	mov	w8, w2
	cmp	x10, x8
	csel	x0, x10, x8, lo
	ldp	x29, x30, [sp, #64]
	ldp	d9, d8, [sp, #48]
	ldp	d11, d10, [sp, #32]
	ldp	d13, d12, [sp, #16]
	ldp	d15, d14, [sp], #80
	ret
LBB2_27:
	add	x12, x8, x13
	b	LBB2_25
LBB2_28:
Lloh0:
	adrp	x2, l_anon.2ec99d86c92543e89b6ead53473604f1.3@PAGE
Lloh1:
	add	x2, x2, l_anon.2ec99d86c92543e89b6ead53473604f1.3@PAGEOFF
	add	x0, x8, #2
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB2_29:
Lloh2:
	adrp	x2, l_anon.2ec99d86c92543e89b6ead53473604f1.4@PAGE
Lloh3:
	add	x2, x2, l_anon.2ec99d86c92543e89b6ead53473604f1.4@PAGEOFF
	add	x0, x8, #3
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB2_30:
Lloh4:
	adrp	x2, l_anon.2ec99d86c92543e89b6ead53473604f1.1@PAGE
Lloh5:
	add	x2, x2, l_anon.2ec99d86c92543e89b6ead53473604f1.1@PAGEOFF
	mov	x0, x8
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB2_31:
Lloh6:
	adrp	x2, l_anon.2ec99d86c92543e89b6ead53473604f1.2@PAGE
Lloh7:
	add	x2, x2, l_anon.2ec99d86c92543e89b6ead53473604f1.2@PAGEOFF
	add	x0, x8, #1
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh0, Lloh1
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh6, Lloh7

	.globl	_entry_exact
	.p2align	2
_entry_exact:
	b	__RNvCs2HiZyOMri21_17a_choice_function10arm_accfit

	.globl	_entry_footprint
	.p2align	2
_entry_footprint:
	b	__RNvCs2HiZyOMri21_17a_choice_function11arm_minimum

	.globl	_entry_speed
	.p2align	2
_entry_speed:
	b	__RNvCs2HiZyOMri21_17a_choice_function16arm_accfit_lanes

	.section	__TEXT,__cstring,cstring_literals
l_anon.2ec99d86c92543e89b6ead53473604f1.0:
	.asciz	"a_choice_function.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.2ec99d86c92543e89b6ead53473604f1.1:
	.quad	l_anon.2ec99d86c92543e89b6ead53473604f1.0
	.asciz	"\024\000\000\000\000\000\000\000:\000\000\000\021\000\000"

	.p2align	3, 0x0
l_anon.2ec99d86c92543e89b6ead53473604f1.2:
	.quad	l_anon.2ec99d86c92543e89b6ead53473604f1.0
	.asciz	"\024\000\000\000\000\000\000\000;\000\000\000\021\000\000"

	.p2align	3, 0x0
l_anon.2ec99d86c92543e89b6ead53473604f1.3:
	.quad	l_anon.2ec99d86c92543e89b6ead53473604f1.0
	.asciz	"\024\000\000\000\000\000\000\000<\000\000\000\021\000\000"

	.p2align	3, 0x0
l_anon.2ec99d86c92543e89b6ead53473604f1.4:
	.quad	l_anon.2ec99d86c92543e89b6ead53473604f1.0
	.asciz	"\024\000\000\000\000\000\000\000=\000\000\000\021\000\000"

	.globl	_entry_speed_short
_entry_speed_short = _entry_exact
.subsections_via_symbols
