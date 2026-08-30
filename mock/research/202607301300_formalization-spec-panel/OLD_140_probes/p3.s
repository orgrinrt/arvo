	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_s_i13_min
	.p2align	2
_s_i13_min:
	add	w8, w1, w0
	sxth	w8, w8
	cmn	w8, #1, lsl #12
	mov	w9, #-4096
	csel	w8, w8, w9, gt
	mov	w9, #4095
	cmp	w8, #4095
	csel	w0, w8, w9, lt
	ret

	.globl	_s_i64_exact
	.p2align	2
_s_i64_exact:
	adds	x8, x0, x1
	asr	x9, x8, #63
	eor	x9, x9, #0x8000000000000000
	csel	x0, x9, x8, vs
	ret

	.globl	_s_u13_headroom
	.p2align	2
_s_u13_headroom:
	mov	w8, #8191
	add	w9, w1, w0
	cmp	w9, w8
	csel	w0, w9, w8, lo
	ret

	.globl	_s_u13_min
	.p2align	2
_s_u13_min:
	add	w8, w1, w0
	and	w8, w8, #0xffff
	mov	w9, #8191
	cmp	w8, w9
	csel	w0, w8, w9, lo
	ret

	.globl	_s_u64_exact
	.p2align	2
_s_u64_exact:
	adds	x8, x0, x1
	csinv	x0, x8, xzr, lo
	ret

	.globl	_s_u64_headroom
	.p2align	2
_s_u64_headroom:
	adds	x8, x2, x0
	adcs	xzr, x3, x1
	csinv	x0, x8, xzr, eq
	mov	x1, #0
	ret

	.globl	_vs_u13_headroom
	.p2align	2
