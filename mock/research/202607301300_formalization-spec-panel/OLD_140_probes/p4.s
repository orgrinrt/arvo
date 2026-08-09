	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_v_w1
	.p2align	2
_v_w1:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.16b	v0, v4, v0
	add.16b	v1, v5, v1
	add.16b	v2, v6, v2
	add.16b	v3, v7, v3
	movi.16b	v4, #1
	and.16b	v0, v0, v4
	and.16b	v1, v1, v4
	and.16b	v2, v2, v4
	and.16b	v3, v3, v4
	stp	q0, q1, [x0]
	stp	q2, q3, [x0, #32]
	ret

	.globl	_v_w127
	.p2align	2
_v_w127:
	stp	x20, x19, [sp, #-16]!
	add	x8, x1, #32
	add	x9, x0, #32
	mov	w10, #64
LBB2_1:
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
	and	x11, x11, #0x7fffffffffffffff
	and	x13, x13, #0x7fffffffffffffff
	and	x15, x15, #0x7fffffffffffffff
	and	x17, x17, #0x7fffffffffffffff
	stp	x12, x11, [x9, #-32]
	stp	x14, x13, [x9, #-16]
	stp	x16, x15, [x9]
	add	x8, x8, #64
	stp	x0, x17, [x9, #16]
	add	x9, x9, #64
	subs	x10, x10, #4
	b.ne	LBB2_1
	ldp	x20, x19, [sp], #16
	ret

	.globl	_v_w128
	.p2align	2
_v_w128:
	stp	x20, x19, [sp, #-16]!
	add	x8, x1, #32
	add	x9, x0, #32
	mov	w10, #64
LBB3_1:
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
	stp	x12, x11, [x9, #-32]
	stp	x14, x13, [x9, #-16]
	stp	x16, x15, [x9]
	add	x8, x8, #64
	stp	x0, x17, [x9, #16]
	add	x9, x9, #64
	subs	x10, x10, #4
	b.ne	LBB3_1
	ldp	x20, x19, [sp], #16
	ret

	.globl	_v_w15
	.p2align	2
_v_w15:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.8h	v0, v4, v0
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
	bic.8h	v0, #128, lsl #8
	bic.8h	v1, #128, lsl #8
	bic.8h	v2, #128, lsl #8
	bic.8h	v3, #128, lsl #8
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
	bic.8h	v0, #128, lsl #8
	bic.8h	v1, #128, lsl #8
	bic.8h	v2, #128, lsl #8
	bic.8h	v3, #128, lsl #8
	stp	q0, q1, [x0, #64]
	stp	q2, q3, [x0, #96]
	ret

	.globl	_v_w16
	.p2align	2
_v_w16:
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

	.globl	_v_w17
	.p2align	2
_v_w17:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.4s	v4, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	movi.4s	v0, #1, msl #16
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

	.globl	_v_w31
	.p2align	2
_v_w31:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.4s	v0, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	bic.4s	v0, #128, lsl #24
	bic.4s	v1, #128, lsl #24
	bic.4s	v2, #128, lsl #24
	bic.4s	v3, #128, lsl #24
	stp	q0, q1, [x0]
	stp	q2, q3, [x0, #32]
	ldp	q0, q1, [x0, #64]
	ldp	q2, q3, [x0, #96]
	ldp	q4, q5, [x1, #64]
	ldp	q6, q7, [x1, #96]
	add.4s	v0, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	bic.4s	v0, #128, lsl #24
	bic.4s	v1, #128, lsl #24
	bic.4s	v2, #128, lsl #24
	bic.4s	v3, #128, lsl #24
	stp	q0, q1, [x0, #64]
	stp	q2, q3, [x0, #96]
	ldp	q0, q1, [x0, #128]
	ldp	q2, q3, [x0, #160]
	ldp	q4, q5, [x1, #128]
	ldp	q6, q7, [x1, #160]
	add.4s	v0, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	bic.4s	v0, #128, lsl #24
	bic.4s	v1, #128, lsl #24
	bic.4s	v2, #128, lsl #24
	bic.4s	v3, #128, lsl #24
	stp	q0, q1, [x0, #128]
	stp	q2, q3, [x0, #160]
	ldp	q0, q1, [x0, #192]
	ldp	q2, q3, [x0, #224]
	ldp	q4, q5, [x1, #192]
	add.4s	v0, v4, v0
	ldp	q4, q6, [x1, #224]
	add.4s	v1, v5, v1
	add.4s	v2, v4, v2
	add.4s	v3, v6, v3
	bic.4s	v0, #128, lsl #24
	bic.4s	v1, #128, lsl #24
	bic.4s	v2, #128, lsl #24
	bic.4s	v3, #128, lsl #24
	stp	q0, q1, [x0, #192]
	stp	q2, q3, [x0, #224]
	ret

	.globl	_v_w32
	.p2align	2
_v_w32:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.4s	v0, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	stp	q0, q1, [x0]
	stp	q2, q3, [x0, #32]
	ldp	q0, q1, [x0, #64]
	ldp	q2, q3, [x0, #96]
	ldp	q4, q5, [x1, #64]
	ldp	q6, q7, [x1, #96]
	add.4s	v0, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	stp	q0, q1, [x0, #64]
	stp	q2, q3, [x0, #96]
	ldp	q0, q1, [x0, #128]
	ldp	q2, q3, [x0, #160]
	ldp	q4, q5, [x1, #128]
	ldp	q6, q7, [x1, #160]
	add.4s	v0, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	stp	q0, q1, [x0, #128]
	stp	q2, q3, [x0, #160]
	ldp	q0, q1, [x0, #192]
	ldp	q2, q3, [x0, #224]
	ldp	q4, q5, [x1, #192]
	ldp	q6, q7, [x1, #224]
	add.4s	v0, v4, v0
	add.4s	v1, v5, v1
	add.4s	v2, v6, v2
	add.4s	v3, v7, v3
	stp	q0, q1, [x0, #192]
	stp	q2, q3, [x0, #224]
	ret

	.globl	_v_w33
	.p2align	2
_v_w33:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.2d	v4, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	mov	x8, #8589934591
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

	.globl	_v_w63
	.p2align	2
_v_w63:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.2d	v4, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	movi.2d	v0, #0xffffffffffffffff
	fneg.2d	v0, v0
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

	.globl	_v_w64
	.p2align	2
_v_w64:
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

	.globl	_v_w65
	.p2align	2
_v_w65:
	stp	x20, x19, [sp, #-16]!
	add	x8, x1, #32
	add	x9, x0, #32
	mov	w10, #64
LBB12_1:
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
	and	x11, x11, #0x1
	and	x13, x13, #0x1
	and	x15, x15, #0x1
	and	x17, x17, #0x1
	stp	x12, x11, [x9, #-32]
	stp	x14, x13, [x9, #-16]
	stp	x16, x15, [x9]
	add	x8, x8, #64
	stp	x0, x17, [x9, #16]
	add	x9, x9, #64
	subs	x10, x10, #4
	b.ne	LBB12_1
	ldp	x20, x19, [sp], #16
	ret

	.globl	_v_w7
	.p2align	2
_v_w7:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.16b	v0, v4, v0
	add.16b	v1, v5, v1
	add.16b	v2, v6, v2
	add.16b	v3, v7, v3
	movi.16b	v4, #127
	and.16b	v0, v0, v4
	and.16b	v1, v1, v4
	and.16b	v2, v2, v4
	and.16b	v3, v3, v4
	stp	q0, q1, [x0]
	stp	q2, q3, [x0, #32]
	ret

	.globl	_v_w8
	.p2align	2
_v_w8:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.16b	v0, v4, v0
	add.16b	v1, v5, v1
	add.16b	v2, v6, v2
	add.16b	v3, v7, v3
	stp	q0, q1, [x0]
	stp	q2, q3, [x0, #32]
	ret

	.globl	_v_w9
	.p2align	2
_v_w9:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	add.8h	v0, v4, v0
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
	bic.8h	v0, #254, lsl #8
	bic.8h	v1, #254, lsl #8
	bic.8h	v2, #254, lsl #8
	bic.8h	v3, #254, lsl #8
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
	bic.8h	v0, #254, lsl #8
	bic.8h	v1, #254, lsl #8
	bic.8h	v2, #254, lsl #8
	bic.8h	v3, #254, lsl #8
	stp	q0, q1, [x0, #64]
	stp	q2, q3, [x0, #96]
	ret

.subsections_via_symbols
