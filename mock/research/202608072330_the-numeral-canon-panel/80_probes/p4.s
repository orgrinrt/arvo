	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCscY7qz2LUPfe_23p4_what_the_law_unlocks11sat_sum_seq
	.p2align	2
__RNvCscY7qz2LUPfe_23p4_what_the_law_unlocks11sat_sum_seq:
	.cfi_startproc
	mov	w8, #0
	cbz	x1, LBB0_3
	mov	w9, #255
LBB0_2:
	ldrb	w10, [x0], #1
	add	w8, w10, w8, uxtb
	cmp	w8, #255
	csel	w8, w8, w9, lo
	subs	x1, x1, #1
	b.ne	LBB0_2
LBB0_3:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	__RNvCscY7qz2LUPfe_23p4_what_the_law_unlocks12wrap_sum_seq
	.p2align	2
__RNvCscY7qz2LUPfe_23p4_what_the_law_unlocks12wrap_sum_seq:
	.cfi_startproc
	cbz	x1, LBB1_3
	cmp	x1, #8
	b.hs	LBB1_4
	mov	w8, #0
	mov	x10, x0
	b	LBB1_13
LBB1_3:
	mov	w8, #0
	mov	x0, x8
	ret
LBB1_4:
	cmp	x1, #64
	b.hs	LBB1_6
	mov	x9, #0
	mov	w8, #0
	b	LBB1_10
LBB1_6:
	and	x10, x1, #0x38
	and	x9, x1, #0x7fffffffffffffc0
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	and	x11, x1, #0x7fffffffffffffc0
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB1_7:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	add.16b	v0, v4, v0
	add.16b	v1, v5, v1
	add.16b	v2, v6, v2
	add.16b	v3, v7, v3
	subs	x11, x11, #64
	b.ne	LBB1_7
	add.16b	v0, v1, v0
	add.16b	v0, v2, v0
	add.16b	v0, v3, v0
	addv.16b	b0, v0
	fmov	w8, s0
	cmp	x1, x9
	b.eq	LBB1_15
	cbz	x10, LBB1_16
LBB1_10:
	and	x11, x1, #0x7ffffffffffffff8
	add	x10, x0, x11
	movi.2d	v0, #0000000000000000
	mov.b	v0[0], w8
	sub	x8, x9, x11
	add	x9, x0, x9
LBB1_11:
	ldr	d1, [x9], #8
	add.8b	v0, v1, v0
	adds	x8, x8, #8
	b.ne	LBB1_11
	addv.8b	b0, v0
	fmov	w8, s0
	cmp	x1, x11
	b.eq	LBB1_15
LBB1_13:
	add	x9, x0, x1
LBB1_14:
	ldrb	w11, [x10], #1
	add	w8, w11, w8
	cmp	x10, x9
	b.ne	LBB1_14
LBB1_15:
	mov	x0, x8
	ret
LBB1_16:
	add	x10, x0, x9
	b	LBB1_13
	.cfi_endproc

	.globl	__RNvCscY7qz2LUPfe_23p4_what_the_law_unlocks13sat_sum_lanes
	.p2align	2
__RNvCscY7qz2LUPfe_23p4_what_the_law_unlocks13sat_sum_lanes:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	cmp	x1, #4
	b.hs	LBB2_2
	mov	x9, #0
	mov	w8, #0
	subs	x10, x1, x9
	b.hi	LBB2_9
	b	LBB2_11
LBB2_2:
	mov	x8, #0
	mov	w10, #0
	mov	w11, #0
	mov	w12, #0
	mov	w13, #0
	add	x14, x0, #1
	mov	w15, #255
