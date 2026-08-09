	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_t2_handwritten_sum
	.p2align	2
_t2_handwritten_sum:
	.cfi_startproc
	stp	x20, x19, [sp, #-16]!
	.cfi_def_cfa_offset 16
	.cfi_offset w19, -8
	.cfi_offset w20, -16
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #39
	mov	w13, #996
	mov	w14, #5
LBB0_1:
	sub	x15, x12, #39
	sub	x16, x12, #26
	sub	x17, x12, #13
	and	x1, x15, #0x4
	and	x2, x16, x14
	and	x3, x17, #0x6
	add	x15, x0, x15, lsr #3
	add	x16, x0, x16, lsr #3
	add	x17, x0, x17, lsr #3
	add	x4, x0, x12, lsr #3
	ldrh	w5, [x15]
	ldrh	w6, [x16]
	ldrh	w7, [x17]
	ldrh	w19, [x4]
	ldrb	w15, [x15, #2]
	ldrb	w16, [x16, #2]
	ldrb	w17, [x17, #2]
	orr	x15, x5, x15, lsl #16
	ldrb	w4, [x4, #2]
	orr	x16, x6, x16, lsl #16
	orr	x17, x7, x17, lsl #16
	orr	x4, x19, x4, lsl #16
	and	x5, x12, #0x7
	lsr	x15, x15, x1
	lsr	x16, x16, x2
	lsr	x17, x17, x3
	lsr	x1, x4, x5
	and	x15, x15, #0x1fff
	and	x16, x16, #0x1fff
	and	x17, x17, #0x1fff
	and	x1, x1, #0x1fff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #52
	subs	x13, x13, #4
	b.ne	LBB0_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x8, x9, x8
	ldrh	w9, [x0, #1618]
	ldrb	w10, [x0, #1620]
	orr	w9, w9, w10, lsl #16
	ldrb	w11, [x0, #1621]
	orr	w10, w10, w11, lsl #8
	ldrb	w12, [x0, #1622]
	orr	w11, w11, w12, lsl #8
	ubfx	x9, x9, #4, #13
	ubfx	x10, x10, #1, #13
	ldrb	w12, [x0, #1623]
	orr	w11, w11, w12, lsl #16
	ubfx	x11, x11, #6, #13
	add	x9, x10, x9
	add	x9, x11, x9
	add	x0, x9, x8
	ldp	x20, x19, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w19
	.cfi_restore w20
	ret
	.cfi_endproc

	.globl	_t2_scalar_native
	.p2align	2
_t2_scalar_native:
	.cfi_startproc
	madd	w8, w1, w0, w2
	and	w0, w8, #0xffff
	ret
	.cfi_endproc

	.globl	_t2_typed_sum
	.p2align	2
_t2_typed_sum:
	.cfi_startproc
	stp	x20, x19, [sp, #-16]!
	.cfi_def_cfa_offset 16
	.cfi_offset w19, -8
	.cfi_offset w20, -16
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #39
	mov	w13, #996
	mov	w14, #5
LBB2_1:
	sub	x15, x12, #39
	sub	x16, x12, #26
	sub	x17, x12, #13
	add	x1, x0, x15, lsr #3
	add	x2, x0, x16, lsr #3
	add	x3, x0, x17, lsr #3
	add	x4, x0, x12, lsr #3
	ldrh	w5, [x1]
	ldrh	w6, [x2]
	ldrh	w7, [x3]
	ldrh	w19, [x4]
	ldrb	w1, [x1, #2]
	ldrb	w2, [x2, #2]
	ldrb	w3, [x3, #2]
	orr	x1, x5, x1, lsl #16
	orr	x2, x6, x2, lsl #16
	orr	x3, x7, x3, lsl #16
	ldrb	w4, [x4, #2]
	orr	x4, x19, x4, lsl #16
	and	x15, x15, #0x4
	and	x16, x16, x14
	and	x17, x17, #0x6
	and	x5, x12, #0x7
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x1fff
	and	x16, x16, #0x1fff
	and	x17, x17, #0x1fff
	and	x1, x1, #0x1fff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #52
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x8, x9, x8
	ldrh	w9, [x0, #1618]
	ldrb	w10, [x0, #1620]
	orr	w9, w9, w10, lsl #16
	ldrb	w11, [x0, #1621]
	orr	w10, w10, w11, lsl #8
	ldrb	w12, [x0, #1622]
	orr	w11, w11, w12, lsl #8
	ubfx	x9, x9, #4, #13
	ubfx	x10, x10, #1, #13
	ldrb	w12, [x0, #1623]
	orr	w11, w11, w12, lsl #16
	ubfx	x11, x11, #6, #13
	add	x9, x10, x9
	add	x9, x11, x9
	add	x0, x9, x8
	ldp	x20, x19, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w19
	.cfi_restore w20
	ret
	.cfi_endproc

	.globl	_t2_typed_sum_aligned_access3
	.p2align	2
_t2_typed_sum_aligned_access3:
	.cfi_startproc
	stp	d9, d8, [sp, #-16]!
	.cfi_def_cfa_offset 16
	.cfi_offset b8, -8
	.cfi_offset b9, -16
	mov	x8, #0
	movi.2d	v0, #0000000000000000
	mov	w9, #7936
	dup.2d	v2, x9
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
	movi.2d	v1, #0000000000000000
	movi.2d	v7, #0000000000000000
	movi.2d	v5, #0000000000000000
	movi.2d	v16, #0000000000000000
	movi.2d	v6, #0000000000000000
LBB3_1:
	add	x9, x0, x8
	ld2.16b	{ v19, v20 }, [x9]
	ushll2.8h	v21, v19, #0
	ushll2.4s	v18, v21, #0
	ushll.2d	v17, v18, #0
	ushll2.2d	v18, v18, #0
	ushll.4s	v21, v21, #0
	ushll.2d	v22, v21, #0
	ushll.8h	v23, v19, #0
	ushll2.4s	v24, v23, #0
	ushll.2d	v25, v24, #0
	ushll2.2d	v21, v21, #0
	ushll2.2d	v24, v24, #0
	ushll.4s	v23, v23, #0
	ushll.2d	v26, v23, #0
	ushll2.2d	v23, v23, #0
	ushll2.8h	v27, v20, #0
	ushll2.4s	v28, v27, #0
	ext.16b	v29, v28, v28, #8
	ushll.4s	v27, v27, #0
	ushll.8h	v19, v20, #0
	ushll2.4s	v20, v19, #0
	ext.16b	v30, v27, v27, #8
	ext.16b	v31, v20, v20, #8
	ushll.4s	v19, v19, #0
	ext.16b	v8, v19, v19, #8
	shl.2s	v8, v8, #8
	ushll.2d	v8, v8, #0
	shl.2s	v19, v19, #8
	ushll.2d	v19, v19, #0
	shl.2s	v31, v31, #8
	ushll.2d	v31, v31, #0
	shl.2s	v30, v30, #8
	ushll.2d	v30, v30, #0
	shl.2s	v20, v20, #8
	ushll.2d	v20, v20, #0
	shl.2s	v27, v27, #8
	ushll.2d	v27, v27, #0
	shl.2s	v29, v29, #8
	ushll.2d	v29, v29, #0
	shl.2s	v28, v28, #8
	ushll.2d	v28, v28, #0
	and.16b	v28, v28, v2
	and.16b	v29, v29, v2
	and.16b	v27, v27, v2
	and.16b	v20, v20, v2
	and.16b	v30, v30, v2
	and.16b	v31, v31, v2
	and.16b	v19, v19, v2
	and.16b	v8, v8, v2
	orr.16b	v23, v8, v23
	orr.16b	v19, v19, v26
	orr.16b	v24, v31, v24
	orr.16b	v21, v30, v21
	orr.16b	v20, v20, v25
	orr.16b	v22, v27, v22
	orr.16b	v18, v29, v18
	orr.16b	v17, v28, v17
	add.2d	v16, v17, v16
	add.2d	v6, v18, v6
	add.2d	v7, v22, v7
	add.2d	v4, v20, v4
	add.2d	v5, v21, v5
	add.2d	v1, v24, v1
	add.2d	v3, v19, v3
	add.2d	v0, v23, v0
	add	x8, x8, #32
	cmp	x8, #800
	b.ne	LBB3_1
	add.2d	v2, v3, v7
	add.2d	v3, v4, v16
	add.2d	v2, v2, v3
	add.2d	v0, v0, v5
	add.2d	v1, v1, v6
	add.2d	v0, v0, v1
	add.2d	v0, v2, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ldp	d9, d8, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore b8
	.cfi_restore b9
	ret
	.cfi_endproc

	.globl	_t2_typed_sum_cold_400
	.p2align	2
_t2_typed_sum_cold_400:
	.cfi_startproc
	stp	x20, x19, [sp, #-16]!
	.cfi_def_cfa_offset 16
	.cfi_offset w19, -8
	.cfi_offset w20, -16
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #39
	mov	w13, #400
	mov	w14, #5
LBB4_1:
	sub	x15, x12, #39
	sub	x16, x12, #26
	sub	x17, x12, #13
	add	x1, x0, x15, lsr #3
	add	x2, x0, x16, lsr #3
	add	x3, x0, x17, lsr #3
	add	x4, x0, x12, lsr #3
	ldrh	w5, [x1]
	ldrh	w6, [x2]
	ldrh	w7, [x3]
	ldrh	w19, [x4]
	ldrb	w1, [x1, #2]
	ldrb	w2, [x2, #2]
	ldrb	w3, [x3, #2]
	orr	x1, x5, x1, lsl #16
	orr	x2, x6, x2, lsl #16
	orr	x3, x7, x3, lsl #16
	ldrb	w4, [x4, #2]
	orr	x4, x19, x4, lsl #16
	and	x15, x15, #0x4
	and	x16, x16, x14
	and	x17, x17, #0x6
	and	x5, x12, #0x7
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x1fff
	and	x16, x16, #0x1fff
	and	x17, x17, #0x1fff
	and	x1, x1, #0x1fff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #52
	subs	x13, x13, #4
	b.ne	LBB4_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ldp	x20, x19, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w19
	.cfi_restore w20
	ret
	.cfi_endproc

	.globl	_t2_typed_sum_warm
_t2_typed_sum_warm = _t2_typed_sum_aligned_access3
	.globl	_t2_scalar_typed
_t2_scalar_typed = _t2_scalar_native
.subsections_via_symbols
