	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_c_fit_w16_a256
	.p2align	2
_c_fit_w16_a256:
	.cfi_startproc
	mov	x8, x0
	mov	w0, #0
	add	x8, x8, #32
	mov	w9, #256
	mov	w10, #65535
	mov	w11, #7937
LBB0_1:
	movi.2d	v0, #0000000000000000
	mov	x12, x8
	mov	w13, #256
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB0_2:
	ldp	q4, q5, [x12, #-32]
	ldp	q6, q7, [x12], #64
	uaddw.4s	v0, v0, v4
	uaddw2.4s	v0, v0, v4
	uaddw.4s	v1, v1, v5
	uaddw2.4s	v1, v1, v5
	uaddw.4s	v2, v2, v6
	uaddw2.4s	v2, v2, v6
	uaddw.4s	v3, v3, v7
	uaddw2.4s	v3, v3, v7
	subs	x13, x13, #32
	b.ne	LBB0_2
	add.4s	v0, v1, v0
	add.4s	v1, v3, v2
	add.4s	v0, v1, v0
	addv.4s	s0, v0
	fmov	w12, s0
	cmp	w12, w10
	csel	w12, w12, w10, lo
	eor	w0, w12, w0
	add	x8, x8, #512
	cmp	x9, x11
	add	x9, x9, #256
	b.lo	LBB0_1
	ret
	.cfi_endproc

	.globl	_c_fit_w16_a4
	.p2align	2
_c_fit_w16_a4:
	.cfi_startproc
	movi.2d	v0, #0000000000000000
	movi.2d	v1, #0x00ffff0000ffff
	add	x8, x0, #128
	movi.2d	v2, #0000000000000000
	mov	w9, #2048
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
	movi.2d	v5, #0000000000000000
	movi.2d	v6, #0000000000000000
	movi.2d	v7, #0000000000000000
	movi.2d	v16, #0000000000000000
LBB1_1:
	sub	x10, x8, #128
	sub	x11, x8, #64
	ld4.8h	{ v17, v18, v19, v20 }, [x10]
	ld4.8h	{ v21, v22, v23, v24 }, [x11]
	mov	x10, x8
	uaddl2.4s	v25, v17, v18
	uaddl.4s	v26, v17, v18
	uaddw.4s	v26, v26, v19
	uaddw2.4s	v25, v25, v19
	uaddw2.4s	v25, v25, v20
	uaddw.4s	v17, v26, v20
	ld4.8h	{ v26, v27, v28, v29 }, [x10], #64
	uaddl2.4s	v18, v21, v22
	uaddl.4s	v19, v21, v22
	uaddw.4s	v19, v19, v23
	uaddw2.4s	v18, v18, v23
	uaddw2.4s	v18, v18, v24
	uaddw.4s	v19, v19, v24
	uaddl2.4s	v20, v26, v27
	uaddl.4s	v21, v26, v27
	uaddw.4s	v21, v21, v28
	uaddw2.4s	v20, v20, v28
	uaddw2.4s	v20, v20, v29
	uaddw.4s	v21, v21, v29
	ld4.8h	{ v26, v27, v28, v29 }, [x10]
	uaddl2.4s	v22, v26, v27
	uaddl.4s	v23, v26, v27
	uaddw.4s	v23, v23, v28
	uaddw2.4s	v22, v22, v28
	uaddw2.4s	v22, v22, v29
	uaddw.4s	v23, v23, v29
	umin.4s	v24, v25, v1
	eor.16b	v3, v24, v3
	umin.4s	v17, v17, v1
	eor.16b	v2, v17, v2
	umin.4s	v17, v18, v1
	eor.16b	v5, v17, v5
	umin.4s	v17, v19, v1
	eor.16b	v4, v17, v4
	umin.4s	v17, v20, v1
	eor.16b	v7, v17, v7
	umin.4s	v17, v21, v1
	eor.16b	v6, v17, v6
	umin.4s	v17, v22, v1
	eor.16b	v0, v17, v0
	umin.4s	v17, v23, v1
	eor.16b	v16, v17, v16
	add	x8, x8, #256
	subs	x9, x9, #32
	b.ne	LBB1_1
	eor.16b	v1, v5, v3
	eor3.16b	v2, v4, v2, v6
	eor3.16b	v0, v7, v1, v0
	eor3.16b	v0, v16, v2, v0
	ext.16b	v1, v0, v0, #8
	eor.8b	v0, v0, v1
	fmov	x8, d0
	lsr	x9, x8, #32
	eor	w0, w8, w9
	ret
	.cfi_endproc

	.globl	_c_head_w16_a256
	.p2align	2
_c_head_w16_a256:
	.cfi_startproc
	mov	x8, x0
	mov	x9, #0
	mov	w0, #0
	mov	w10, #65535
	mov	w11, #7937
LBB2_1:
	ldp	q0, q1, [x8]
	ldp	q2, q3, [x8, #32]
	ldp	q4, q5, [x8, #64]
	ldp	q6, q7, [x8, #96]
	add.4s	v0, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	ldp	q4, q5, [x8, #128]
	ldp	q6, q7, [x8, #160]
	ldp	q16, q17, [x8, #192]
	ldp	q18, q19, [x8, #224]
	add.4s	v4, v16, v4
	add.4s	v0, v4, v0
	add.4s	v4, v17, v5
	add.4s	v1, v4, v1
	add.4s	v4, v18, v6
	add.4s	v2, v4, v2
	add.4s	v4, v19, v7
	add.4s	v3, v4, v3
	ldp	q4, q5, [x8, #256]
	ldp	q6, q7, [x8, #288]
	ldp	q16, q17, [x8, #320]
	ldp	q18, q19, [x8, #352]
	add.4s	v4, v16, v4
	add.4s	v5, v17, v5
	add.4s	v6, v18, v6
	add.4s	v7, v19, v7
	ldp	q16, q17, [x8, #384]
	ldp	q18, q19, [x8, #416]
	add.4s	v4, v16, v4
	add.4s	v0, v4, v0
	add.4s	v4, v17, v5
	add.4s	v1, v4, v1
	add.4s	v4, v18, v6
	add.4s	v2, v4, v2
	add.4s	v4, v19, v7
	add.4s	v3, v4, v3
	ldp	q4, q5, [x8, #448]
	ldp	q6, q7, [x8, #480]
	ldp	q16, q17, [x8, #512]
	ldp	q18, q19, [x8, #544]
	add.4s	v4, v16, v4
	add.4s	v5, v17, v5
	add.4s	v6, v18, v6
	add.4s	v7, v19, v7
	ldp	q16, q17, [x8, #576]
	ldp	q18, q19, [x8, #608]
	add.4s	v4, v16, v4
	add.4s	v5, v17, v5
	add.4s	v6, v18, v6
	add.4s	v7, v19, v7
	ldp	q16, q17, [x8, #640]
	add.4s	v4, v16, v4
	ldp	q16, q18, [x8, #672]
	add.4s	v0, v4, v0
	add.4s	v4, v17, v5
	add.4s	v1, v4, v1
	add.4s	v4, v16, v6
	add.4s	v2, v4, v2
	add.4s	v4, v18, v7
	add.4s	v3, v4, v3
	ldp	q4, q5, [x8, #704]
	ldp	q6, q7, [x8, #768]
	add.4s	v4, v6, v4
	add.4s	v5, v7, v5
	ldp	q6, q7, [x8, #736]
	ldp	q16, q17, [x8, #800]
	add.4s	v6, v16, v6
	add.4s	v7, v17, v7
	ldp	q16, q17, [x8, #832]
	add.4s	v4, v16, v4
	add.4s	v5, v17, v5
	ldp	q16, q17, [x8, #864]
	add.4s	v6, v16, v6
	add.4s	v7, v17, v7
	ldp	q16, q17, [x8, #896]
	add.4s	v4, v16, v4
	add.4s	v5, v17, v5
	ldp	q16, q17, [x8, #928]
	add.4s	v6, v16, v6
	add.4s	v7, v17, v7
	ldp	q16, q17, [x8, #960]
	add.4s	v4, v16, v4
	add.4s	v0, v4, v0
	add.4s	v4, v17, v5
	add.4s	v1, v4, v1
	ldp	q4, q5, [x8, #992]
	add.4s	v4, v4, v6
	add.4s	v2, v4, v2
	add.4s	v4, v5, v7
	add.4s	v3, v4, v3
	add.4s	v0, v1, v0
	add.4s	v0, v2, v0
	add.4s	v0, v3, v0
	addv.4s	s0, v0
	fmov	w12, s0
	cmp	w12, w10
	csel	w12, w12, w10, lo
	eor	w0, w12, w0
	add	x9, x9, #256
	add	x8, x8, #1024
	cmp	x9, x11
	b.lo	LBB2_1
	ret
	.cfi_endproc

	.globl	_c_lanes_w16_a256
	.p2align	2
_c_lanes_w16_a256:
	.cfi_startproc
	mov	x9, #0
	mov	w8, #0
	add	x10, x0, #256
	mov	w11, #65535
	mov	w12, #7937
LBB3_1:
	ldp	q0, q1, [x10, #-256]
	uqadd.8h	v0, v0, v1
	ldp	q1, q2, [x10, #-224]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #-192]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #-160]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #-128]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #-96]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #-64]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #-32]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #32]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #64]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #96]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #128]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #160]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #192]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	ldp	q1, q2, [x10, #224]
	uqadd.8h	v0, v0, v1
	uqadd.8h	v0, v0, v2
	umov.h	w13, v0[0]
	umov.h	w14, v0[1]
	add	w13, w13, w14, uxth
	cmp	w13, w11
	csel	w13, w13, w11, lo
	umov.h	w14, v0[2]
	add	w13, w13, w14, uxth
	cmp	w13, w11
	csel	w13, w13, w11, lo
	umov.h	w14, v0[3]
	add	w13, w13, w14, uxth
	cmp	w13, w11
	csel	w13, w13, w11, lo
	umov.h	w14, v0[4]
	add	w13, w13, w14, uxth
	umov.h	w14, v0[5]
	cmp	w13, w11
	csel	w13, w13, w11, lo
	add	w13, w13, w14, uxth
	umov.h	w14, v0[6]
	cmp	w13, w11
	csel	w13, w13, w11, lo
	add	w13, w13, w14, uxth
	cmp	w13, w11
	csel	w13, w13, w11, lo
	umov.h	w14, v0[7]
	add	w13, w13, w14, uxth
	cmp	w13, w11
	csel	w13, w13, w11, lo
	eor	w8, w13, w8
	add	x9, x9, #256
	add	x10, x10, #512
	cmp	x9, x12
	b.lo	LBB3_1
	and	x0, x8, #0xffff
	ret
	.cfi_endproc

	.globl	_c_lanes_w64_a16
	.p2align	2
_c_lanes_w64_a16:
	.cfi_startproc
	mov	x8, x0
	mov	x9, #0
	mov	x0, #0
	add	x8, x8, #64
	mov	w10, #8177
LBB4_1:
	ldp	x11, x12, [x8, #-64]
	ldp	x13, x14, [x8, #-48]
	ldp	x15, x16, [x8, #-32]
	ldp	x17, x1, [x8, #-16]
	ldp	x2, x3, [x8]
	adds	x11, x11, x2
	csinv	x11, x11, xzr, lo
	adds	x12, x12, x3
	csinv	x12, x12, xzr, lo
	ldp	x2, x3, [x8, #16]
	adds	x13, x13, x2
	csinv	x13, x13, xzr, lo
	adds	x14, x14, x3
	csinv	x14, x14, xzr, lo
	ldp	x2, x3, [x8, #32]
	adds	x15, x15, x2
	csinv	x15, x15, xzr, lo
	adds	x16, x16, x3
	csinv	x16, x16, xzr, lo
	ldp	x2, x3, [x8, #48]
	adds	x17, x17, x2
	csinv	x17, x17, xzr, lo
	adds	x1, x1, x3
	csinv	x1, x1, xzr, lo
	adds	x11, x11, x12
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x13
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x14
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x15
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x16
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x17
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x1
	csinv	x11, x11, xzr, lo
	eor	x0, x11, x0
	add	x9, x9, #16
	add	x8, x8, #128
	cmp	x9, x10
	b.lo	LBB4_1
	ret
	.cfi_endproc

	.globl	_c_min_w16_a256
	.p2align	2
_c_min_w16_a256:
	.cfi_startproc
	mov	w8, #0
	mov	w10, #256
	mov	w9, #65535
	mov	w11, #7937
LBB5_1:
	mov	x13, #0
	mov	w12, #0
LBB5_2:
	ldrh	w14, [x0, x13]
	add	x13, x13, #2
	add	w12, w14, w12, uxth
	cmp	w12, w9
	csel	w12, w12, w9, lo
	cmp	x13, #512
	b.ne	LBB5_2
	eor	w8, w12, w8
	add	x0, x0, #512
	cmp	x10, x11
	add	x10, x10, #256
	b.lo	LBB5_1
	and	x0, x8, #0xffff
	ret
	.cfi_endproc

	.globl	_c_min_w16_a4
	.p2align	2
_c_min_w16_a4:
	.cfi_startproc
	add	x8, x0, #128
	movi.2d	v0, #0000000000000000
	mov	w9, #2048
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB6_1:
	sub	x10, x8, #128
	sub	x11, x8, #64
	ld4.8h	{ v4, v5, v6, v7 }, [x10]
	ld4.8h	{ v16, v17, v18, v19 }, [x11]
	mov	x10, x8
	ld4.8h	{ v20, v21, v22, v23 }, [x10], #64
	ld4.8h	{ v24, v25, v26, v27 }, [x10]
	uqadd.8h	v28, v4, v5
	uqadd.8h	v29, v16, v17
	uqadd.8h	v30, v20, v21
	uqadd.8h	v31, v24, v25
	uqadd.8h	v28, v28, v6
	uqadd.8h	v29, v29, v18
	uqadd.8h	v30, v30, v22
	uqadd.8h	v31, v31, v26
	uqadd.8h	v4, v28, v7
	uqadd.8h	v5, v29, v19
	uqadd.8h	v6, v30, v23
	uqadd.8h	v7, v31, v27
	eor.16b	v0, v4, v0
	eor.16b	v1, v5, v1
	eor.16b	v2, v6, v2
	eor.16b	v3, v7, v3
	add	x8, x8, #256
	subs	x9, x9, #32
	b.ne	LBB6_1
	eor.16b	v0, v1, v0
	eor3.16b	v0, v2, v0, v3
	ext.16b	v1, v0, v0, #8
	eor.8b	v0, v0, v1
	fmov	x8, d0
	lsr	x9, x8, #32
	eor	w8, w8, w9
	eor	w8, w8, w8, lsr #16
	and	x0, x8, #0xffff
	ret
	.cfi_endproc

	.globl	_c_min_w64_a16
	.p2align	2
_c_min_w64_a16:
	.cfi_startproc
	mov	x8, x0
	mov	x9, #0
	mov	x0, #0
	add	x8, x8, #64
	mov	w10, #8177
LBB7_1:
	ldp	x11, x12, [x8, #-64]
	adds	x11, x11, x12
	csinv	x11, x11, xzr, lo
	ldp	x12, x13, [x8, #-48]
	adds	x11, x11, x12
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x13
	csinv	x11, x11, xzr, lo
	ldp	x12, x13, [x8, #-32]
	adds	x11, x11, x12
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x13
	csinv	x11, x11, xzr, lo
	ldp	x12, x13, [x8, #-16]
	adds	x11, x11, x12
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x13
	csinv	x11, x11, xzr, lo
	ldp	x12, x13, [x8]
	adds	x11, x11, x12
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x13
	csinv	x11, x11, xzr, lo
	ldp	x12, x13, [x8, #16]
	adds	x11, x11, x12
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x13
	csinv	x11, x11, xzr, lo
	ldp	x12, x13, [x8, #32]
	adds	x11, x11, x12
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x13
	csinv	x11, x11, xzr, lo
	ldp	x12, x13, [x8, #48]
	adds	x11, x11, x12
	csinv	x11, x11, xzr, lo
	adds	x11, x11, x13
	csinv	x11, x11, xzr, lo
	eor	x0, x11, x0
	add	x9, x9, #16
	add	x8, x8, #128
	cmp	x9, x10
	b.lo	LBB7_1
	ret
	.cfi_endproc

.subsections_via_symbols
