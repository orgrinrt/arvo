	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_d_b2
	.p2align	2
_d_b2:
	.cfi_startproc
	add	w0, w1, w0
	ret
	.cfi_endproc

	.globl	_d_b8
	.p2align	2
_d_b8:
	.cfi_startproc
	add	x0, x1, x0
	ret
	.cfi_endproc

	.globl	_v_d_b2
	.p2align	2
_v_d_b2:
	.cfi_startproc
	add	x8, x0, #32
	add	x9, x1, #32
	mov	w10, #1024
LBB2_1:
	ldp	q0, q1, [x8, #-32]
	ldp	q2, q3, [x8]
	ldp	q4, q5, [x9, #-32]
	ldp	q6, q7, [x9], #64
	add.8h	v0, v4, v0
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
	stp	q0, q1, [x8, #-32]
	stp	q2, q3, [x8], #64
	subs	x10, x10, #32
	b.ne	LBB2_1
	ret
	.cfi_endproc

.subsections_via_symbols
