	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
	.cfi_startproc
LBB0_1:
	b	LBB0_1
	.cfi_endproc

	.globl	_w13_headroom
	.p2align	2
_w13_headroom:
	.cfi_startproc
	add	w8, w1, w0
	and	w0, w8, #0x1fff
	ret
	.cfi_endproc

	.globl	_w13_native
	.p2align	2
_w13_native:
	.cfi_startproc
	add	w8, w1, w0
	and	w0, w8, #0x1fff
	ret
	.cfi_endproc

	.globl	_w64_headroom
	.p2align	2
_w64_headroom:
	.cfi_startproc
	add	x0, x2, x0
	mov	x1, #0
	ret
	.cfi_endproc

	.globl	_w64_headroom_vec
	.p2align	2
_w64_headroom_vec:
	.cfi_startproc
	add	x8, x1, #32
	add	x9, x0, #32
	mov	w10, #64
LBB4_1:
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
	b.ne	LBB4_1
	ret
	.cfi_endproc

	.globl	_w64_native
	.p2align	2
_w64_native:
	.cfi_startproc
	add	x0, x1, x0
	ret
	.cfi_endproc

	.globl	_w64_native_vec
	.p2align	2
_w64_native_vec:
	.cfi_startproc
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
	.cfi_endproc

.subsections_via_symbols
