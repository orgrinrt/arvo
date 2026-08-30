	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__RINvCshtEs8sf6Dux_30p4_what_the_lifted_arm_unlocks14lanes4_indexedINtB2_6NonNegKh0_Kh7f_EEB2_:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	cmp	x1, #4
	b.hs	LBB0_5
	mov	x9, #0
	mov	w8, #0
	subs	x10, x1, x9
	b.ls	LBB0_12
LBB0_2:
	mov	w11, #0
	add	x9, x0, x9
	mov	w12, #127
	mov	w13, #-128
LBB0_3:
	ldrsb	w14, [x9], #1
	add	w11, w14, w11, sxtb
	cmp	w11, #127
	csel	w11, w11, w12, lt
	cmn	w11, #128
	csel	w11, w11, w13, gt
	subs	x10, x10, #1
	b.ne	LBB0_3
	sxtb	w8, w8
	add	w8, w8, w11, sxtb
	mov	w9, #127
	cmp	w8, #127
	csel	w8, w8, w9, lt
	cmn	w8, #128
	mov	w9, #-128
	csel	w0, w8, w9, gt
	ldp	x29, x30, [sp], #16
	ret
LBB0_5:
	mov	x8, #0
	mov	w10, #0
	mov	w11, #0
	movi.2d	v0, #0000000000000000
	mov	w12, #127
	mov	w13, #-128
