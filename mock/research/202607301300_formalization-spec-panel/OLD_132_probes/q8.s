	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_s_bytes
	.p2align	2
_s_bytes:
	.cfi_startproc
	add	w0, w1, w0
	ret
	.cfi_endproc

	.globl	_v_bytes
	.p2align	2
_v_bytes:
	.cfi_startproc
	add	x8, x0, #32
	add	x9, x1, #32
	mov	w10, #1024
LBB1_1:
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
	b.ne	LBB1_1
	ret
	.cfi_endproc

	.globl	_v_native
	.p2align	2
_v_native:
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

	.globl	_s_native
_s_native = _s_bytes
.subsections_via_symbols
