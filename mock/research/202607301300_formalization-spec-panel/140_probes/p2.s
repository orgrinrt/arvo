	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_v_w13_headroom
	.p2align	2
_v_w13_headroom:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.4s	v4, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	movi.4s	v0, #31, msl #8
	and.16b	v4, v4, v0
	and.16b	v1, v1, v0
	and.16b	v2, v2, v0
	and.16b	v3, v3, v0
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
	and.16b	v1, v1, v0
	and.16b	v2, v2, v0
	and.16b	v3, v3, v0
	and.16b	v4, v4, v0
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
	and.16b	v1, v1, v0
	and.16b	v2, v2, v0
	and.16b	v3, v3, v0
	and.16b	v4, v4, v0
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
	and.16b	v1, v1, v0
	and.16b	v2, v2, v0
	and.16b	v3, v3, v0
	and.16b	v0, v4, v0
	stp	q1, q2, [x0, #192]
	stp	q3, q0, [x0, #224]
	ret

	.globl	_v_w13_min
	.p2align	2
_v_w13_min:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.8h	v0, v4, v0
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
	bic.8h	v0, #224, lsl #8
	bic.8h	v1, #224, lsl #8
	bic.8h	v2, #224, lsl #8
	bic.8h	v3, #224, lsl #8
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
	bic.8h	v0, #224, lsl #8
	bic.8h	v1, #224, lsl #8
	bic.8h	v2, #224, lsl #8
	bic.8h	v3, #224, lsl #8
	stp	q0, q1, [x0, #64]
	stp	q2, q3, [x0, #96]
	ret

	.globl	_v_w13_reduce_eager
	.p2align	2
_v_w13_reduce_eager:
	ldp	q0, q1, [x0, #64]
	ldp	q2, q3, [x0]
	ldp	q4, q5, [x0, #96]
	ldp	q6, q7, [x0, #32]
	add.8h	v5, v7, v5
	add.8h	v1, v3, v1
	add.8h	v1, v1, v5
	add.8h	v3, v6, v4
	add.8h	v0, v2, v0
	add.8h	v0, v0, v3
	add.8h	v0, v0, v1
	addv.8h	h0, v0
	fmov	w8, s0
	and	w0, w8, #0x1fff
	ret

	.globl	_v_w13_three_eager
	.p2align	2
_v_w13_three_eager:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.8h	v0, v4, v0
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
	ldp	q16, q17, [x2]
	ldp	q18, q19, [x2, #32]
	neg.8h	v4, v4
	mla.8h	v4, v0, v16
	neg.8h	v0, v5
	mla.8h	v0, v1, v17
	neg.8h	v1, v6
	mla.8h	v1, v2, v18
	neg.8h	v2, v7
	mla.8h	v2, v3, v19
	bic.8h	v4, #224, lsl #8
	bic.8h	v0, #224, lsl #8
	bic.8h	v1, #224, lsl #8
	bic.8h	v2, #224, lsl #8
	stp	q4, q0, [x0]
	stp	q1, q2, [x0, #32]
	ldp	q0, q1, [x0, #64]
	ldp	q2, q3, [x0, #96]
	ldp	q4, q5, [x1, #64]
	ldp	q6, q7, [x1, #96]
	add.8h	v0, v4, v0
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
	ldp	q16, q17, [x2, #64]
	ldp	q18, q19, [x2, #96]
	neg.8h	v4, v4
	mla.8h	v4, v0, v16
	neg.8h	v0, v5
	mla.8h	v0, v1, v17
	neg.8h	v1, v6
	mla.8h	v1, v2, v18
	neg.8h	v2, v7
	mla.8h	v2, v3, v19
	bic.8h	v4, #224, lsl #8
	bic.8h	v0, #224, lsl #8
	bic.8h	v1, #224, lsl #8
	bic.8h	v2, #224, lsl #8
	stp	q4, q0, [x0, #64]
	stp	q1, q2, [x0, #96]
	ret

	.globl	_v_w16_min
	.p2align	2
_v_w16_min:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.8h	v0, v4, v0
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
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
	stp	q0, q1, [x0, #64]
	stp	q2, q3, [x0, #96]
	ret

	.globl	_v_w60_headroom
	.p2align	2
_v_w60_headroom:
	add	x8, x1, #32
	add	x9, x0, #32
	mov	w10, #64
LBB6_1:
	ldur	x11, [x9, #-32]
	ldur	x12, [x9, #-16]
	ldr	x13, [x9]
	ldr	x14, [x9, #16]
	ldur	x15, [x8, #-32]
	ldur	x16, [x8, #-16]
	ldr	x17, [x8]
	ldr	x0, [x8, #16]
	add	x11, x15, x11
	add	x12, x16, x12
	add	x13, x17, x13
	add	x14, x0, x14
	and	x11, x11, #0xfffffffffffffff
	and	x12, x12, #0xfffffffffffffff
	and	x13, x13, #0xfffffffffffffff
	and	x14, x14, #0xfffffffffffffff
	stp	x11, xzr, [x9, #-32]
	stp	x12, xzr, [x9, #-16]
	stp	x13, xzr, [x9]
	add	x8, x8, #64
	stp	x14, xzr, [x9, #16]
	add	x9, x9, #64
	subs	x10, x10, #4
	b.ne	LBB6_1
	ret

	.globl	_v_w60_min
	.p2align	2
_v_w60_min:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.2d	v4, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	mov	x8, #1152921504606846975
	dup.2d	v0, x8
	and.16b	v4, v4, v0
	and.16b	v1, v1, v0
	and.16b	v2, v2, v0
	and.16b	v3, v3, v0
	stp	q4, q1, [x0]
	stp	q2, q3, [x0, #32]
	ldp	q1, q2, [x0, #64]
	ldp	q3, q4, [x0, #96]
	ldp	q5, q6, [x1, #64]
	ldp	q7, q16, [x1, #96]
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	add.2d	v4, v16, v4
	and.16b	v1, v1, v0
	and.16b	v2, v2, v0
	and.16b	v3, v3, v0
	and.16b	v4, v4, v0
	stp	q1, q2, [x0, #64]
	stp	q3, q4, [x0, #96]
	ldp	q1, q2, [x0, #128]
	ldp	q3, q4, [x0, #160]
	ldp	q5, q6, [x1, #128]
	ldp	q7, q16, [x1, #160]
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	add.2d	v4, v16, v4
	and.16b	v1, v1, v0
	and.16b	v2, v2, v0
	and.16b	v3, v3, v0
	and.16b	v4, v4, v0
	stp	q1, q2, [x0, #128]
	stp	q3, q4, [x0, #160]
	ldp	q1, q2, [x0, #192]
	ldp	q3, q4, [x0, #224]
	ldp	q5, q6, [x1, #192]
	ldp	q7, q16, [x1, #224]
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	add.2d	v4, v16, v4
	and.16b	v1, v1, v0
	and.16b	v2, v2, v0
	and.16b	v3, v3, v0
	and.16b	v4, v4, v0
	stp	q1, q2, [x0, #192]
	stp	q3, q4, [x0, #224]
	ldp	q1, q2, [x0, #256]
	ldp	q3, q4, [x0, #288]
	ldp	q5, q6, [x1, #256]
	ldp	q7, q16, [x1, #288]
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	add.2d	v4, v16, v4
	and.16b	v1, v1, v0
	and.16b	v2, v2, v0
	and.16b	v3, v3, v0
	and.16b	v4, v4, v0
	stp	q1, q2, [x0, #256]
	stp	q3, q4, [x0, #288]
	ldp	q1, q2, [x0, #320]
	ldp	q3, q4, [x0, #352]
	ldp	q5, q6, [x1, #320]
	ldp	q7, q16, [x1, #352]
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	add.2d	v4, v16, v4
	and.16b	v1, v1, v0
	and.16b	v2, v2, v0
	and.16b	v3, v3, v0
	and.16b	v4, v4, v0
	stp	q1, q2, [x0, #320]
	stp	q3, q4, [x0, #352]
	ldp	q1, q2, [x0, #384]
	ldp	q3, q4, [x0, #416]
	ldp	q5, q6, [x1, #384]
	ldp	q7, q16, [x1, #416]
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	add.2d	v4, v16, v4
	and.16b	v1, v1, v0
	and.16b	v2, v2, v0
	and.16b	v3, v3, v0
	and.16b	v4, v4, v0
	stp	q1, q2, [x0, #384]
	stp	q3, q4, [x0, #416]
	ldp	q1, q2, [x0, #448]
	ldp	q3, q4, [x0, #480]
	ldp	q5, q6, [x1, #448]
	add.2d	v1, v5, v1
	ldp	q5, q7, [x1, #480]
	add.2d	v2, v6, v2
	add.2d	v3, v5, v3
	add.2d	v4, v7, v4
	and.16b	v1, v1, v0
	and.16b	v2, v2, v0
	and.16b	v3, v3, v0
	and.16b	v0, v4, v0
	stp	q1, q2, [x0, #448]
	stp	q3, q0, [x0, #480]
	ret

	.globl	_v_w64_headroom
	.p2align	2
_v_w64_headroom:
	add	x8, x1, #32
	add	x9, x0, #32
	mov	w10, #64
LBB8_1:
	ldur	x11, [x9, #-32]
	ldur	x12, [x9, #-16]
	ldr	x13, [x9]
	ldr	x14, [x9, #16]
	ldur	x15, [x8, #-32]
	ldur	x16, [x8, #-16]
	ldr	x17, [x8]
	ldr	x0, [x8, #16]
	add	x11, x15, x11
	add	x12, x16, x12
	add	x13, x17, x13
	add	x14, x0, x14
	stp	x11, xzr, [x9, #-32]
	stp	x12, xzr, [x9, #-16]
	stp	x13, xzr, [x9]
	add	x8, x8, #64
	stp	x14, xzr, [x9, #16]
	add	x9, x9, #64
	subs	x10, x10, #4
	b.ne	LBB8_1
	ret

	.globl	_v_w64_min
	.p2align	2
_v_w64_min:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	stp	q0, q1, [x0]
	stp	q2, q3, [x0, #32]
	ldp	q0, q1, [x0, #64]
	ldp	q2, q3, [x0, #96]
	ldp	q4, q5, [x1, #64]
	ldp	q6, q7, [x1, #96]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	stp	q0, q1, [x0, #64]
	stp	q2, q3, [x0, #96]
	ldp	q0, q1, [x0, #128]
	ldp	q2, q3, [x0, #160]
	ldp	q4, q5, [x1, #128]
	ldp	q6, q7, [x1, #160]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	stp	q0, q1, [x0, #128]
	stp	q2, q3, [x0, #160]
	ldp	q0, q1, [x0, #192]
	ldp	q2, q3, [x0, #224]
	ldp	q4, q5, [x1, #192]
	ldp	q6, q7, [x1, #224]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	stp	q0, q1, [x0, #192]
	stp	q2, q3, [x0, #224]
	ldp	q0, q1, [x0, #256]
	ldp	q2, q3, [x0, #288]
	ldp	q4, q5, [x1, #256]
	ldp	q6, q7, [x1, #288]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	stp	q0, q1, [x0, #256]
	stp	q2, q3, [x0, #288]
	ldp	q0, q1, [x0, #320]
	ldp	q2, q3, [x0, #352]
	ldp	q4, q5, [x1, #320]
	ldp	q6, q7, [x1, #352]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	stp	q0, q1, [x0, #320]
	stp	q2, q3, [x0, #352]
	ldp	q0, q1, [x0, #384]
	ldp	q2, q3, [x0, #416]
	ldp	q4, q5, [x1, #384]
	ldp	q6, q7, [x1, #416]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	stp	q0, q1, [x0, #384]
	stp	q2, q3, [x0, #416]
	ldp	q0, q1, [x0, #448]
	ldp	q2, q3, [x0, #480]
	ldp	q4, q5, [x1, #448]
	ldp	q6, q7, [x1, #480]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	stp	q0, q1, [x0, #448]
	stp	q2, q3, [x0, #480]
	ret

	.globl	_v_w13_three_lazy
_v_w13_three_lazy = _v_w13_three_eager
	.globl	_v_w13_reduce_lazy
_v_w13_reduce_lazy = _v_w13_reduce_eager
.subsections_via_symbols