_vs_u13_headroom:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.4s	v4, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	movi.4s	v0, #31, msl #8
	umin.4s	v4, v4, v0
	umin.4s	v1, v1, v0
	umin.4s	v2, v2, v0
	umin.4s	v3, v3, v0
	stp	q4, q1, [x0]
	stp	q2, q3, [x0, #32]
	ldp	q1, q2, [x0, #64]
	ldp	q3, q4, [x0, #96]
	ldp	q5, q6, [x1, #64]
	ldp	q7, q16, [x1, #96]
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	add.4s	v4, v16, v4
	umin.4s	v1, v1, v0
	umin.4s	v2, v2, v0
	umin.4s	v3, v3, v0
	umin.4s	v4, v4, v0
	stp	q1, q2, [x0, #64]
	stp	q3, q4, [x0, #96]
	ldp	q1, q2, [x0, #128]
	ldp	q3, q4, [x0, #160]
	ldp	q5, q6, [x1, #128]
	ldp	q7, q16, [x1, #160]
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	add.4s	v4, v16, v4
	umin.4s	v1, v1, v0
	umin.4s	v2, v2, v0
	umin.4s	v3, v3, v0
	umin.4s	v4, v4, v0
	stp	q1, q2, [x0, #128]
	stp	q3, q4, [x0, #160]
	ldp	q1, q2, [x0, #192]
	ldp	q3, q4, [x0, #224]
	ldp	q5, q6, [x1, #192]
	add.4s	v1, v5, v1
	ldp	q5, q7, [x1, #224]
	add.4s	v2, v6, v2
	add.4s	v3, v5, v3
	add.4s	v4, v7, v4
	umin.4s	v1, v1, v0
	umin.4s	v2, v2, v0
	umin.4s	v3, v3, v0
	umin.4s	v0, v4, v0
	stp	q1, q2, [x0, #192]
	stp	q3, q0, [x0, #224]
	ret

	.globl	_vs_u13_min
	.p2align	2
_vs_u13_min:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.8h	v0, v4, v0
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
	mvni.8h	v4, #224, lsl #8
	umin.8h	v0, v0, v4
	umin.8h	v1, v1, v4
	umin.8h	v2, v2, v4
	umin.8h	v3, v3, v4
	stp	q0, q1, [x0]
	stp	q2, q3, [x0, #32]
	ldp	q0, q1, [x0, #64]
	ldp	q2, q3, [x0, #96]
	ldp	q5, q6, [x1, #64]
	ldp	q7, q16, [x1, #96]
	add.8h	v0, v5, v0
	add.8h	v1, v6, v1
	add.8h	v2, v7, v2
	add.8h	v3, v16, v3
	umin.8h	v0, v0, v4
	umin.8h	v1, v1, v4
	umin.8h	v2, v2, v4
	umin.8h	v3, v3, v4
	stp	q0, q1, [x0, #64]
	stp	q2, q3, [x0, #96]
	ret

	.globl	_vs_u64_exact
	.p2align	2
_vs_u64_exact:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	uqadd.2d	v0, v0, v4
	uqadd.2d	v1, v1, v5
	uqadd.2d	v2, v2, v6
	uqadd.2d	v3, v3, v7
	stp	q0, q1, [x0]
	stp	q2, q3, [x0, #32]
	ldp	q0, q1, [x0, #64]
	ldp	q2, q3, [x0, #96]
	ldp	q4, q5, [x1, #64]
	ldp	q6, q7, [x1, #96]
	uqadd.2d	v0, v0, v4
	uqadd.2d	v1, v1, v5
	uqadd.2d	v2, v2, v6
	uqadd.2d	v3, v3, v7
	stp	q0, q1, [x0, #64]
	stp	q2, q3, [x0, #96]
	ldp	q0, q1, [x0, #128]
	ldp	q2, q3, [x0, #160]
	ldp	q4, q5, [x1, #128]
	ldp	q6, q7, [x1, #160]
	uqadd.2d	v0, v0, v4
	uqadd.2d	v1, v1, v5
	uqadd.2d	v2, v2, v6
	uqadd.2d	v3, v3, v7
	stp	q0, q1, [x0, #128]
	stp	q2, q3, [x0, #160]
	ldp	q0, q1, [x0, #192]
	ldp	q2, q3, [x0, #224]
	ldp	q4, q5, [x1, #192]
	ldp	q6, q7, [x1, #224]
	uqadd.2d	v0, v0, v4
	uqadd.2d	v1, v1, v5
	uqadd.2d	v2, v2, v6
	uqadd.2d	v3, v3, v7
	stp	q0, q1, [x0, #192]
	stp	q2, q3, [x0, #224]
	ldp	q0, q1, [x0, #256]
	ldp	q2, q3, [x0, #288]
	ldp	q4, q5, [x1, #256]
	ldp	q6, q7, [x1, #288]
	uqadd.2d	v0, v0, v4
	uqadd.2d	v1, v1, v5
	uqadd.2d	v2, v2, v6
	uqadd.2d	v3, v3, v7
	stp	q0, q1, [x0, #256]
	stp	q2, q3, [x0, #288]
	ldp	q0, q1, [x0, #320]
	ldp	q2, q3, [x0, #352]
	ldp	q4, q5, [x1, #320]
	ldp	q6, q7, [x1, #352]
	uqadd.2d	v0, v0, v4
	uqadd.2d	v1, v1, v5
	uqadd.2d	v2, v2, v6
	uqadd.2d	v3, v3, v7
	stp	q0, q1, [x0, #320]
	stp	q2, q3, [x0, #352]
	ldp	q0, q1, [x0, #384]
	ldp	q2, q3, [x0, #416]
	ldp	q4, q5, [x1, #384]
	ldp	q6, q7, [x1, #416]
	uqadd.2d	v0, v0, v4
	uqadd.2d	v1, v1, v5
	uqadd.2d	v2, v2, v6
	uqadd.2d	v3, v3, v7
	stp	q0, q1, [x0, #384]
	stp	q2, q3, [x0, #416]
	ldp	q0, q1, [x0, #448]
	ldp	q2, q3, [x0, #480]
	ldp	q4, q5, [x1, #448]
	ldp	q6, q7, [x1, #480]
	uqadd.2d	v0, v0, v4
	uqadd.2d	v1, v1, v5
	uqadd.2d	v2, v2, v6
	uqadd.2d	v3, v3, v7
	stp	q0, q1, [x0, #448]
	stp	q2, q3, [x0, #480]
	ret

	.globl	_vs_u64_headroom
	.p2align	2
_vs_u64_headroom:
	stp	x20, x19, [sp, #-16]!
	add	x8, x1, #32
	add	x9, x0, #32
	mov	w10, #64
LBB10_1:
	ldp	x12, x11, [x9, #-32]
	ldp	x14, x13, [x9, #-16]
	ldp	x16, x15, [x9]
	ldp	x0, x17, [x9, #16]
	ldp	x2, x1, [x8, #-32]
	ldp	x4, x3, [x8, #-16]
	ldp	x6, x5, [x8]
	ldp	x19, x7, [x8, #16]
	adds	x12, x2, x12
	adc	x11, x1, x11
	adds	x14, x4, x14
	adc	x13, x3, x13
	adds	x16, x6, x16
	adc	x15, x5, x15
	adds	x0, x19, x0
	adc	x17, x7, x17
	cmp	x11, #0
	csinv	x11, x12, xzr, eq
	cmp	x13, #0
	csinv	x12, x14, xzr, eq
	cmp	x15, #0
	csinv	x13, x16, xzr, eq
	cmp	x17, #0
	csinv	x14, x0, xzr, eq
	stp	x11, xzr, [x9, #-32]
	stp	x12, xzr, [x9, #-16]
	stp	x13, xzr, [x9]
	stp	x14, xzr, [x9, #16]
	add	x8, x8, #64
	add	x9, x9, #64
	subs	x10, x10, #4
	b.ne	LBB10_1
	ldp	x20, x19, [sp], #16
	ret

	.globl	_vw_i13_min
	.p2align	2
_vw_i13_min:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.8h	v0, v4, v0
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
	shl.8h	v0, v0, #3
	sshr.8h	v0, v0, #3
	shl.8h	v1, v1, #3
	sshr.8h	v1, v1, #3
	shl.8h	v2, v2, #3
	sshr.8h	v2, v2, #3
	shl.8h	v3, v3, #3
	sshr.8h	v3, v3, #3
	stp	q0, q1, [x0]
	stp	q2, q3, [x0, #32]
	ldp	q0, q1, [x0, #64]
	ldp	q2, q3, [x0, #96]
	ldp	q4, q5, [x1, #64]
	ldp	q6, q7, [x1, #96]
	add.8h	v0, v4, v0
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
	shl.8h	v0, v0, #3
	sshr.8h	v0, v0, #3
	shl.8h	v1, v1, #3
	sshr.8h	v1, v1, #3
	shl.8h	v2, v2, #3
	sshr.8h	v2, v2, #3
	shl.8h	v3, v3, #3
	sshr.8h	v3, v3, #3
	stp	q0, q1, [x0, #64]
	stp	q2, q3, [x0, #96]
	ret

	.globl	_w_i13_headroom
	.p2align	2
_w_i13_headroom:
	add	w8, w1, w0
	sbfx	w0, w8, #0, #13
	ret

	.globl	_w_i13_min
	.p2align	2
_w_i13_min:
	add	w8, w1, w0
	sbfx	w0, w8, #0, #13
	ret

	.globl	_w_i16_exact
	.p2align	2
_w_i16_exact:
	add	w8, w1, w0
	sxth	w0, w8
	ret

.subsections_via_symbols