LBB2_3:
	cmp	x8, x1
	b.hs	LBB2_15
	add	x9, x14, x8
	ldurb	w9, [x9, #-1]
	add	w9, w9, w13, uxtb
	cmp	w9, #255
	csel	w13, w9, w15, lo
	add	x9, x8, #1
	cmp	x9, x1
	b.hs	LBB2_14
	ldrb	w9, [x14, x8]
	add	w9, w9, w12, uxtb
	cmp	w9, #255
	csel	w12, w9, w15, lo
	add	x9, x8, #2
	cmp	x9, x1
	b.hs	LBB2_13
	add	x9, x8, #3
	cmp	x9, x1
	b.hs	LBB2_12
	add	x9, x14, x8
	ldrb	w16, [x9, #1]
	add	w11, w16, w11, uxtb
	cmp	w11, #255
	csel	w11, w11, w15, lo
	ldrb	w9, [x9, #2]
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w10, w9, w15, lo
	add	x9, x8, #4
	add	x16, x8, #8
	mov	x8, x9
	cmp	x16, x1
	b.ls	LBB2_3
	and	w8, w13, #0xff
	add	w8, w8, w12, uxtb
	mov	w12, #255
	cmp	w8, #255
	csel	w8, w8, w12, lo
	add	w8, w8, w11, uxtb
	cmp	w8, #255
	csel	w8, w8, w12, lo
	add	w8, w8, w10, uxtb
	cmp	w8, #255
	csel	w8, w8, w12, lo
	subs	x10, x1, x9
	b.ls	LBB2_11
LBB2_9:
	add	x9, x0, x9
	mov	w11, #255
LBB2_10:
	ldrb	w12, [x9], #1
	add	w8, w12, w8, uxtb
	cmp	w8, #255
	csel	w8, w8, w11, lo
	subs	x10, x10, #1
	b.ne	LBB2_10
LBB2_11:
	mov	x0, x8
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB2_12:
	.cfi_restore_state
Lloh0:
	adrp	x2, l_anon.49f33f0a2c3e61d603f533e3d400179b.4@PAGE
Lloh1:
	add	x2, x2, l_anon.49f33f0a2c3e61d603f533e3d400179b.4@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB2_13:
Lloh2:
	adrp	x2, l_anon.49f33f0a2c3e61d603f533e3d400179b.3@PAGE
Lloh3:
	add	x2, x2, l_anon.49f33f0a2c3e61d603f533e3d400179b.3@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB2_14:
Lloh4:
	adrp	x2, l_anon.49f33f0a2c3e61d603f533e3d400179b.2@PAGE
Lloh5:
	add	x2, x2, l_anon.49f33f0a2c3e61d603f533e3d400179b.2@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB2_15:
Lloh6:
	adrp	x2, l_anon.49f33f0a2c3e61d603f533e3d400179b.1@PAGE
Lloh7:
	add	x2, x2, l_anon.49f33f0a2c3e61d603f533e3d400179b.1@PAGEOFF
	mov	x0, x8
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh0, Lloh1
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh6, Lloh7
	.cfi_endproc

	.globl	__RNvCscY7qz2LUPfe_23p4_what_the_law_unlocks15sat_sum_lanes16
	.p2align	2
__RNvCscY7qz2LUPfe_23p4_what_the_law_unlocks15sat_sum_lanes16:
	.cfi_startproc
	and	x9, x1, #0xf
	ands	x10, x1, #0x7ffffffffffffff0
	b.eq	LBB3_4
	and	x8, x1, #0xfffffffffffffff0
	neg	x8, x8
	movi.2d	v0, #0000000000000000
	mov	x11, x0
LBB3_2:
	ldr	q1, [x11], #16
	uqadd.16b	v0, v0, v1
	adds	x8, x8, #16
	b.ne	LBB3_2
	umov.b	w8, v0[0]
	umov.b	w11, v0[1]
	add	w11, w8, w11, uxtb
	mov	w8, #255
	cmp	w11, #255
	csel	w11, w11, w8, lo
	umov.b	w12, v0[2]
	add	w11, w11, w12, uxtb
	cmp	w11, #255
	csel	w11, w11, w8, lo
	umov.b	w12, v0[3]
	add	w11, w11, w12, uxtb
	cmp	w11, #255
	csel	w11, w11, w8, lo
	umov.b	w12, v0[4]
	add	w11, w11, w12, uxtb
	umov.b	w12, v0[5]
	cmp	w11, #255
	csel	w11, w11, w8, lo
	add	w11, w11, w12, uxtb
	cmp	w11, #255
	csel	w11, w11, w8, lo
	umov.b	w12, v0[6]
	add	w11, w11, w12, uxtb
	cmp	w11, #255
	csel	w11, w11, w8, lo
	umov.b	w12, v0[7]
	add	w11, w11, w12, uxtb
	cmp	w11, #255
	csel	w11, w11, w8, lo
	umov.b	w12, v0[8]
	add	w11, w11, w12, uxtb
	cmp	w11, #255
	csel	w11, w11, w8, lo
	umov.b	w12, v0[9]
	add	w11, w11, w12, uxtb
	cmp	w11, #255
	csel	w11, w11, w8, lo
	umov.b	w12, v0[10]
	add	w11, w11, w12, uxtb
	umov.b	w12, v0[11]
	cmp	w11, #255
	csel	w11, w11, w8, lo
	add	w11, w11, w12, uxtb
	cmp	w11, #255
	csel	w11, w11, w8, lo
	umov.b	w12, v0[12]
	add	w11, w11, w12, uxtb
	cmp	w11, #255
	csel	w11, w11, w8, lo
	umov.b	w12, v0[13]
	add	w11, w11, w12, uxtb
	cmp	w11, #255
	csel	w11, w11, w8, lo
	umov.b	w12, v0[14]
	add	w11, w11, w12, uxtb
	cmp	w11, #255
	csel	w11, w11, w8, lo
	umov.b	w12, v0[15]
	add	w11, w11, w12, uxtb
	cmp	w11, #255
	csel	w8, w11, w8, lo
	cbnz	x9, LBB3_5
	b	LBB3_7
LBB3_4:
	mov	w8, #0
	cbz	x9, LBB3_7
LBB3_5:
	add	x10, x0, x10
	mov	w11, #255
LBB3_6:
	ldrb	w12, [x10], #1
	add	w8, w12, w8, uxtb
	cmp	w8, #255
	csel	w8, w8, w11, lo
	subs	x9, x9, #1
	b.ne	LBB3_6
LBB3_7:
	mov	x0, x8
	ret
	.cfi_endproc

	.section	__TEXT,__literal8,8byte_literals
	.p2align	3, 0x0
lCPI4_0:
	.byte	6
	.byte	5
	.byte	255
	.byte	255
	.byte	255
	.byte	255
	.byte	255
	.byte	255
lCPI4_1:
	.byte	4
	.byte	3
	.byte	2
	.byte	1
	.byte	255
	.byte	255
	.byte	255
	.byte	255
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCscY7qz2LUPfe_23p4_what_the_law_unlocks15sat_sum_lanes64
	.p2align	2
__RNvCscY7qz2LUPfe_23p4_what_the_law_unlocks15sat_sum_lanes64:
	.cfi_startproc
	and	x9, x1, #0x3f
	ands	x10, x1, #0x7fffffffffffffc0
	b.eq	LBB4_4
	and	x8, x1, #0xffffffffffffffc0
	neg	x8, x8
	movi.2d	v0, #0000000000000000
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	mov	x11, x0
LBB4_2:
	ldp	q4, q5, [x11]
	uqadd.16b	v3, v3, v4
	uqadd.16b	v2, v2, v5
	ldp	q4, q5, [x11, #32]
	uqadd.16b	v1, v1, v4
	uqadd.16b	v0, v0, v5
	add	x11, x11, #64
	adds	x8, x8, #64
	b.ne	LBB4_2
	umov.b	w8, v3[0]
	umov.b	w11, v2[0]
	umov.b	w12, v1[0]
	umov.b	w13, v0[0]
	umov.b	w14, v3[7]
	umov.b	w15, v2[7]
	umov.b	w16, v1[7]
	umov.b	w17, v0[7]
	umov.b	w1, v3[8]
	umov.b	w2, v2[8]
	umov.b	w3, v1[8]
	umov.b	w4, v0[8]
	add	w11, w8, w11, uxtb
	mov	w8, #255
	cmp	w11, #255
	csel	w11, w11, w8, lo
	add	w12, w12, w13, uxtb
	cmp	w12, #255
	csel	w12, w12, w8, lo
	add	w11, w11, w12
	cmp	w11, #255
	csel	w11, w11, w8, lo
	add	w12, w14, w15, uxtb
	cmp	w12, #255
	csel	w12, w12, w8, lo
	add	w13, w16, w17, uxtb
	cmp	w13, #255
	csel	w13, w13, w8, lo
	add	w14, w1, w2, uxtb
	cmp	w14, #255
	csel	w14, w14, w8, lo
	add	w15, w3, w4, uxtb
	cmp	w15, #255
	csel	w15, w15, w8, lo
	add	w14, w14, w15
	dup.16b	v4, v3[9]
	mov.b	v4[1], w12
	uqadd.8b	v5, v3, v2
	mov.d	v5[1], v5[0]
	cmp	w14, #255
	csel	w12, w14, w8, lo
Lloh8:
	adrp	x14, lCPI4_0@PAGE
Lloh9:
	ldr	d6, [x14, lCPI4_0@PAGEOFF]
	tbl.8b	v7, { v5 }, v6
	zip1.4h	v4, v4, v7
Lloh10:
	adrp	x14, lCPI4_1@PAGE
Lloh11:
	ldr	d7, [x14, lCPI4_1@PAGEOFF]
	tbl.8b	v5, { v5 }, v7
	dup.16b	v16, v2[9]
	mov.b	v16[1], w13
	zip1.2s	v4, v4, v5
	uqadd.8b	v5, v1, v0
	mov.d	v5[1], v5[0]
	tbl.8b	v6, { v5 }, v6
	zip1.4h	v6, v16, v6
	tbl.8b	v5, { v5 }, v7
	zip1.2s	v5, v6, v5
	uqadd.8b	v4, v4, v5
	add	w11, w11, w12
	cmp	w11, #255
	csel	w8, w11, w8, lo
	b	LBB4_5
LBB4_4:
	mov	w8, #0
	movi.2d	v0, #0000000000000000
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
LBB4_5:
	umov.b	w11, v1[9]
	umov.b	w12, v0[9]
	add	w12, w11, w12, uxtb
	mov	w11, #255
	umov.b	w13, v4[0]
	cmp	w12, #255
	csel	w12, w12, w11, lo
	add	w12, w12, w13, uxtb
	cmp	w12, #255
	csel	w12, w12, w11, lo
	umov.b	w13, v3[10]
	umov.b	w14, v2[10]
	add	w13, w13, w14, uxtb
	umov.b	w14, v1[10]
	cmp	w13, #255
	csel	w13, w13, w11, lo
	umov.b	w15, v0[10]
	add	w14, w14, w15, uxtb
	cmp	w14, #255
	csel	w14, w14, w11, lo
	add	w13, w13, w14
	umov.b	w14, v3[11]
	umov.b	w15, v2[11]
	cmp	w13, #255
	csel	w13, w13, w11, lo
	add	w14, w14, w15, uxtb
	cmp	w14, #255
	csel	w14, w14, w11, lo
	umov.b	w15, v1[11]
	umov.b	w16, v0[11]
	add	w15, w15, w16, uxtb
	cmp	w15, #255
	csel	w15, w15, w11, lo
	add	w14, w14, w15
	cmp	w14, #255
	csel	w14, w14, w11, lo
	umov.b	w15, v3[12]
	umov.b	w16, v2[12]
	add	w15, w15, w16, uxtb
	umov.b	w16, v1[12]
	umov.b	w17, v0[12]
	cmp	w15, #255
	csel	w15, w15, w11, lo
	add	w16, w16, w17, uxtb
	cmp	w16, #255
	csel	w16, w16, w11, lo
	add	w15, w15, w16
	cmp	w15, #255
	csel	w15, w15, w11, lo
	umov.b	w16, v3[13]
	umov.b	w17, v2[13]
	add	w16, w16, w17, uxtb
	cmp	w16, #255
	csel	w16, w16, w11, lo
	umov.b	w17, v1[13]
	umov.b	w1, v0[13]
	add	w17, w17, w1, uxtb
	cmp	w17, #255
	csel	w17, w17, w11, lo
	add	w16, w16, w17
	cmp	w16, #255
	csel	w16, w16, w11, lo
	umov.b	w17, v3[14]
	umov.b	w1, v2[14]
	add	w17, w17, w1, uxtb
	cmp	w17, #255
	csel	w17, w17, w11, lo
	umov.b	w1, v1[14]
	umov.b	w2, v0[14]
	add	w1, w1, w2, uxtb
	cmp	w1, #255
	csel	w1, w1, w11, lo
	add	w17, w17, w1
	umov.b	w1, v3[15]
	cmp	w17, #255
	csel	w17, w17, w11, lo
	umov.b	w2, v2[15]
	add	w1, w1, w2, uxtb
	cmp	w1, #255
	csel	w1, w1, w11, lo
	umov.b	w2, v1[15]
	umov.b	w3, v0[15]
	add	w2, w2, w3, uxtb
	cmp	w2, #255
	csel	w2, w2, w11, lo
	add	w1, w1, w2
	cmp	w1, #255
	csel	w1, w1, w11, lo
	umov.b	w2, v4[7]
	add	w12, w12, w2, uxtb
	cmp	w12, #255
	csel	w12, w12, w11, lo
	umov.b	w2, v4[6]
	add	w13, w13, w2, uxtb
	cmp	w13, #255
	csel	w13, w13, w11, lo
	umov.b	w2, v4[5]
	add	w14, w14, w2, uxtb
	cmp	w14, #255
	csel	w14, w14, w11, lo
	umov.b	w2, v4[4]
	add	w15, w15, w2, uxtb
	cmp	w15, #255
	csel	w15, w15, w11, lo
	umov.b	w2, v4[3]
	add	w16, w16, w2, uxtb
	umov.b	w2, v4[2]
	cmp	w16, #255
	csel	w16, w16, w11, lo
	add	w17, w17, w2, uxtb
	cmp	w17, #255
	csel	w17, w17, w11, lo
	umov.b	w2, v4[1]
	add	w1, w1, w2, uxtb
	cmp	w1, #255
	csel	w1, w1, w11, lo
	add	w8, w15, w8, uxtb
	cmp	w8, #255
	csel	w8, w8, w11, lo
	add	w12, w12, w16
	cmp	w12, #255
	csel	w12, w12, w11, lo
	add	w13, w13, w17
	cmp	w13, #255
	csel	w13, w13, w11, lo
	add	w14, w14, w1
	cmp	w14, #255
	csel	w14, w14, w11, lo
	add	w8, w8, w13
	cmp	w8, #255
	csel	w8, w8, w11, lo
	add	w12, w12, w14
	cmp	w12, #255
	csel	w12, w12, w11, lo
	add	w8, w8, w12
	cmp	w8, #255
	csel	w8, w8, w11, lo
	cbz	x9, LBB4_8
	add	x10, x0, x10
LBB4_7:
	ldrb	w12, [x10], #1
	add	w8, w12, w8, uxtb
	cmp	w8, #255
	csel	w8, w8, w11, lo
	subs	x9, x9, #1
	b.ne	LBB4_7
LBB4_8:
	mov	x0, x8
	ret
	.loh AdrpLdr	Lloh10, Lloh11
	.loh AdrpAdrp	Lloh8, Lloh10
	.loh AdrpLdr	Lloh8, Lloh9
	.cfi_endproc

	.section	__TEXT,__literal8,8byte_literals
	.p2align	3, 0x0
lCPI5_0:
	.long	0
	.long	1
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCscY7qz2LUPfe_23p4_what_the_law_unlocks25assoc_holds_u8_saturating
	.p2align	2
__RNvCscY7qz2LUPfe_23p4_what_the_law_unlocks25assoc_holds_u8_saturating:
	.cfi_startproc
	stp	d11, d10, [sp, #-32]!
	.cfi_def_cfa_offset 32
	stp	d9, d8, [sp, #16]
	.cfi_offset b8, -8
	.cfi_offset b9, -16
	.cfi_offset b10, -24
	.cfi_offset b11, -32
	mov	x0, #0
	mov	x1, #0
	mov	w8, #0
Lloh12:
	adrp	x9, lCPI5_0@PAGE
Lloh13:
	ldr	d0, [x9, lCPI5_0@PAGEOFF]
	movi.2s	v1, #2
	mov	w9, #255
	movi	d2, #0x0000ff000000ff
	movi.2s	v3, #4
	movi.2s	v4, #6
	mov	w10, #1
	dup.2d	v5, x10
	movi.2s	v6, #8
LBB5_1:
	mov	w10, #0
	dup.2s	v7, w8
	and.8b	v7, v7, v2
LBB5_2:
	and	w11, w8, #0xff
	add	w11, w11, w10, uxtb
	movi.2d	v16, #0000000000000000
	mov.d	v16[0], x0
	cmp	w11, #255
	csel	w11, w11, w9, lo
	movi.2d	v17, #0000000000000000
	mov.d	v17[0], x1
	dup.2s	v20, w10
	movi.2d	v18, #0000000000000000
	dup.2s	v19, w11
	bic.2s	v19, #1, lsl #8
	mov	w11, #256
	and.8b	v23, v20, v2
	movi.2d	v20, #0000000000000000
	movi.2d	v21, #0000000000000000
	movi.2d	v22, #0000000000000000
	movi.2d	v24, #0000000000000000
	movi.2d	v25, #0000000000000000
	mov.16b	v26, v0
LBB5_3:
	add.2s	v27, v26, v1
	and.8b	v27, v27, v2
	and.8b	v28, v26, v2
	add.2s	v29, v26, v3
	and.8b	v29, v29, v2
	add.2s	v30, v26, v4
	and.8b	v30, v30, v2
	add.2d	v16, v16, v5
	add.2d	v20, v20, v5
	add.2d	v21, v21, v5
	add.2d	v22, v22, v5
	add.2s	v31, v19, v28
	umin.2s	v31, v31, v2
	add.2s	v8, v19, v27
	umin.2s	v8, v8, v2
	add.2s	v9, v19, v29
	umin.2s	v9, v9, v2
	add.2s	v10, v19, v30
	umin.2s	v10, v10, v2
	add.2s	v28, v23, v28
	umin.2s	v28, v28, v2
	add.2s	v27, v23, v27
	umin.2s	v27, v27, v2
	add.2s	v29, v23, v29
	umin.2s	v29, v29, v2
	add.2s	v30, v23, v30
	umin.2s	v30, v30, v2
	add.2s	v28, v7, v28
	umin.2s	v28, v28, v2
	add.2s	v27, v7, v27
	umin.2s	v27, v27, v2
	add.2s	v29, v7, v29
	umin.2s	v29, v29, v2
	add.2s	v30, v7, v30
	umin.2s	v30, v30, v2
	cmeq.2s	v28, v31, v28
	mvn.8b	v28, v28
	ushll.2d	v28, v28, #0
	and.16b	v28, v28, v5
	cmeq.2s	v27, v8, v27
	mvn.8b	v27, v27
	ushll.2d	v27, v27, #0
	and.16b	v27, v27, v5
	cmeq.2s	v29, v9, v29
	mvn.8b	v29, v29
	ushll.2d	v29, v29, #0
	and.16b	v29, v29, v5
	cmeq.2s	v30, v10, v30
	mvn.8b	v30, v30
	ushll.2d	v30, v30, #0
	and.16b	v30, v30, v5
	add.2d	v17, v17, v28
	add.2d	v18, v18, v27
	add.2d	v24, v24, v29
	add.2d	v25, v25, v30
	add.2s	v26, v26, v6
	subs	w11, w11, #8
	b.ne	LBB5_3
	add.2d	v17, v18, v17
	add.2d	v17, v24, v17
	add.2d	v17, v25, v17
	addp.2d	d17, v17
	fmov	x1, d17
	add.2d	v16, v20, v16
	add.2d	v16, v21, v16
	add.2d	v16, v22, v16
	addp.2d	d16, v16
	fmov	x0, d16
	and	w10, w10, #0xff
	add	w10, w10, #1
	tbz	w10, #8, LBB5_2
	and	w8, w8, #0xff
	add	w8, w8, #1
	tbz	w8, #8, LBB5_1
	ldp	d9, d8, [sp, #16]
	ldp	d11, d10, [sp], #32
	.cfi_def_cfa_offset 0
	.cfi_restore b8
	.cfi_restore b9
	.cfi_restore b10
	.cfi_restore b11
	ret
	.loh AdrpLdr	Lloh12, Lloh13
	.cfi_endproc

	.section	__TEXT,__cstring,cstring_literals
l_anon.49f33f0a2c3e61d603f533e3d400179b.0:
	.asciz	"p4_what_the_law_unlocks.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.49f33f0a2c3e61d603f533e3d400179b.1:
	.quad	l_anon.49f33f0a2c3e61d603f533e3d400179b.0
	.asciz	"\032\000\000\000\000\000\000\0003\000\000\000$\000\000"

	.p2align	3, 0x0
l_anon.49f33f0a2c3e61d603f533e3d400179b.2:
	.quad	l_anon.49f33f0a2c3e61d603f533e3d400179b.0
	.asciz	"\032\000\000\000\000\000\000\0004\000\000\000$\000\000"

	.p2align	3, 0x0
l_anon.49f33f0a2c3e61d603f533e3d400179b.3:
	.quad	l_anon.49f33f0a2c3e61d603f533e3d400179b.0
	.asciz	"\032\000\000\000\000\000\000\0005\000\000\000$\000\000"

	.p2align	3, 0x0
l_anon.49f33f0a2c3e61d603f533e3d400179b.4:
	.quad	l_anon.49f33f0a2c3e61d603f533e3d400179b.0
	.asciz	"\032\000\000\000\000\000\000\0006\000\000\000$\000\000"

.subsections_via_symbols