LBB0_6:
	cmp	x8, x1
	b.hs	LBB0_16
	ldrsb	w9, [x0, x8]
	add	w9, w9, w11, sxtb
	cmp	w9, #127
	csel	w9, w9, w12, lt
	cmn	w9, #128
	csel	w11, w9, w13, gt
	add	x9, x8, #1
	cmp	x9, x1
	b.hs	LBB0_15
	add	x9, x0, x8
	ldrsb	w9, [x9, #1]
	add	w9, w9, w10, sxtb
	cmp	w9, #127
	csel	w9, w9, w12, lt
	cmn	w9, #128
	csel	w10, w9, w13, gt
	add	x9, x8, #2
	cmp	x9, x1
	b.hs	LBB0_14
	add	x9, x8, #3
	cmp	x9, x1
	b.hs	LBB0_13
	add	x9, x0, x8
	ldr	h1, [x9, #2]
	ushll.8h	v1, v1, #0
	ushll.4s	v1, v1, #0
	shl.2s	v1, v1, #24
	shl.2s	v0, v0, #24
	sqadd.2s	v0, v0, v1
	sshr.2s	v0, v0, #24
	add	x9, x8, #4
	add	x14, x8, #8
	mov	x8, x9
	cmp	x14, x1
	b.ls	LBB0_6
	sxtb	w8, w11
	add	w8, w8, w10, sxtb
	mov	w10, #127
	cmp	w8, #127
	csel	w8, w8, w10, lt
	cmn	w8, #128
	mov	w11, #-128
	csel	w8, w8, w11, gt
	fmov	w12, s0
	mov.s	w13, v0[1]
	add	w12, w12, w13
	cmp	w12, #127
	csel	w12, w12, w10, lt
	cmn	w12, #128
	csel	w12, w12, w11, gt
	add	w8, w8, w12
	cmp	w8, #127
	csel	w8, w8, w10, lt
	cmn	w8, #128
	csel	w8, w8, w11, gt
	subs	x10, x1, x9
	b.hi	LBB0_2
LBB0_12:
	mov	x0, x8
	ldp	x29, x30, [sp], #16
	ret
LBB0_13:
Lloh0:
	adrp	x2, l_anon.73a000eae2a2070bd5bfc3d33ee39a13.4@PAGE
Lloh1:
	add	x2, x2, l_anon.73a000eae2a2070bd5bfc3d33ee39a13.4@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB0_14:
Lloh2:
	adrp	x2, l_anon.73a000eae2a2070bd5bfc3d33ee39a13.3@PAGE
Lloh3:
	add	x2, x2, l_anon.73a000eae2a2070bd5bfc3d33ee39a13.3@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB0_15:
Lloh4:
	adrp	x2, l_anon.73a000eae2a2070bd5bfc3d33ee39a13.2@PAGE
Lloh5:
	add	x2, x2, l_anon.73a000eae2a2070bd5bfc3d33ee39a13.2@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB0_16:
Lloh6:
	adrp	x2, l_anon.73a000eae2a2070bd5bfc3d33ee39a13.1@PAGE
Lloh7:
	add	x2, x2, l_anon.73a000eae2a2070bd5bfc3d33ee39a13.1@PAGEOFF
	mov	x0, x8
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh0, Lloh1
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh6, Lloh7

	.p2align	2
__RINvCshtEs8sf6Dux_30p4_what_the_lifted_arm_unlocks15lanes16_chunkedINtB2_6NonNegKh0_Kh7f_EEB2_:
	and	x8, x1, #0xf
	ands	x10, x1, #0x7ffffffffffffff0
	b.eq	LBB1_4
	and	x9, x1, #0xfffffffffffffff0
	neg	x9, x9
	movi.2d	v0, #0000000000000000
	mov	x11, x0
LBB1_2:
	ldr	q1, [x11], #16
	sqadd.16b	v0, v0, v1
	adds	x9, x9, #16
	b.ne	LBB1_2
	umov.b	w9, v0[0]
	cbnz	x8, LBB1_5
	b	LBB1_8
LBB1_4:
	movi.2d	v0, #0000000000000000
	umov.b	w9, v0[0]
	cbz	x8, LBB1_8
LBB1_5:
	mov	w11, #0
	add	x10, x0, x10
	mov	w12, #127
	mov	w13, #-128
LBB1_6:
	ldrsb	w14, [x10], #1
	add	w11, w14, w11, sxtb
	cmp	w11, #127
	csel	w11, w11, w12, lt
	cmn	w11, #128
	csel	w11, w11, w13, gt
	subs	x8, x8, #1
	b.ne	LBB1_6
	sxtb	w8, w11
	add	w8, w8, w9, sxtb
	mov	w9, #127
	cmp	w8, #127
	csel	w8, w8, w9, lt
	cmn	w8, #128
	mov	w9, #-128
	csel	w9, w8, w9, gt
LBB1_8:
	umov.b	w8, v0[1]
	sxtb	w9, w9
	add	w9, w9, w8, sxtb
	mov	w8, #127
	cmp	w9, #127
	csel	w10, w9, w8, lt
	cmn	w10, #128
	mov	w9, #-128
	csel	w10, w10, w9, gt
	umov.b	w11, v0[2]
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	csel	w10, w10, w9, gt
	umov.b	w11, v0[3]
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	umov.b	w11, v0[4]
	csel	w10, w10, w9, gt
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	csel	w10, w10, w9, gt
	umov.b	w11, v0[5]
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	csel	w10, w10, w9, gt
	umov.b	w11, v0[6]
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	csel	w10, w10, w9, gt
	umov.b	w11, v0[7]
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	csel	w10, w10, w9, gt
	umov.b	w11, v0[8]
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	csel	w10, w10, w9, gt
	umov.b	w11, v0[9]
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	umov.b	w11, v0[10]
	csel	w10, w10, w9, gt
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	csel	w10, w10, w9, gt
	umov.b	w11, v0[11]
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	csel	w10, w10, w9, gt
	umov.b	w11, v0[12]
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	csel	w10, w10, w9, gt
	umov.b	w11, v0[13]
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	csel	w10, w10, w9, gt
	umov.b	w11, v0[14]
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w10, w10, w8, lt
	cmn	w10, #128
	csel	w10, w10, w9, gt
	umov.b	w11, v0[15]
	add	w10, w10, w11, sxtb
	cmp	w10, #127
	csel	w8, w10, w8, lt
	cmn	w8, #128
	csel	w0, w8, w9, gt
	ret

	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB2_1:
	b	LBB2_1

	.globl	_sat_sum_lanes16_nonneg
	.p2align	2
_sat_sum_lanes16_nonneg:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	__RINvCshtEs8sf6Dux_30p4_what_the_lifted_arm_unlocks15lanes16_chunkedINtB2_6NonNegKh0_Kh7f_EEB2_
	sxtb	w0, w0
	ldp	x29, x30, [sp], #16
	ret

	.globl	_sat_sum_lanes4_nonneg
	.p2align	2
_sat_sum_lanes4_nonneg:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	__RINvCshtEs8sf6Dux_30p4_what_the_lifted_arm_unlocks14lanes4_indexedINtB2_6NonNegKh0_Kh7f_EEB2_
	sxtb	w0, w0
	ldp	x29, x30, [sp], #16
	ret

	.globl	_sat_sum_seq
	.p2align	2
_sat_sum_seq:
	mov	w8, #0
	cbz	x1, LBB5_3
	mov	w9, #127
	mov	w10, #-128
LBB5_2:
	ldrsb	w11, [x0], #1
	add	w8, w11, w8, sxtb
	cmp	w8, #127
	csel	w8, w8, w9, lt
	cmn	w8, #128
	csel	w8, w8, w10, gt
	subs	x1, x1, #1
	b.ne	LBB5_2
LBB5_3:
	sxtb	w0, w8
	ret

	.globl	_sat_sum_seq_chunked_no_law
	.p2align	2
_sat_sum_seq_chunked_no_law:
	and	x8, x1, #0xf
	mov	w14, #0
	ands	x9, x1, #0x7ffffffffffffff0
	b.eq	LBB6_3
	and	x10, x1, #0x7ffffffffffffff0
	neg	x10, x10
	mov	w11, #127
	mov	w12, #-128
	mov	x13, x0
LBB6_2:
	ldrsb	w15, [x13]
	add	w14, w15, w14, sxtb
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #1]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #2]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #3]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #4]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #5]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #6]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #7]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #8]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #9]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #10]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #11]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #12]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #13]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #14]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	ldrsb	w15, [x13, #15]
	add	w14, w14, w15
	cmp	w14, #127
	csel	w14, w14, w11, lt
	cmn	w14, #128
	csel	w14, w14, w12, gt
	add	x13, x13, #16
	adds	x10, x10, #16
	b.ne	LBB6_2
LBB6_3:
	cbz	x8, LBB6_6
	add	x9, x0, x9
	mov	w10, #127
	mov	w11, #-128
LBB6_5:
	ldrsb	w12, [x9], #1
	add	w12, w12, w14, sxtb
	cmp	w12, #127
	csel	w12, w12, w10, lt
	cmn	w12, #128
	csel	w14, w12, w11, gt
	subs	x8, x8, #1
	b.ne	LBB6_5
LBB6_6:
	sxtb	w0, w14
	ret

	.globl	_wrap_sum_seq
	.p2align	2
_wrap_sum_seq:
	cbz	x1, LBB7_3
	cmp	x1, #8
	b.hs	LBB7_4
	mov	w9, #0
	mov	x8, #0
	b	LBB7_13
LBB7_3:
	mov	w9, #0
	sxtb	w0, w9
	ret
LBB7_4:
	cmp	x1, #64
	b.hs	LBB7_6
	mov	x8, #0
	mov	w9, #0
	b	LBB7_10
LBB7_6:
	and	x10, x1, #0x38
	and	x8, x1, #0xffffffffffffffc0
	add	x9, x0, #32
	movi.2d	v0, #0000000000000000
	and	x11, x1, #0xffffffffffffffc0
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB7_7:
	ldp	q4, q5, [x9, #-32]
	ldp	q6, q7, [x9], #64
	add.16b	v0, v4, v0
	add.16b	v1, v5, v1
	add.16b	v2, v6, v2
	add.16b	v3, v7, v3
	subs	x11, x11, #64
	b.ne	LBB7_7
	add.16b	v0, v1, v0
	add.16b	v0, v2, v0
	add.16b	v0, v3, v0
	addv.16b	b0, v0
	fmov	w9, s0
	cmp	x1, x8
	b.eq	LBB7_15
	cbz	x10, LBB7_13
LBB7_10:
	mov	x10, x8
	and	x8, x1, #0xfffffffffffffff8
	movi.2d	v0, #0000000000000000
	mov.b	v0[0], w9
	sub	x9, x10, x8
	add	x10, x0, x10
LBB7_11:
	ldr	d1, [x10], #8
	add.8b	v0, v1, v0
	adds	x9, x9, #8
	b.ne	LBB7_11
	addv.8b	b0, v0
	fmov	w9, s0
	cmp	x1, x8
	b.eq	LBB7_15
LBB7_13:
	sub	x10, x1, x8
	add	x8, x0, x8
LBB7_14:
	ldrb	w11, [x8], #1
	add	w9, w11, w9
	subs	x10, x10, #1
	b.ne	LBB7_14
LBB7_15:
	sxtb	w0, w9
	ret

	.section	__TEXT,__cstring,cstring_literals
l_anon.73a000eae2a2070bd5bfc3d33ee39a13.0:
	.asciz	"p4_what_the_lifted_arm_unlocks.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.73a000eae2a2070bd5bfc3d33ee39a13.1:
	.quad	l_anon.73a000eae2a2070bd5bfc3d33ee39a13.0
	.asciz	"!\000\000\000\000\000\000\000U\000\000\000\032\000\000"

	.p2align	3, 0x0
l_anon.73a000eae2a2070bd5bfc3d33ee39a13.2:
	.quad	l_anon.73a000eae2a2070bd5bfc3d33ee39a13.0
	.asciz	"!\000\000\000\000\000\000\000V\000\000\000\032\000\000"

	.p2align	3, 0x0
l_anon.73a000eae2a2070bd5bfc3d33ee39a13.3:
	.quad	l_anon.73a000eae2a2070bd5bfc3d33ee39a13.0
	.asciz	"!\000\000\000\000\000\000\000W\000\000\000\032\000\000"

	.p2align	3, 0x0
l_anon.73a000eae2a2070bd5bfc3d33ee39a13.4:
	.quad	l_anon.73a000eae2a2070bd5bfc3d33ee39a13.0
	.asciz	"!\000\000\000\000\000\000\000X\000\000\000\032\000\000"

	.globl	_sat_sum_lanes16_nonpos
_sat_sum_lanes16_nonpos = _sat_sum_lanes16_nonneg
	.globl	_sat_sum_lanes16_smallgain
_sat_sum_lanes16_smallgain = _sat_sum_lanes16_nonneg
.subsections_via_symbols
