	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_extract_aligned_standalone
	.p2align	2
_extract_aligned_standalone:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	mov	x8, x0
	lsl	x0, x2, #1
	cmp	x0, x1
	b.hs	LBB0_3
	orr	x9, x0, #0x1
	cmp	x9, x1
	b.hs	LBB0_4
	ldrb	w0, [x8, x0]
	ldrb	w8, [x8, x9]
	bfi	w0, w8, #8, #5
	ldp	x29, x30, [sp], #16
	ret
LBB0_3:
Lloh0:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.5@PAGE
Lloh1:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.5@PAGEOFF
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB0_4:
Lloh2:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.6@PAGE
Lloh3:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.6@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh0, Lloh1
	.loh AdrpAdd	Lloh2, Lloh3

	.globl	_extract_native_standalone
	.p2align	2
_extract_native_standalone:
	cmp	x2, x1
	b.hs	LBB1_2
	ldrh	w8, [x0, x2, lsl #1]
	and	w0, w8, #0x1fff
	ret
LBB1_2:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
Lloh4:
	adrp	x8, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.7@PAGE
Lloh5:
	add	x8, x8, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.7@PAGEOFF
	mov	x0, x2
	mov	x2, x8
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh4, Lloh5

	.globl	_extract_zeropad_standalone
	.p2align	2
_extract_zeropad_standalone:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	mov	w8, #13
	mul	x12, x2, x8
	lsr	x8, x12, #3
	cmp	x8, x1
	b.hs	LBB2_5
	add	x9, x8, #1
	cmp	x9, x1
	b.hs	LBB2_6
	add	x10, x8, #2
	cmp	x10, x1
	b.hs	LBB2_7
	add	x11, x8, #3
	cmp	x11, x1
	b.hs	LBB2_8
	ldrb	w8, [x0, x8]
	ldrb	w9, [x0, x9]
	ldrb	w10, [x0, x10]
	and	w11, w12, #0x7
	orr	w8, w8, w9, lsl #8
	orr	w8, w8, w10, lsl #16
	lsr	w8, w8, w11
	and	w0, w8, #0x1fff
	ldp	x29, x30, [sp], #16
	ret
LBB2_5:
Lloh6:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.1@PAGE
Lloh7:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.1@PAGEOFF
	mov	x0, x8
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB2_6:
Lloh8:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.2@PAGE
Lloh9:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.2@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB2_7:
Lloh10:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.3@PAGE
Lloh11:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.3@PAGEOFF
	mov	x0, x10
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB2_8:
Lloh12:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.4@PAGE
Lloh13:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.4@PAGEOFF
	mov	x0, x11
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh12, Lloh13

	.globl	_sum_aligned
	.p2align	2
_sum_aligned:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	cbz	x2, LBB3_6
	mov	x9, x0
	mov	x0, #0
	mov	x8, #0
	add	x10, x1, #1
	lsr	x10, x10, #1
	lsr	x11, x1, #1
	add	x9, x9, #1
LBB3_2:
	cbz	x10, LBB3_7
	cbz	x11, LBB3_8
	ldurb	w12, [x9, #-1]
	ldrb	w13, [x9], #2
	ubfiz	x13, x13, #8, #5
	add	x8, x8, x12
	add	x8, x8, x13
	sub	x11, x11, #1
	sub	x10, x10, #1
	add	x0, x0, #2
	sub	x2, x2, #1
	cbnz	x2, LBB3_2
	mov	x0, x8
	ldp	x29, x30, [sp], #16
	ret
LBB3_6:
	mov	x8, #0
	mov	x0, x8
	ldp	x29, x30, [sp], #16
	ret
LBB3_7:
Lloh14:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.5@PAGE
Lloh15:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.5@PAGEOFF
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB3_8:
Lloh16:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.6@PAGE
Lloh17:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.6@PAGEOFF
	add	x0, x0, #1
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh16, Lloh17

	.globl	_sum_aligned_rand
	.p2align	2
_sum_aligned_rand:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	mov	x8, #0
	cbz	x2, LBB4_6
	mov	x11, #0
LBB4_2:
	cmp	x4, x11
	b.eq	LBB4_7
	ldr	w9, [x3, x11, lsl #2]
	lsl	x9, x9, #1
	cmp	x9, x1
	b.hs	LBB4_9
	orr	x10, x9, #0x1
	cmp	x10, x1
	b.hs	LBB4_8
	ldrb	w9, [x0, x9]
	ldrb	w10, [x0, x10]
	ubfiz	x10, x10, #8, #5
	add	x8, x8, x9
	add	x8, x8, x10
	add	x11, x11, #1
	cmp	x2, x11
	b.ne	LBB4_2
LBB4_6:
	mov	x0, x8
	ldp	x29, x30, [sp], #16
	ret
LBB4_7:
Lloh18:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.8@PAGE
Lloh19:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.8@PAGEOFF
	mov	x0, x4
	mov	x1, x4
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB4_8:
Lloh20:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.6@PAGE
Lloh21:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.6@PAGEOFF
	mov	x0, x10
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB4_9:
Lloh22:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.5@PAGE
Lloh23:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.5@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh22, Lloh23

	.globl	_sum_native
	.p2align	2
_sum_native:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	cbz	x2, LBB5_5
	sub	x8, x2, #1
	cmp	x1, x8
	b.ls	LBB5_6
	mov	x8, #0
LBB5_3:
	ldrh	w9, [x0], #2
	and	x9, x9, #0x1fff
	add	x8, x8, x9
	subs	x2, x2, #1
	b.ne	LBB5_3
	mov	x0, x8
	ldp	x29, x30, [sp], #16
	ret
LBB5_5:
	mov	x8, #0
	mov	x0, x8
	ldp	x29, x30, [sp], #16
	ret
LBB5_6:
Lloh24:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.7@PAGE
Lloh25:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.7@PAGEOFF
	mov	x0, x1
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh24, Lloh25

	.globl	_sum_native_rand
	.p2align	2
_sum_native_rand:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	mov	x8, #0
	cbz	x2, LBB6_5
	mov	x10, #0
LBB6_2:
	cmp	x4, x10
	b.eq	LBB6_6
	ldr	w9, [x3, x10, lsl #2]
	cmp	x1, x9
	b.ls	LBB6_7
	ldrh	w9, [x0, x9, lsl #1]
	and	x9, x9, #0x1fff
	add	x8, x8, x9
	add	x10, x10, #1
	cmp	x2, x10
	b.ne	LBB6_2
LBB6_5:
	mov	x0, x8
	ldp	x29, x30, [sp], #16
	ret
LBB6_6:
Lloh26:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.9@PAGE
Lloh27:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.9@PAGEOFF
	mov	x0, x4
	mov	x1, x4
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB6_7:
Lloh28:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.7@PAGE
Lloh29:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.7@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh28, Lloh29

	.globl	_sum_zeropad
	.p2align	2
_sum_zeropad:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	cbz	x2, LBB7_8
	mov	x13, #0
	mov	x8, #0
LBB7_2:
	lsr	x9, x13, #3
	cmp	x9, x1
	b.hs	LBB7_12
	add	x10, x9, #1
	cmp	x10, x1
	b.hs	LBB7_11
	add	x11, x9, #2
	cmp	x11, x1
	b.hs	LBB7_10
	add	x12, x9, #3
	cmp	x12, x1
	b.hs	LBB7_9
	ldrb	w9, [x0, x9]
	ldrb	w10, [x0, x10]
	ldrb	w11, [x0, x11]
	and	w12, w13, #0x7
	orr	w9, w9, w10, lsl #8
	orr	w9, w9, w11, lsl #16
	lsr	w9, w9, w12
	and	w9, w9, #0x1fff
	add	x8, x8, x9
	add	x13, x13, #13
	subs	x2, x2, #1
	b.ne	LBB7_2
	mov	x0, x8
	ldp	x29, x30, [sp], #16
	ret
LBB7_8:
	mov	x8, #0
	mov	x0, x8
	ldp	x29, x30, [sp], #16
	ret
LBB7_9:
Lloh30:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.4@PAGE
Lloh31:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.4@PAGEOFF
	mov	x0, x12
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB7_10:
Lloh32:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.3@PAGE
Lloh33:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.3@PAGEOFF
	mov	x0, x11
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB7_11:
Lloh34:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.2@PAGE
Lloh35:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.2@PAGEOFF
	mov	x0, x10
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB7_12:
Lloh36:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.1@PAGE
Lloh37:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.1@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh36, Lloh37

	.globl	_sum_zeropad_rand
	.p2align	2
_sum_zeropad_rand:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	cbz	x2, LBB8_9
	mov	x8, x0
	mov	x0, #0
	mov	x13, #0
	mov	w14, #13
LBB8_2:
	cmp	x4, x13
	b.eq	LBB8_10
	ldr	w9, [x3, x13, lsl #2]
	umull	x15, w9, w14
	lsr	x9, x15, #3
	cmp	x9, x1
	b.hs	LBB8_14
	add	x10, x9, #1
	cmp	x10, x1
	b.hs	LBB8_13
	add	x11, x9, #2
	cmp	x11, x1
	b.hs	LBB8_12
	add	x12, x9, #3
	cmp	x12, x1
	b.hs	LBB8_11
	ldrb	w9, [x8, x9]
	ldrb	w10, [x8, x10]
	ldrb	w11, [x8, x11]
	and	w12, w15, #0x7
	orr	w9, w9, w10, lsl #8
	orr	w9, w9, w11, lsl #16
	lsr	w9, w9, w12
	and	w9, w9, #0x1fff
	add	x0, x0, x9
	add	x13, x13, #1
	cmp	x2, x13
	b.ne	LBB8_2
	ldp	x29, x30, [sp], #16
	ret
LBB8_9:
	mov	x0, #0
	ldp	x29, x30, [sp], #16
	ret
LBB8_10:
Lloh38:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.10@PAGE
Lloh39:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.10@PAGEOFF
	mov	x0, x4
	mov	x1, x4
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB8_11:
Lloh40:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.4@PAGE
Lloh41:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.4@PAGEOFF
	mov	x0, x12
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB8_12:
Lloh42:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.3@PAGE
Lloh43:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.3@PAGEOFF
	mov	x0, x11
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB8_13:
Lloh44:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.2@PAGE
Lloh45:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.2@PAGEOFF
	mov	x0, x10
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB8_14:
Lloh46:
	adrp	x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.1@PAGE
Lloh47:
	add	x2, x2, l_anon.2cd64cd334ca2d8b51c02da1c2f29773.1@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh38, Lloh39
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpAdd	Lloh44, Lloh45
	.loh AdrpAdd	Lloh46, Lloh47

	.section	__TEXT,__cstring,cstring_literals
l_anon.2cd64cd334ca2d8b51c02da1c2f29773.0:
	.asciz	"codegen.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.2cd64cd334ca2d8b51c02da1c2f29773.1:
	.quad	l_anon.2cd64cd334ca2d8b51c02da1c2f29773.0
	.asciz	"\n\000\000\000\000\000\000\000'\000\000\000\t\000\000"

	.p2align	3, 0x0
l_anon.2cd64cd334ca2d8b51c02da1c2f29773.2:
	.quad	l_anon.2cd64cd334ca2d8b51c02da1c2f29773.0
	.asciz	"\n\000\000\000\000\000\000\000(\000\000\000\t\000\000"

	.p2align	3, 0x0
l_anon.2cd64cd334ca2d8b51c02da1c2f29773.3:
	.quad	l_anon.2cd64cd334ca2d8b51c02da1c2f29773.0
	.asciz	"\n\000\000\000\000\000\000\000)\000\000\000\t\000\000"

	.p2align	3, 0x0
l_anon.2cd64cd334ca2d8b51c02da1c2f29773.4:
	.quad	l_anon.2cd64cd334ca2d8b51c02da1c2f29773.0
	.asciz	"\n\000\000\000\000\000\000\000*\000\000\000\t\000\000"

	.p2align	3, 0x0
l_anon.2cd64cd334ca2d8b51c02da1c2f29773.5:
	.quad	l_anon.2cd64cd334ca2d8b51c02da1c2f29773.0
	.asciz	"\n\000\000\000\000\000\000\000\031\000\000\000#\000\000"

	.p2align	3, 0x0
l_anon.2cd64cd334ca2d8b51c02da1c2f29773.6:
	.quad	l_anon.2cd64cd334ca2d8b51c02da1c2f29773.0
	.asciz	"\n\000\000\000\000\000\000\000\031\000\000\000-\000\000"

	.p2align	3, 0x0
l_anon.2cd64cd334ca2d8b51c02da1c2f29773.7:
	.quad	l_anon.2cd64cd334ca2d8b51c02da1c2f29773.0
	.asciz	"\n\000\000\000\000\000\000\000\202\000\000\000\005\000\000"

	.p2align	3, 0x0
l_anon.2cd64cd334ca2d8b51c02da1c2f29773.8:
	.quad	l_anon.2cd64cd334ca2d8b51c02da1c2f29773.0
	.asciz	"\n\000\000\000\000\000\000\000c\000\000\0001\000\000"

	.p2align	3, 0x0
l_anon.2cd64cd334ca2d8b51c02da1c2f29773.9:
	.quad	l_anon.2cd64cd334ca2d8b51c02da1c2f29773.0
	.asciz	"\n\000\000\000\000\000\000\000\235\000\000\0000\000\000"

	.p2align	3, 0x0
l_anon.2cd64cd334ca2d8b51c02da1c2f29773.10:
	.quad	l_anon.2cd64cd334ca2d8b51c02da1c2f29773.0
	.asciz	"\n\000\000\000\000\000\000\000r\000\000\0001\000\000"

.subsections_via_symbols
