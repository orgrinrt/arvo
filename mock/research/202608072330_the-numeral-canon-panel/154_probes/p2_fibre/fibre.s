	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	__RNvXs1_Csa9RnP3DN9kJ_5fibreNtB5_11Packed13ColNtB5_13NumeralColumn7get_u64
	.p2align	2
__RNvXs1_Csa9RnP3DN9kJ_5fibreNtB5_11Packed13ColNtB5_13NumeralColumn7get_u64:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	mov	x8, x1
	ldr	x1, [x0, #8]
	mov	w9, #13
	mul	x12, x8, x9
	lsr	x8, x12, #3
	cmp	x8, x1
	b.hs	LBB1_5
	add	x9, x8, #1
	cmp	x9, x1
	b.hs	LBB1_6
	add	x10, x8, #2
	cmp	x10, x1
	b.hs	LBB1_7
	add	x11, x8, #3
	cmp	x11, x1
	b.hs	LBB1_8
	ldr	x11, [x0]
	ldrb	w8, [x11, x8]
	ldrb	w9, [x11, x9]
	ldrb	w10, [x11, x10]
	orr	w8, w8, w9, lsl #8
	orr	w8, w8, w10, lsl #16
	and	w9, w12, #0x7
	lsr	w8, w8, w9
	and	w0, w8, #0x1fff
	ldp	x29, x30, [sp], #16
	ret
LBB1_5:
Lloh0:
	adrp	x2, l_anon.de73de75c6c70a9b91fa41f46b9925b3.1@PAGE
Lloh1:
	add	x2, x2, l_anon.de73de75c6c70a9b91fa41f46b9925b3.1@PAGEOFF
	mov	x0, x8
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB1_6:
Lloh2:
	adrp	x2, l_anon.de73de75c6c70a9b91fa41f46b9925b3.2@PAGE
Lloh3:
	add	x2, x2, l_anon.de73de75c6c70a9b91fa41f46b9925b3.2@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB1_7:
Lloh4:
	adrp	x2, l_anon.de73de75c6c70a9b91fa41f46b9925b3.3@PAGE
Lloh5:
	add	x2, x2, l_anon.de73de75c6c70a9b91fa41f46b9925b3.3@PAGEOFF
	mov	x0, x10
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB1_8:
Lloh6:
	adrp	x2, l_anon.de73de75c6c70a9b91fa41f46b9925b3.4@PAGE
Lloh7:
	add	x2, x2, l_anon.de73de75c6c70a9b91fa41f46b9925b3.4@PAGEOFF
	mov	x0, x11
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh0, Lloh1
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh6, Lloh7

	.globl	_both_instances_via_column
	.p2align	2
_both_instances_via_column:
	sub	sp, sp, #80
	stp	x24, x23, [sp, #16]
	stp	x22, x21, [sp, #32]
	stp	x20, x19, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	mov	x20, x3
	mov	x19, x2
	mov	x22, #0
	stp	x0, x1, [sp]
	subs	x8, x1, #4
	csel	x8, xzr, x8, lo
	lsl	x8, x8, #3
	cmp	x8, #13
	b.lo	LBB2_3
	mov	x22, #0
	mov	x21, #0
	mov	x9, #20165
	movk	x9, #50412, lsl #16
	movk	x9, #60494, lsl #32
	movk	x9, #20164, lsl #48
	umulh	x8, x8, x9
	lsr	x23, x8, #2
LBB2_2:
	mov	x0, sp
	mov	x1, x21
	bl	__RNvXs1_Csa9RnP3DN9kJ_5fibreNtB5_11Packed13ColNtB5_13NumeralColumn7get_u64
	add	x22, x0, x22
	add	x21, x21, #1
	cmp	x23, x21
	b.ne	LBB2_2
LBB2_3:
	cbz	x20, LBB2_6
	cmp	x20, #8
	b.hs	LBB2_7
	mov	x10, #0
	mov	x8, #0
	b	LBB2_16
LBB2_6:
	mov	x10, #0
	b	LBB2_18
LBB2_7:
	cmp	x20, #32
	b.hs	LBB2_9
	mov	x8, #0
	mov	x10, #0
	b	LBB2_13
LBB2_9:
	and	x9, x20, #0x18
	and	x8, x20, #0x3fffffffffffffe0
	add	x10, x19, #32
	movi.2d	v0, #0000000000000000
	and	x11, x20, #0x3fffffffffffffe0
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB2_10:
	ldp	q4, q5, [x10, #-32]
	ldp	q6, q7, [x10], #64
	ushll2.4s	v16, v4, #0
	ushll.4s	v4, v4, #0
	uaddw.2d	v0, v0, v4
	uaddw2.2d	v0, v0, v4
	uaddw.2d	v0, v0, v16
	uaddw2.2d	v0, v0, v16
	ushll2.4s	v4, v5, #0
	ushll.4s	v5, v5, #0
	uaddw.2d	v1, v1, v5
	uaddw2.2d	v1, v1, v5
	uaddw.2d	v1, v1, v4
	uaddw2.2d	v1, v1, v4
	ushll2.4s	v4, v6, #0
	ushll.4s	v5, v6, #0
	uaddw.2d	v2, v2, v5
	uaddw2.2d	v2, v2, v5
	uaddw.2d	v2, v2, v4
	uaddw2.2d	v2, v2, v4
	ushll2.4s	v4, v7, #0
	ushll.4s	v5, v7, #0
	uaddw.2d	v3, v3, v5
	uaddw2.2d	v3, v3, v5
	uaddw.2d	v3, v3, v4
	uaddw2.2d	v3, v3, v4
	subs	x11, x11, #32
	b.ne	LBB2_10
	add.2d	v0, v1, v0
	add.2d	v1, v3, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	fmov	x10, d0
	cmp	x20, x8
	b.eq	LBB2_18
	cbz	x9, LBB2_16
LBB2_13:
	mov	x11, x8
	and	x8, x20, #0x3ffffffffffffff8
	movi.2d	v0, #0000000000000000
	mov.d	v0[0], x10
	sub	x9, x11, x8
	add	x10, x19, x11, lsl #1
LBB2_14:
	ldr	q1, [x10], #16
	ushll2.4s	v2, v1, #0
	ushll.4s	v1, v1, #0
	uaddw.2d	v0, v0, v1
	uaddw2.2d	v0, v0, v1
	uaddw.2d	v0, v0, v2
	uaddw2.2d	v0, v0, v2
	adds	x9, x9, #8
	b.ne	LBB2_14
	addp.2d	d0, v0
	fmov	x10, d0
	cmp	x20, x8
	b.eq	LBB2_18
LBB2_16:
	sub	x9, x20, x8
	add	x8, x19, x8, lsl #1
LBB2_17:
	ldrh	w11, [x8], #2
	add	x10, x10, x11
	subs	x9, x9, #1
	b.ne	LBB2_17
LBB2_18:
	add	x0, x10, x22
	ldp	x29, x30, [sp, #64]
	ldp	x20, x19, [sp, #48]
	ldp	x22, x21, [sp, #32]
	ldp	x24, x23, [sp, #16]
	add	sp, sp, #80
	ret

	.globl	_control_dense_works
	.p2align	2
_control_dense_works:
	mov	w0, #8205
	ret

	.globl	_dense_size
	.p2align	2
_dense_size:
	mov	w0, #2
	ret

	.section	__TEXT,__cstring,cstring_literals
l_anon.de73de75c6c70a9b91fa41f46b9925b3.0:
	.asciz	"p2_fibre/fibre.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.de73de75c6c70a9b91fa41f46b9925b3.1:
	.quad	l_anon.de73de75c6c70a9b91fa41f46b9925b3.0
	.asciz	"\021\000\000\000\000\000\000\000;\000\000\000\r\000\000"

	.p2align	3, 0x0
l_anon.de73de75c6c70a9b91fa41f46b9925b3.2:
	.quad	l_anon.de73de75c6c70a9b91fa41f46b9925b3.0
	.asciz	"\021\000\000\000\000\000\000\000<\000\000\000\r\000\000"

	.p2align	3, 0x0
l_anon.de73de75c6c70a9b91fa41f46b9925b3.3:
	.quad	l_anon.de73de75c6c70a9b91fa41f46b9925b3.0
	.asciz	"\021\000\000\000\000\000\000\000=\000\000\000\r\000\000"

	.p2align	3, 0x0
l_anon.de73de75c6c70a9b91fa41f46b9925b3.4:
	.quad	l_anon.de73de75c6c70a9b91fa41f46b9925b3.0
	.asciz	"\021\000\000\000\000\000\000\000>\000\000\000\r\000\000"

.subsections_via_symbols
