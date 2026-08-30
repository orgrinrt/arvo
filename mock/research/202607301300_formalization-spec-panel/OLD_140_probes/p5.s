	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_hi_add
	.p2align	2
_hi_add:
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

	.globl	_hi_cmp
	.p2align	2
_hi_cmp:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	cmhi.8h	v0, v4, v0
	movi.8h	v4, #1
	and.16b	v0, v0, v4
	cmhi.8h	v1, v5, v1
	and.16b	v1, v1, v4
	cmhi.8h	v2, v6, v2
	and.16b	v2, v2, v4
	cmhi.8h	v3, v7, v3
	and.16b	v3, v3, v4
	stp	q0, q1, [x2]
	stp	q2, q3, [x2, #32]
	ldp	q0, q1, [x0, #64]
	ldp	q2, q3, [x0, #96]
	ldp	q5, q6, [x1, #64]
	ldp	q7, q16, [x1, #96]
	cmhi.8h	v0, v5, v0
	and.16b	v0, v0, v4
	cmhi.8h	v1, v6, v1
	and.16b	v1, v1, v4
	cmhi.8h	v2, v7, v2
	and.16b	v2, v2, v4
	cmhi.8h	v3, v16, v3
	and.16b	v3, v3, v4
	stp	q0, q1, [x2, #64]
	stp	q2, q3, [x2, #96]
	ret

	.globl	_hi_mul
	.p2align	2
_hi_mul:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	ushr.8h	v4, v4, #3
	ushr.8h	v5, v5, #3
	ushr.8h	v6, v6, #3
	ushr.8h	v7, v7, #3
	mul.8h	v0, v4, v0
	mul.8h	v1, v5, v1
	mul.8h	v2, v6, v2
	mul.8h	v3, v7, v3
	stp	q0, q1, [x0]
	stp	q2, q3, [x0, #32]
	ldp	q0, q1, [x0, #64]
	ldp	q2, q3, [x0, #96]
	ldp	q4, q5, [x1, #64]
	ldp	q6, q7, [x1, #96]
	ushr.8h	v4, v4, #3
	ushr.8h	v5, v5, #3
	ushr.8h	v6, v6, #3
	ushr.8h	v7, v7, #3
	mul.8h	v0, v4, v0
	mul.8h	v1, v5, v1
	mul.8h	v2, v6, v2
	mul.8h	v3, v7, v3
	stp	q0, q1, [x0, #64]
	stp	q2, q3, [x0, #96]
	ret

	.globl	_hi_read
	.p2align	2
_hi_read:
	ldp	q0, q1, [x0]
	ushr.8h	v0, v0, #3
	ushr.8h	v1, v1, #3
	stp	q0, q1, [x1]
	ldp	q0, q1, [x0, #32]
	ushr.8h	v0, v0, #3
	ushr.8h	v1, v1, #3
	stp	q0, q1, [x1, #32]
	ldp	q0, q1, [x0, #64]
	ushr.8h	v0, v0, #3
	ushr.8h	v1, v1, #3
	stp	q0, q1, [x1, #64]
	ldp	q0, q1, [x0, #96]
	ushr.8h	v0, v0, #3
	ushr.8h	v1, v1, #3
	stp	q0, q1, [x1, #96]
	ret

	.globl	_lo_add
	.p2align	2
_lo_add:
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

	.globl	_lo_mul
	.p2align	2
_lo_mul:
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x1]
	ldp	q6, q7, [x1, #32]
	mul.8h	v0, v4, v0
	mul.8h	v1, v5, v1
	mul.8h	v2, v6, v2
	mul.8h	v3, v7, v3
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
	mul.8h	v0, v4, v0
	mul.8h	v1, v5, v1
	mul.8h	v2, v6, v2
	mul.8h	v3, v7, v3
	bic.8h	v0, #224, lsl #8
	bic.8h	v1, #224, lsl #8
	bic.8h	v2, #224, lsl #8
	bic.8h	v3, #224, lsl #8
	stp	q0, q1, [x0, #64]
	stp	q2, q3, [x0, #96]
	ret

	.globl	_lo_cmp
_lo_cmp = _hi_cmp
.subsections_via_symbols
