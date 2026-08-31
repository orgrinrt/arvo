	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_sat_left
	.p2align	2
_sat_left:
	.cfi_startproc
	and	w8, w0, #0xff
	add	w8, w8, w1, uxtb
	mov	w9, #255
	cmp	w8, #255
	csel	w8, w8, w9, lo
	add	w8, w8, w2, uxtb
	cmp	w8, #255
	csel	w8, w8, w9, lo
	add	w8, w8, w3, uxtb
	cmp	w8, #255
	csel	w0, w8, w9, lo
	ret
	.cfi_endproc

	.globl	_sat_reduce
	.p2align	2
_sat_reduce:
	.cfi_startproc
	mov	w8, #0
	mov	x9, #0
	mov	w10, #255
LBB1_1:
	ldrb	w11, [x0, x9]
	add	w8, w11, w8, uxtb
	cmp	w8, #255
	csel	w8, w8, w10, lo
	add	x9, x9, #1
	cmp	x9, #256
	b.ne	LBB1_1
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_sat_reduce_tree
	.p2align	2
_sat_reduce_tree:
	.cfi_startproc
	ldp	q0, q1, [x0]
	uqadd.16b	v0, v0, v1
	ldp	q1, q2, [x0, #32]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	ldp	q1, q2, [x0, #64]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	ldp	q1, q2, [x0, #96]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	ldp	q1, q2, [x0, #128]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	ldp	q1, q2, [x0, #160]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	ldp	q1, q2, [x0, #192]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	ldp	q1, q2, [x0, #224]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	umov.b	w8, v0[0]
	umov.b	w9, v0[1]
	add	w9, w8, w9, uxtb
	mov	w8, #255
	cmp	w9, #255
	csel	w9, w9, w8, lo
	umov.b	w10, v0[2]
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w9, w9, w8, lo
	umov.b	w10, v0[3]
	add	w9, w9, w10, uxtb
	umov.b	w10, v0[4]
	cmp	w9, #255
	csel	w9, w9, w8, lo
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w9, w9, w8, lo
	umov.b	w10, v0[5]
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w9, w9, w8, lo
	umov.b	w10, v0[6]
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w9, w9, w8, lo
	umov.b	w10, v0[7]
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w9, w9, w8, lo
	umov.b	w10, v0[8]
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w9, w9, w8, lo
	umov.b	w10, v0[9]
	add	w9, w9, w10, uxtb
	umov.b	w10, v0[10]
	cmp	w9, #255
	csel	w9, w9, w8, lo
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w9, w9, w8, lo
	umov.b	w10, v0[11]
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w9, w9, w8, lo
	umov.b	w10, v0[12]
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w9, w9, w8, lo
	umov.b	w10, v0[13]
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w9, w9, w8, lo
	umov.b	w10, v0[14]
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w9, w9, w8, lo
	umov.b	w10, v0[15]
	add	w9, w9, w10, uxtb
	cmp	w9, #255
	csel	w0, w9, w8, lo
	ret
	.cfi_endproc

	.globl	_sat_right
	.p2align	2
_sat_right:
	.cfi_startproc
	and	w8, w2, #0xff
	add	w8, w8, w3, uxtb
	mov	w9, #255
	cmp	w8, #255
	csel	w8, w8, w9, lo
	add	w8, w8, w1, uxtb
	cmp	w8, #255
	csel	w8, w8, w9, lo
	add	w8, w8, w0, uxtb
	cmp	w8, #255
	csel	w0, w8, w9, lo
	ret
	.cfi_endproc

	.globl	_wrap_left
	.p2align	2
_wrap_left:
	.cfi_startproc
	add	w8, w1, w0
	add	w9, w2, w3
	add	w0, w8, w9
	ret
	.cfi_endproc

	.globl	_wrap_reduce
	.p2align	2
_wrap_reduce:
	.cfi_startproc
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	ldp	q4, q5, [x0, #64]
	ldp	q6, q7, [x0, #96]
	add.16b	v0, v4, v0
	add.16b	v1, v5, v1
	add.16b	v2, v6, v2
	add.16b	v3, v7, v3
	ldp	q4, q5, [x0, #128]
	ldp	q6, q7, [x0, #160]
	ldp	q16, q17, [x0, #192]
	ldp	q18, q19, [x0, #224]
	add.16b	v4, v16, v4
	add.16b	v0, v4, v0
	add.16b	v4, v17, v5
	add.16b	v1, v4, v1
	add.16b	v4, v18, v6
	add.16b	v2, v4, v2
	add.16b	v4, v19, v7
	add.16b	v3, v4, v3
	add.16b	v0, v1, v0
	add.16b	v0, v2, v0
	add.16b	v0, v3, v0
	addv.16b	b0, v0
	fmov	w0, s0
	ret
	.cfi_endproc

	.globl	_wrap_right
_wrap_right = _wrap_left
.subsections_via_symbols
