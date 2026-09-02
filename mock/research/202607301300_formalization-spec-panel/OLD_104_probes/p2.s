	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECshdop7HPDjU1_14p2_composition
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECshdop7HPDjU1_14p2_composition
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECshdop7HPDjU1_14p2_composition:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x4, x3
	mov	x3, x2
	mov	x2, x1
	str	x0, [sp, #8]
Lloh0:
	adrp	x1, l_anon.5134cccca99e7a045e7be4444c8c72b6.2@PAGE
Lloh1:
	add	x1, x1, l_anon.5134cccca99e7a045e7be4444c8c72b6.2@PAGEOFF
	add	x0, sp, #8
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh0, Lloh1

	.p2align	2
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECshdop7HPDjU1_14p2_composition:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	blr	x0
	; InlineAsm Start
	; InlineAsm End
	ldp	x29, x30, [sp], #16
	ret

	.p2align	2
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cshdop7HPDjU1_14p2_composition:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECshdop7HPDjU1_14p2_composition
	mov	w0, #0
	ldp	x29, x30, [sp], #16
	ret

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCshdop7HPDjU1_14p2_composition:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECshdop7HPDjU1_14p2_composition
	mov	w0, #0
	ldp	x29, x30, [sp], #16
	ret

	.p2align	2
__RNvCshdop7HPDjU1_14p2_composition12sum_one_step:
	.cfi_startproc
	mov	x2, x1
	mov	x8, x0
	mov	x0, #0
	mov	w10, #3
	mov	w11, #53251
LBB4_1:
	lsr	x9, x10, #3
	add	x1, x9, #8
	cmp	x1, x2
	b.hi	LBB4_4
	ldr	x9, [x8, x9]
	and	x12, x10, #0x7
	lsr	x9, x9, x12
	and	x9, x9, #0x1f
	add	x0, x9, x0
	add	x10, x10, #13
	cmp	x10, x11
	b.ne	LBB4_1
	ret
LBB4_4:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh2:
	adrp	x3, l_anon.5134cccca99e7a045e7be4444c8c72b6.4@PAGE
Lloh3:
	add	x3, x3, l_anon.5134cccca99e7a045e7be4444c8c72b6.4@PAGEOFF
	mov	x0, x9
	bl	__RNvNtNtCs5dyeT9KiOLK_4core5slice5index16slice_index_fail
	.loh AdrpAdd	Lloh2, Lloh3
	.cfi_endproc

	.p2align	2
__RNvCshdop7HPDjU1_14p2_composition12sum_two_step:
	.cfi_startproc
	mov	x2, x1
	mov	x10, #0
	mov	x8, #0
LBB5_1:
	lsr	x9, x10, #3
	add	x1, x9, #8
	cmp	x1, x2
	b.hi	LBB5_4
	ldr	x9, [x0, x9]
	and	x11, x10, #0x7
	lsr	x9, x9, x11
	ubfx	x9, x9, #3, #5
	add	x8, x9, x8
	add	x10, x10, #13
	cmp	x10, #13, lsl #12
	b.ne	LBB5_1
	mov	x0, x8
	ret
LBB5_4:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh4:
	adrp	x3, l_anon.5134cccca99e7a045e7be4444c8c72b6.5@PAGE
Lloh5:
	add	x3, x3, l_anon.5134cccca99e7a045e7be4444c8c72b6.5@PAGEOFF
	mov	x0, x9
	bl	__RNvNtNtCs5dyeT9KiOLK_4core5slice5index16slice_index_fail
	.loh AdrpAdd	Lloh4, Lloh5
	.cfi_endproc

	.p2align	2
__RNvCshdop7HPDjU1_14p2_composition13read_one_step:
	.cfi_startproc
	mov	x9, x1
Lloh6:
	adrp	x8, l_anon.5134cccca99e7a045e7be4444c8c72b6.6@PAGE
Lloh7:
	add	x8, x8, l_anon.5134cccca99e7a045e7be4444c8c72b6.6@PAGEOFF
	add	x10, x8, x3, lsl #4
	ldr	x8, [x10]
	mov	w11, #13
	madd	x11, x2, x11, x8
	lsr	x8, x11, #3
	add	x1, x8, #8
	cmp	x1, x9
	b.hi	LBB6_2
	ldr	x8, [x0, x8]
	ldr	x9, [x10, #8]
	and	x10, x11, #0x7
	lsr	x8, x8, x10
	mov	x10, #-1
	lsl	x9, x10, x9
	bic	w0, w8, w9
	ret
LBB6_2:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh8:
	adrp	x3, l_anon.5134cccca99e7a045e7be4444c8c72b6.7@PAGE
Lloh9:
	add	x3, x3, l_anon.5134cccca99e7a045e7be4444c8c72b6.7@PAGEOFF
	mov	x0, x8
	mov	x2, x9
	bl	__RNvNtNtCs5dyeT9KiOLK_4core5slice5index16slice_index_fail
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh8, Lloh9
	.cfi_endproc

	.p2align	2
__RNvCshdop7HPDjU1_14p2_composition13read_two_step:
	.cfi_startproc
	mov	x9, x1
	mov	x8, x0
	mov	w10, #13
	mul	x10, x2, x10
	lsr	x0, x10, #3
	add	x1, x0, #8
	cmp	x1, x9
	b.hi	LBB7_2
	ldr	x8, [x8, x0]
	and	x9, x10, #0x7
	lsr	x8, x8, x9
	and	x8, x8, #0x1fff
Lloh10:
	adrp	x9, l_anon.5134cccca99e7a045e7be4444c8c72b6.6@PAGE
Lloh11:
	add	x9, x9, l_anon.5134cccca99e7a045e7be4444c8c72b6.6@PAGEOFF
	add	x9, x9, x3, lsl #4
	ldp	x10, x9, [x9]
	lsr	x8, x8, x10
	mov	x10, #-1
	lsl	x9, x10, x9
	bic	w0, w8, w9
	ret
LBB7_2:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh12:
	adrp	x3, l_anon.5134cccca99e7a045e7be4444c8c72b6.8@PAGE
Lloh13:
	add	x3, x3, l_anon.5134cccca99e7a045e7be4444c8c72b6.8@PAGEOFF
	mov	x2, x9
	bl	__RNvNtNtCs5dyeT9KiOLK_4core5slice5index16slice_index_fail
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh12, Lloh13
	.cfi_endproc

	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI8_0:
	.long	0
	.long	1
	.long	2
	.long	3
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__RNvCshdop7HPDjU1_14p2_composition4main
	.globl	__RNvCshdop7HPDjU1_14p2_composition4main
	.p2align	2
__RNvCshdop7HPDjU1_14p2_composition4main:
	.cfi_startproc
	stp	x26, x25, [sp, #-80]!
	stp	x24, x23, [sp, #16]
	stp	x22, x21, [sp, #32]
	stp	x20, x19, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	sub	x9, sp, #12, lsl #12
LBB8_1:
	sub	sp, sp, #1, lsl #12
	str	xzr, [sp]
	cmp	sp, x9
	b.ne	LBB8_1
	sub	sp, sp, #160
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	mov	x8, #0
	sub	x22, x29, #216
	add	x9, sp, #8
LBB8_3:
	add	x10, x9, x8
	str	wzr, [x10, #8]
	str	xzr, [x10]
	add	x8, x8, #12
	cmp	x8, #12, lsl #12
	b.ne	LBB8_3
Lloh14:
	adrp	x8, lCPI8_0@PAGE
Lloh15:
	ldr	q0, [x8, lCPI8_0@PAGEOFF]
	add	x8, sp, #8
	add	x9, x8, #96
	mov	w8, #4096
	movi.4s	v1, #4
	movi.4s	v2, #8
	movi.4s	v3, #12
	movi.4s	v4, #31
	movi.4s	v5, #7
	movi.4s	v6, #16
LBB8_5:
	ushr.4s	v18, v0, #8
	ushr.4s	v7, v0, #3
	sub	x10, x9, #96
	and.16b	v17, v7, v4
	and.16b	v16, v0, v5
	st3.4s	{ v16, v17, v18 }, [x10]
	add.4s	v7, v0, v1
	ushr.4s	v21, v7, #8
	ushr.4s	v22, v7, #3
	sub	x10, x9, #48
	and.16b	v19, v7, v5
	and.16b	v20, v22, v4
	st3.4s	{ v19, v20, v21 }, [x10]
	add.4s	v7, v0, v2
	add.4s	v19, v0, v3
	ushr.4s	v20, v7, #8
	ushr.4s	v23, v19, #8
	ushr.4s	v7, v7, #3
	and.16b	v17, v7, v4
	mov.16b	v18, v20
	add	x10, x9, #192
	ushr.4s	v7, v19, #3
	and.16b	v21, v19, v5
	and.16b	v22, v7, v4
	st3.4s	{ v16, v17, v18 }, [x9], #48
	st3.4s	{ v21, v22, v23 }, [x9]
	mov	x9, x10
	add.4s	v0, v0, v6
	subs	x8, x8, #16
	b.ne	LBB8_5
	bl	__RNvCske4UNIzLImn_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w0, #6664
	mov	w1, #1
	bl	__RNvCske4UNIzLImn_7___rustc19___rust_alloc_zeroed
	cbz	x0, LBB8_50
	mov	x19, x0
	mov	x8, #0
	add	x9, sp, #8
	add	x9, x9, #8
	mov	w10, #49152
LBB8_8:
	lsr	x11, x8, #3
	ldp	w13, w12, [x9, #-8]
	and	w13, w13, #0x7
	bfi	w13, w12, #3, #5
	ldr	w12, [x9], #12
	bfi	w13, w12, #8, #5
	and	x12, x8, #0x7
	ldr	x14, [x19, x11]
	lsl	x12, x13, x12
	orr	x12, x12, x14
	str	x12, [x19, x11]
	add	x8, x8, #13
	subs	x10, x10, #12
	b.ne	LBB8_8
	mov	x20, #0
	mov	x24, #0
	mov	x25, #0
	add	x8, sp, #8
	add	x23, x8, #8
LBB8_10:
	mov	x0, x19
	mov	w1, #6664
	mov	x2, x20
	mov	x3, #0
	bl	__RNvCshdop7HPDjU1_14p2_composition13read_two_step
	mov	x21, x0
	mov	x0, x19
	mov	w1, #6664
	mov	x2, x20
	mov	x3, #0
	bl	__RNvCshdop7HPDjU1_14p2_composition13read_one_step
	ldur	w8, [x23, #-8]
	cmp	w21, w8
	cinc	x24, x24, ne
	cmp	w21, w0
	cinc	x25, x25, ne
	mov	x0, x19
	mov	w1, #6664
	mov	x2, x20
	mov	w3, #1
	bl	__RNvCshdop7HPDjU1_14p2_composition13read_two_step
	mov	x21, x0
	mov	x0, x19
	mov	w1, #6664
	mov	x2, x20
	mov	w3, #1
	bl	__RNvCshdop7HPDjU1_14p2_composition13read_one_step
	ldur	w8, [x23, #-4]
	cmp	w21, w8
	cinc	x24, x24, ne
	cmp	w21, w0
	cinc	x25, x25, ne
	mov	x0, x19
	mov	w1, #6664
	mov	x2, x20
	mov	w3, #2
	bl	__RNvCshdop7HPDjU1_14p2_composition13read_two_step
	mov	x21, x0
	mov	x0, x19
	mov	w1, #6664
	mov	x2, x20
	mov	w3, #2
	bl	__RNvCshdop7HPDjU1_14p2_composition13read_one_step
	ldr	w8, [x23], #12
	cmp	w21, w8
	cinc	x24, x24, ne
	cmp	w21, w0
	cinc	x25, x25, ne
	add	x20, x20, #1
	cmp	x20, #1, lsl #12
	b.ne	LBB8_10
	stp	x24, x25, [x22]
Lloh16:
	adrp	x8, l_anon.5134cccca99e7a045e7be4444c8c72b6.9@PAGE
Lloh17:
	add	x8, x8, l_anon.5134cccca99e7a045e7be4444c8c72b6.9@PAGEOFF
Lloh18:
	adrp	x23, __RNvXsi_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impjNtB9_7Display3fmt@GOTPAGE
Lloh19:
	ldr	x23, [x23, __RNvXsi_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impjNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x8, x23, [x22, #72]
Lloh20:
	adrp	x8, l_anon.5134cccca99e7a045e7be4444c8c72b6.10@PAGE
Lloh21:
	add	x8, x8, l_anon.5134cccca99e7a045e7be4444c8c72b6.10@PAGEOFF
	stp	x8, x23, [x22, #88]
Lloh22:
	adrp	x8, l_anon.5134cccca99e7a045e7be4444c8c72b6.6@PAGE
Lloh23:
	add	x8, x8, l_anon.5134cccca99e7a045e7be4444c8c72b6.6@PAGEOFF
Lloh24:
	adrp	x9, __RNvXsa_NtCs5dyeT9KiOLK_4core5arrayATjjEj3_NtNtB7_3fmt5Debug3fmtCshdop7HPDjU1_14p2_composition@PAGE
Lloh25:
	add	x9, x9, __RNvXsa_NtCs5dyeT9KiOLK_4core5arrayATjjEj3_NtNtB7_3fmt5Debug3fmtCshdop7HPDjU1_14p2_composition@PAGEOFF
	stp	x8, x9, [x22, #104]
Lloh26:
	adrp	x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.11@PAGE
Lloh27:
	add	x0, x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.11@PAGEOFF
	sub	x1, x29, #144
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	stp	x22, x23, [x22, #72]
Lloh28:
	adrp	x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.12@PAGE
Lloh29:
	add	x0, x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.12@PAGEOFF
	sub	x1, x29, #144
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	sub	x8, x29, #208
	stp	x8, x23, [x22, #72]
Lloh30:
	adrp	x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.13@PAGE
Lloh31:
	add	x0, x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.13@PAGEOFF
	sub	x1, x29, #144
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #6664
	bl	__RNvCshdop7HPDjU1_14p2_composition12sum_two_step
	str	x0, [x22, #40]
	mov	x0, x19
	mov	w1, #6664
	bl	__RNvCshdop7HPDjU1_14p2_composition12sum_one_step
	str	x0, [x22, #48]
Lloh32:
	adrp	x8, __RNvXsd_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impyNtB9_7Display3fmt@GOTPAGE
Lloh33:
	ldr	x8, [x8, __RNvXsd_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impyNtB9_7Display3fmt@GOTPAGEOFF]
	sub	x9, x29, #176
	stp	x9, x8, [x22, #72]
	sub	x9, x29, #168
	stp	x9, x8, [x22, #88]
Lloh34:
	adrp	x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.14@PAGE
Lloh35:
	add	x0, x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.14@PAGEOFF
	sub	x1, x29, #144
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w8, #13
	mov	w10, #8
LBB8_12:
	mov	x9, x10
	udiv	w10, w8, w9
	msub	w10, w10, w9, w8
	mov	x8, x9
	cbnz	w10, LBB8_12
	mov	w8, #8
	udiv	w8, w8, w9
	str	x8, [x22, #16]
	mov	w9, #13
	umull	x8, w8, w9
	lsr	x8, x8, #3
	str	x8, [x22, #48]
	sub	x8, x29, #200
	stp	x8, x23, [x22, #72]
	sub	x8, x29, #168
	stp	x8, x23, [x22, #88]
Lloh36:
	adrp	x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.15@PAGE
Lloh37:
	add	x0, x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.15@PAGEOFF
	sub	x1, x29, #144
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x20, #0
	stp	xzr, xzr, [x22, #24]
	mov	w8, #3
	str	x8, [x22, #40]
	ldr	x24, [x22, #16]
	lsl	x21, x24, #3
	lsr	x8, x24, #61
	cbnz	x8, LBB8_43
	mov	x8, #9223372036854775800
	cmp	x21, x8
	b.hi	LBB8_43
	cbz	x21, LBB8_18
	bl	__RNvCske4UNIzLImn_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w20, #8
	mov	x0, x21
	mov	w1, #8
	bl	__RNvCske4UNIzLImn_7___rustc12___rust_alloc
	cbz	x0, LBB8_43
	mov	x8, x24
	cbnz	x24, LBB8_19
	b	LBB8_26
LBB8_18:
	mov	x8, #0
	mov	w0, #8
	cbz	x24, LBB8_26
LBB8_19:
	cmp	x24, #4
	b.hs	LBB8_21
	mov	x9, #0
	b	LBB8_24
LBB8_21:
	and	x9, x24, #0x1ffffffffffffffc
	add	x10, x0, #16
	mov	w11, #15
	mov	w12, #5
	and	x13, x24, #0x1ffffffffffffffc
LBB8_22:
	sub	w14, w11, #15
	sub	w15, w11, #10
	sub	w16, w11, #5
	and	x14, x14, #0x4
	and	x15, x15, x12
	and	x16, x16, #0x6
	stp	x14, x15, [x10, #-16]
	and	x14, x11, #0x7
	stp	x16, x14, [x10], #32
	add	x11, x11, #20
	subs	x13, x13, #4
	b.ne	LBB8_22
	cmp	x24, x9
	b.eq	LBB8_26
LBB8_24:
	add	x10, x9, x9, lsl #2
	sub	x11, x24, x9
	add	x9, x0, x9, lsl #3
LBB8_25:
	and	x12, x10, #0x7
	str	x12, [x9], #8
	add	x10, x10, #5
	subs	x11, x11, #1
	b.ne	LBB8_25
LBB8_26:
	stp	x8, x0, [x22, #48]
	sub	x8, x29, #192
	stp	x24, x8, [x22, #64]
	sub	x8, x29, #184
	stp	x23, x8, [x22, #80]
	sub	x8, x29, #176
	stp	x23, x8, [x22, #96]
	stp	x23, x8, [x22, #112]
	sub	x8, x29, #168
	stp	x23, x8, [x22, #128]
Lloh38:
	adrp	x24, __RNvXsq_NtCseduYQEDYcHM_5alloc3vecINtB5_3VecjENtNtCs5dyeT9KiOLK_4core3fmt5Debug3fmtCshdop7HPDjU1_14p2_composition@PAGE
Lloh39:
	add	x24, x24, __RNvXsq_NtCseduYQEDYcHM_5alloc3vecINtB5_3VecjENtNtCs5dyeT9KiOLK_4core3fmt5Debug3fmtCshdop7HPDjU1_14p2_composition@PAGEOFF
	str	x24, [x22, #144]
Lloh40:
	adrp	x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.16@PAGE
Lloh41:
	add	x0, x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.16@PAGEOFF
	sub	x1, x29, #144
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	ldr	x8, [x22, #48]
	cbz	x8, LBB8_28
	ldr	x0, [x22, #56]
	lsl	x1, x8, #3
	mov	w2, #8
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
LBB8_28:
	mov	x20, #0
	mov	w8, #1
	mov	w9, #3
	stp	x8, x9, [x22, #24]
	mov	w8, #5
	str	x8, [x22, #40]
	ldr	x25, [x22, #16]
	lsl	x21, x25, #3
	lsr	x8, x25, #61
	cbnz	x8, LBB8_43
	mov	x8, #9223372036854775800
	cmp	x21, x8
	b.hi	LBB8_43
	cbz	x21, LBB8_33
	bl	__RNvCske4UNIzLImn_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w20, #8
	mov	x0, x21
	mov	w1, #8
	bl	__RNvCske4UNIzLImn_7___rustc12___rust_alloc
	cbz	x0, LBB8_43
	mov	x8, x25
	cbnz	x25, LBB8_34
	b	LBB8_36
LBB8_33:
	mov	x8, #0
	mov	w0, #8
	cbz	x25, LBB8_36
LBB8_34:
	mov	x9, #0
	mov	x10, x25
	mov	x11, x0
LBB8_35:
	ldr	w12, [x22, #32]
	add	w12, w9, w12
	and	x12, x12, #0x7
	str	x12, [x11], #8
	add	x9, x9, #5
	subs	x10, x10, #1
	b.ne	LBB8_35
LBB8_36:
	stp	x8, x0, [x22, #48]
	sub	x8, x29, #192
	stp	x25, x8, [x22, #64]
	sub	x8, x29, #184
	stp	x23, x8, [x22, #80]
	sub	x8, x29, #176
	stp	x23, x8, [x22, #96]
	stp	x23, x8, [x22, #112]
	sub	x8, x29, #168
	stp	x23, x8, [x22, #128]
	str	x24, [x22, #144]
Lloh42:
	adrp	x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.16@PAGE
Lloh43:
	add	x0, x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.16@PAGEOFF
	sub	x1, x29, #144
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	ldr	x8, [x22, #48]
	cbz	x8, LBB8_38
	ldr	x0, [x22, #56]
	lsl	x1, x8, #3
	mov	w2, #8
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
LBB8_38:
	mov	x20, #0
	mov	w8, #2
	mov	w9, #8
	stp	x8, x9, [x22, #24]
	mov	w8, #5
	str	x8, [x22, #40]
	ldr	x25, [x22, #16]
	lsl	x21, x25, #3
	lsr	x8, x25, #61
	cbnz	x8, LBB8_43
	mov	x8, #9223372036854775800
	cmp	x21, x8
	b.hi	LBB8_43
	cbz	x21, LBB8_44
	bl	__RNvCske4UNIzLImn_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w20, #8
	mov	x0, x21
	mov	w1, #8
	bl	__RNvCske4UNIzLImn_7___rustc12___rust_alloc
	cbz	x0, LBB8_43
	mov	x8, x25
	cbnz	x25, LBB8_45
	b	LBB8_47
LBB8_43:
	mov	x0, x20
	mov	x1, x21
	bl	__RNvNtCseduYQEDYcHM_5alloc7raw_vec12handle_error
LBB8_44:
	mov	x8, #0
	mov	w0, #8
	cbz	x25, LBB8_47
LBB8_45:
	mov	x9, #0
	mov	x10, x25
	mov	x11, x0
LBB8_46:
	ldr	w12, [x22, #32]
	add	w12, w9, w12
	and	x12, x12, #0x7
	str	x12, [x11], #8
	add	x9, x9, #5
	subs	x10, x10, #1
	b.ne	LBB8_46
LBB8_47:
	stp	x8, x0, [x22, #48]
	sub	x8, x29, #192
	stp	x25, x8, [x22, #64]
	sub	x8, x29, #184
	stp	x23, x8, [x22, #80]
	sub	x8, x29, #176
	stp	x23, x8, [x22, #96]
	stp	x23, x8, [x22, #112]
	sub	x8, x29, #168
	stp	x23, x8, [x22, #128]
	str	x24, [x22, #144]
Lloh44:
	adrp	x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.16@PAGE
Lloh45:
	add	x0, x0, l_anon.5134cccca99e7a045e7be4444c8c72b6.16@PAGEOFF
	sub	x1, x29, #144
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	ldr	x8, [x22, #48]
	cbz	x8, LBB8_49
	ldr	x0, [x22, #56]
	lsl	x1, x8, #3
	mov	w2, #8
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
LBB8_49:
	mov	x0, x19
	mov	w1, #6664
	mov	w2, #1
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
	add	sp, sp, #12, lsl #12
	add	sp, sp, #160
	ldp	x29, x30, [sp, #64]
	ldp	x20, x19, [sp, #48]
	ldp	x22, x21, [sp, #32]
	ldp	x24, x23, [sp, #16]
	ldp	x26, x25, [sp], #80
	ret
LBB8_50:
	mov	w0, #1
	mov	w1, #6664
	bl	__RNvNtCseduYQEDYcHM_5alloc7raw_vec12handle_error
	.loh AdrpLdr	Lloh14, Lloh15
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpLdrGot	Lloh32, Lloh33
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpLdrGot	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh36, Lloh37
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpAdd	Lloh38, Lloh39
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpAdd	Lloh44, Lloh45
	.cfi_endproc

	.p2align	2
__RNvXs1g_NtCs5dyeT9KiOLK_4core3fmtRTjjENtB6_5Debug3fmtCshdop7HPDjU1_14p2_composition:
	sub	sp, sp, #80
	stp	x20, x19, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	ldr	x19, [x0]
	add	x8, sp, #8
	mov	x0, x1
	mov	w1, #1
	mov	x2, #0
	bl	__RNvMsa_NtCs5dyeT9KiOLK_4core3fmtNtB5_9Formatter11debug_tuple
	str	x19, [sp, #32]
	add	x8, x19, #8
	stur	x8, [x29, #-24]
Lloh46:
	adrp	x19, l_anon.5134cccca99e7a045e7be4444c8c72b6.1@PAGE
Lloh47:
	add	x19, x19, l_anon.5134cccca99e7a045e7be4444c8c72b6.1@PAGEOFF
	add	x0, sp, #8
	add	x1, sp, #32
	mov	x2, x19
	bl	__RNvMs2_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_10DebugTuple5field
	add	x0, sp, #8
	sub	x1, x29, #24
	mov	x2, x19
	bl	__RNvMs2_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_10DebugTuple5field
	add	x0, sp, #8
	bl	__RNvMs2_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_10DebugTuple6finish
	ldp	x29, x30, [sp, #64]
	ldp	x20, x19, [sp, #48]
	add	sp, sp, #80
	ret
	.loh AdrpAdd	Lloh46, Lloh47

	.p2align	2
__RNvXs1g_NtCs5dyeT9KiOLK_4core3fmtRjNtB6_5Debug3fmtCshdop7HPDjU1_14p2_composition:
	ldr	x0, [x0]
	ldr	w8, [x1, #16]
	tbnz	w8, #25, LBB10_3
	tbnz	w8, #26, LBB10_4
	b	__RNvXsi_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impjNtB9_7Display3fmt
LBB10_3:
	b	__RNvXs6_NtNtCs5dyeT9KiOLK_4core3fmt3numjNtB7_8LowerHex3fmt
LBB10_4:
	b	__RNvXs8_NtNtCs5dyeT9KiOLK_4core3fmt3numjNtB7_8UpperHex3fmt

	.p2align	2
__RNvXsa_NtCs5dyeT9KiOLK_4core5arrayATjjEj3_NtNtB7_3fmt5Debug3fmtCshdop7HPDjU1_14p2_composition:
	.cfi_startproc
	sub	sp, sp, #64
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	mov	x19, x0
	add	x8, sp, #8
	mov	x0, x1
	bl	__RNvMsa_NtCs5dyeT9KiOLK_4core3fmtNtB5_9Formatter10debug_list
	str	x19, [sp, #24]
Lloh48:
	adrp	x20, l_anon.5134cccca99e7a045e7be4444c8c72b6.0@PAGE
Lloh49:
	add	x20, x20, l_anon.5134cccca99e7a045e7be4444c8c72b6.0@PAGEOFF
	add	x0, sp, #8
	add	x1, sp, #24
	mov	x2, x20
	bl	__RNvMs5_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_9DebugList5entry
	add	x8, x19, #16
	str	x8, [sp, #24]
	add	x0, sp, #8
	add	x1, sp, #24
	mov	x2, x20
	bl	__RNvMs5_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_9DebugList5entry
	add	x8, x19, #32
	str	x8, [sp, #24]
	add	x0, sp, #8
	add	x1, sp, #24
	mov	x2, x20
	bl	__RNvMs5_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_9DebugList5entry
	add	x0, sp, #8
	bl	__RNvMs5_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_9DebugList6finish
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	add	sp, sp, #64
	ret
	.loh AdrpAdd	Lloh48, Lloh49
	.cfi_endproc

	.p2align	2
__RNvXsq_NtCseduYQEDYcHM_5alloc3vecINtB5_3VecjENtNtCs5dyeT9KiOLK_4core3fmt5Debug3fmtCshdop7HPDjU1_14p2_composition:
	.cfi_startproc
	sub	sp, sp, #80
	stp	x22, x21, [sp, #32]
	stp	x20, x19, [sp, #48]
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	ldp	x20, x19, [x0, #8]
	add	x8, sp, #8
	mov	x0, x1
	bl	__RNvMsa_NtCs5dyeT9KiOLK_4core3fmtNtB5_9Formatter10debug_list
	cbz	x19, LBB12_3
	lsl	x21, x19, #3
Lloh50:
	adrp	x19, l_anon.5134cccca99e7a045e7be4444c8c72b6.1@PAGE
Lloh51:
	add	x19, x19, l_anon.5134cccca99e7a045e7be4444c8c72b6.1@PAGEOFF
LBB12_2:
	str	x20, [sp, #24]
	add	x20, x20, #8
	add	x0, sp, #8
	add	x1, sp, #24
	mov	x2, x19
	bl	__RNvMs5_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_9DebugList5entry
	subs	x21, x21, #8
	b.ne	LBB12_2
LBB12_3:
	add	x0, sp, #8
	bl	__RNvMs5_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_9DebugList6finish
	ldp	x29, x30, [sp, #64]
	ldp	x20, x19, [sp, #48]
	ldp	x22, x21, [sp, #32]
	add	sp, sp, #80
	ret
	.loh AdrpAdd	Lloh50, Lloh51
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x3, x1
	sxtw	x2, w0
Lloh52:
	adrp	x8, __RNvCshdop7HPDjU1_14p2_composition4main@PAGE
Lloh53:
	add	x8, x8, __RNvCshdop7HPDjU1_14p2_composition4main@PAGEOFF
	str	x8, [sp, #8]
Lloh54:
	adrp	x1, l_anon.5134cccca99e7a045e7be4444c8c72b6.2@PAGE
Lloh55:
	add	x1, x1, l_anon.5134cccca99e7a045e7be4444c8c72b6.2@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh54, Lloh55
	.loh AdrpAdd	Lloh52, Lloh53

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.5134cccca99e7a045e7be4444c8c72b6.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNvXs1g_NtCs5dyeT9KiOLK_4core3fmtRTjjENtB6_5Debug3fmtCshdop7HPDjU1_14p2_composition

	.p2align	3, 0x0
l_anon.5134cccca99e7a045e7be4444c8c72b6.1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNvXs1g_NtCs5dyeT9KiOLK_4core3fmtRjNtB6_5Debug3fmtCshdop7HPDjU1_14p2_composition

	.p2align	3, 0x0
l_anon.5134cccca99e7a045e7be4444c8c72b6.2:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCshdop7HPDjU1_14p2_composition
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cshdop7HPDjU1_14p2_composition
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cshdop7HPDjU1_14p2_composition

	.section	__TEXT,__cstring,cstring_literals
l_anon.5134cccca99e7a045e7be4444c8c72b6.3:
	.asciz	"p2_composition.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.5134cccca99e7a045e7be4444c8c72b6.4:
	.quad	l_anon.5134cccca99e7a045e7be4444c8c72b6.3
	.asciz	"\021\000\000\000\000\000\000\000Y\000\000\000*\000\000"

	.p2align	3, 0x0
l_anon.5134cccca99e7a045e7be4444c8c72b6.5:
	.quad	l_anon.5134cccca99e7a045e7be4444c8c72b6.3
	.asciz	"\021\000\000\000\000\000\000\000K\000\000\000*\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.5134cccca99e7a045e7be4444c8c72b6.6:
	.asciz	"\000\000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\005\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\005\000\000\000\000\000\000"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.5134cccca99e7a045e7be4444c8c72b6.7:
	.quad	l_anon.5134cccca99e7a045e7be4444c8c72b6.3
	.asciz	"\021\000\000\000\000\000\000\000=\000\000\000&\000\000"

	.p2align	3, 0x0
l_anon.5134cccca99e7a045e7be4444c8c72b6.8:
	.quad	l_anon.5134cccca99e7a045e7be4444c8c72b6.3
	.asciz	"\021\000\000\000\000\000\000\0000\000\000\000&\000\000"

	.section	__TEXT,__literal8,8byte_literals
	.p2align	3, 0x0
l_anon.5134cccca99e7a045e7be4444c8c72b6.9:
	.asciz	"\000\020\000\000\000\000\000"

	.p2align	3, 0x0
l_anon.5134cccca99e7a045e7be4444c8c72b6.10:
	.asciz	"\r\000\000\000\000\000\000"

	.section	__TEXT,__cstring,cstring_literals
l_anon.5134cccca99e7a045e7be4444c8c72b6.11:
	.asciz	"\013elements = \300\013, stride = \300\020 bits, fields = \300\001\n"

l_anon.5134cccca99e7a045e7be4444c8c72b6.12:
	.asciz	"2round-trip mismatches (two-step vs packed input): \300\001\n"

l_anon.5134cccca99e7a045e7be4444c8c72b6.13:
	.asciz	"2composition mismatches (one-step vs two-step):    \300\001\n"

l_anon.5134cccca99e7a045e7be4444c8c72b6.14:
	.asciz	"\017sum_two_step = \300\021, sum_one_step = \300\001\n"

l_anon.5134cccca99e7a045e7be4444c8c72b6.15:
	.asciz	"\031period P = 8/gcd(WS,8) = \300\033, group bytes G = WS*P/8 = \300\001\n"

l_anon.5134cccca99e7a045e7be4444c8c72b6.16:
	.asciz	"\b  field \300\004 (o=\300\004, w=\300\016): mask width \300\036, lane shifts over the period \300\001\n"

.subsections_via_symbols
