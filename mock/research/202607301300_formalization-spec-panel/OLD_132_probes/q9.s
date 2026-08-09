	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_limb_b2
	.p2align	2
_limb_b2:
	.cfi_startproc
	and	w8, w0, #0xff00
	and	w9, w1, #0xff
	ubfx	w10, w1, #8, #8
	add	w9, w9, w0, uxtb
	and	w11, w9, #0xff
	cmp	w11, w9
	add	w8, w10, w8, lsr #8
	cinc	w8, w8, ne
	orr	w0, w11, w8, lsl #8
	ret
	.cfi_endproc

	.globl	_limb_b8
	.p2align	2
_limb_b8:
	.cfi_startproc
	lsr	w9, w0, #8
	lsr	w10, w0, #16
	lsr	x11, x0, #32
	lsr	x12, x0, #40
	lsr	x13, x0, #48
	and	w8, w1, #0xff
	ubfx	w14, w1, #8, #8
	ubfx	w15, w1, #16, #8
	lsr	w16, w1, #24
	ubfx	x17, x1, #32, #8
	ubfx	x2, x1, #40, #8
	ubfx	x3, x1, #48, #8
	lsr	x1, x1, #56
	add	w8, w8, w0, uxtb
	add	w9, w14, w9, uxtb
	and	w14, w9, #0xff
	cmp	w14, w9
	add	w9, w14, w8, lsr #8
	lsr	w14, w9, #8
	csinc	w14, w14, wzr, eq
	add	w10, w15, w10, uxtb
	and	w15, w10, #0xff
	cmp	w15, w10
	cset	w10, ne
	add	w14, w15, w14
	orr	w10, w10, w14, lsr #8
	add	w15, w16, w0, lsr #24
	and	w16, w15, #0xff
	cmp	w16, w15
	cset	w15, ne
	add	w10, w16, w10
	orr	w15, w15, w10, lsr #8
	add	w11, w17, w11, uxtb
	and	w16, w11, #0xff
	cmp	w16, w11
	cset	w11, ne
	add	w15, w16, w15
	orr	w11, w11, w15, lsr #8
	add	w12, w2, w12, uxtb
	and	w16, w12, #0xff
	cmp	w16, w12
	cset	w12, ne
	add	w11, w16, w11
	orr	w12, w12, w11, lsr #8
	add	w13, w3, w13, uxtb
	and	w16, w13, #0xff
	cmp	w16, w13
	cset	w13, ne
	add	w12, w16, w12
	orr	w13, w13, w12, lsr #8
	add	x16, x1, x0, lsr #56
	add	w13, w16, w13
	and	w12, w12, #0xff
	lsl	x12, x12, #48
	orr	x12, x12, x13, lsl #56
	and	w11, w11, #0xff
	orr	x11, x12, x11, lsl #40
	and	w12, w15, #0xff
	orr	x11, x11, x12, lsl #32
	lsl	w10, w10, #24
	orr	x10, x11, x10
	bfi	x10, x14, #16, #8
	and	x8, x8, #0xff
	bfi	x10, x9, #8, #8
	orr	x0, x10, x8
	ret
	.cfi_endproc

	.globl	_nat_16
	.p2align	2
_nat_16:
	.cfi_startproc
	add	w0, w1, w0
	ret
	.cfi_endproc

	.globl	_nat_64
	.p2align	2
_nat_64:
	.cfi_startproc
	add	x0, x1, x0
	ret
	.cfi_endproc

	.globl	_v_limb_b2
	.p2align	2
_v_limb_b2:
	.cfi_startproc
	add	x8, x0, #32
	add	x9, x1, #32
	mov	w10, #1024
LBB4_1:
	ldp	q0, q1, [x8, #-32]
	ldp	q2, q3, [x8]
	ldp	q4, q5, [x9, #-32]
	xtn.8b	v6, v0
	xtn.8b	v7, v1
	ldp	q16, q17, [x9], #64
	xtn.8b	v18, v2
	xtn.8b	v19, v3
	shrn.8b	v0, v0, #8
	shrn.8b	v1, v1, #8
	shrn.8b	v2, v2, #8
	shrn.8b	v3, v3, #8
	xtn.8b	v20, v4
	xtn.8b	v21, v5
	xtn.8b	v22, v16
	xtn.8b	v23, v17
	shrn.8b	v4, v4, #8
	shrn.8b	v5, v5, #8
	shrn.8b	v16, v16, #8
	shrn.8b	v17, v17, #8
	add.8b	v20, v20, v6
	add.8b	v21, v21, v7
	add.8b	v22, v22, v18
	add.8b	v23, v23, v19
	cmhi.8b	v6, v6, v20
	cmhi.8b	v7, v7, v21
	cmhi.8b	v18, v18, v22
	cmhi.8b	v19, v19, v23
	add.8b	v0, v4, v0
	add.8b	v1, v5, v1
	add.8b	v2, v16, v2
	add.8b	v3, v17, v3
	sub.8b	v0, v0, v6
	sub.8b	v1, v1, v7
	sub.8b	v2, v2, v18
	sub.8b	v3, v3, v19
	shll.8h	v0, v0, #8
	shll.8h	v1, v1, #8
	shll.8h	v2, v2, #8
	shll.8h	v3, v3, #8
	ushll.8h	v4, v20, #0
	ushll.8h	v5, v21, #0
	ushll.8h	v6, v22, #0
	ushll.8h	v7, v23, #0
	orr.16b	v0, v0, v4
	orr.16b	v1, v1, v5
	orr.16b	v2, v2, v6
	stp	q0, q1, [x8, #-32]
	orr.16b	v0, v3, v7
	stp	q2, q0, [x8], #64
	subs	x10, x10, #32
	b.ne	LBB4_1
	ret
	.cfi_endproc

	.globl	_v_nat_16
	.p2align	2
_v_nat_16:
	.cfi_startproc
	add	x8, x0, #32
	add	x9, x1, #32
	mov	w10, #1024
LBB5_1:
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
	b.ne	LBB5_1
	ret
	.cfi_endproc

.subsections_via_symbols
