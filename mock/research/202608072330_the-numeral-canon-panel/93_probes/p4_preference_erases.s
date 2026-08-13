	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__RINvNtCs5dyeT9KiOLK_4core3ptr9drop_glueINtNtCseduYQEDYcHM_5alloc3vec3VecNtNtBG_6string6StringEECs4uY3R60P7Lt_20p4_preference_erases:
	.cfi_startproc
	stp	x22, x21, [sp, #-48]!
	.cfi_def_cfa_offset 48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_remember_state
	mov	x20, x0
	ldp	x19, x21, [x0, #8]
	cbz	x21, LBB0_5
	add	x22, x19, #8
	b	LBB0_3
LBB0_2:
	add	x22, x22, #24
	subs	x21, x21, #1
	b.eq	LBB0_5
LBB0_3:
	ldur	x1, [x22, #-8]
	cbz	x1, LBB0_2
	ldr	x0, [x22]
	mov	w2, #1
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
	b	LBB0_2
LBB0_5:
	ldr	x8, [x20]
	cbz	x8, LBB0_7
	add	x8, x8, x8, lsl #1
	lsl	x1, x8, #3
	mov	x0, x19
	mov	w2, #8
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	b	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
LBB0_7:
	.cfi_restore_state
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	ret
	.cfi_endproc

	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECs4uY3R60P7Lt_20p4_preference_erases
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECs4uY3R60P7Lt_20p4_preference_erases
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECs4uY3R60P7Lt_20p4_preference_erases:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x4, x3
	mov	x3, x2
	mov	x2, x1
	str	x0, [sp, #8]
Lloh0:
	adrp	x1, l_anon.2cd7e48f040dfd2430fbbf3246836027.7@PAGE
Lloh1:
	add	x1, x1, l_anon.2cd7e48f040dfd2430fbbf3246836027.7@PAGEOFF
	add	x0, sp, #8
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.loh AdrpAdd	Lloh0, Lloh1
	.cfi_endproc

	.p2align	2
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs4uY3R60P7Lt_20p4_preference_erases:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr	x0
	; InlineAsm Start
	; InlineAsm End
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RINvNvMs2_NtCseduYQEDYcHM_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs4uY3R60P7Lt_20p4_preference_erases:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_remember_state
	adds	x8, x2, x1
	b.hs	LBB3_3
	mov	x5, x4
	mov	x4, x3
	mov	x19, x0
	ldp	x1, x2, [x0]
	lsl	x9, x1, #1
	cmp	x8, x9
	csel	x8, x8, x9, hi
	mov	w9, #4
	mov	w10, #8
	cmp	x5, #1
	csel	x9, x10, x9, eq
	cmp	x8, x9
	csel	x20, x8, x9, hi
	add	x0, sp, #8
	mov	x3, x20
	bl	__RNvMs4_NtCseduYQEDYcHM_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs4uY3R60P7Lt_20p4_preference_erases
	ldr	x8, [sp, #8]
	cmp	x8, #1
	b.eq	LBB3_4
	ldr	x8, [sp, #16]
	stp	x20, x8, [x19]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	ret
LBB3_3:
	.cfi_restore_state
	mov	x0, #0
	bl	__RNvNtCseduYQEDYcHM_5alloc7raw_vec12handle_error
LBB3_4:
	ldp	x0, x1, [sp, #16]
	bl	__RNvNtCseduYQEDYcHM_5alloc7raw_vec12handle_error
	.cfi_endproc

	.p2align	2
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cs4uY3R60P7Lt_20p4_preference_erases:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs4uY3R60P7Lt_20p4_preference_erases
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCs4uY3R60P7Lt_20p4_preference_erases:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs4uY3R60P7Lt_20p4_preference_erases
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__RNvCs4uY3R60P7Lt_20p4_preference_erases4main
	.globl	__RNvCs4uY3R60P7Lt_20p4_preference_erases4main
	.p2align	2
__RNvCs4uY3R60P7Lt_20p4_preference_erases4main:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	sub	sp, sp, #288
	.cfi_def_cfa_offset 288
	stp	x28, x27, [sp, #192]
	stp	x26, x25, [sp, #208]
	stp	x24, x23, [sp, #224]
	stp	x22, x21, [sp, #240]
	stp	x20, x19, [sp, #256]
	stp	x29, x30, [sp, #272]
	add	x29, sp, #272
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
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_remember_state
Lloh2:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.14@PAGE
Lloh3:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.14@PAGEOFF
	mov	w1, #81
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh4:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.15@PAGE
Lloh5:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.15@PAGEOFF
	mov	w1, #81
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh6:
	adrp	x19, l_anon.2cd7e48f040dfd2430fbbf3246836027.16@PAGE
Lloh7:
	add	x19, x19, l_anon.2cd7e48f040dfd2430fbbf3246836027.16@PAGEOFF
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh8:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.17@PAGE
Lloh9:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.17@PAGEOFF
	mov	w1, #129
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh10:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.18@PAGE
Lloh11:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.18@PAGEOFF
	mov	w1, #133
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh12:
	adrp	x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.20@PAGE
Lloh13:
	add	x8, x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.20@PAGEOFF
Lloh14:
	adrp	x27, __RNvXs1i_NtCs5dyeT9KiOLK_4core3fmtReNtB6_7Display3fmtCs4uY3R60P7Lt_20p4_preference_erases@PAGE
Lloh15:
	add	x27, x27, __RNvXs1i_NtCs5dyeT9KiOLK_4core3fmtReNtB6_7Display3fmtCs4uY3R60P7Lt_20p4_preference_erases@PAGEOFF
	stp	x8, x27, [sp, #56]
Lloh16:
	adrp	x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.22@PAGE
Lloh17:
	add	x8, x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.22@PAGEOFF
	stp	x8, x27, [sp, #72]
Lloh18:
	adrp	x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.24@PAGE
Lloh19:
	add	x8, x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.24@PAGEOFF
	stp	x8, x27, [sp, #88]
Lloh20:
	adrp	x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.26@PAGE
Lloh21:
	add	x8, x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.26@PAGEOFF
	stp	x8, x27, [sp, #104]
Lloh22:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.27@PAGE
Lloh23:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.27@PAGEOFF
	add	x1, sp, #56
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh24:
	adrp	x28, l_anon.2cd7e48f040dfd2430fbbf3246836027.12@PAGE
Lloh25:
	add	x28, x28, l_anon.2cd7e48f040dfd2430fbbf3246836027.12@PAGEOFF
	stp	x28, x27, [sp, #56]
Lloh26:
	adrp	x21, l_anon.2cd7e48f040dfd2430fbbf3246836027.8@PAGE
Lloh27:
	add	x21, x21, l_anon.2cd7e48f040dfd2430fbbf3246836027.8@PAGEOFF
Lloh28:
	adrp	x25, __RNvXs8_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impmNtB9_7Display3fmt@GOTPAGE
Lloh29:
	ldr	x25, [x25, __RNvXs8_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impmNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x21, x25, [sp, #72]
	add	x8, x21, #4
	stp	x8, x25, [sp, #88]
	add	x8, x21, #8
	stp	x8, x25, [sp, #104]
Lloh30:
	adrp	x20, l_anon.2cd7e48f040dfd2430fbbf3246836027.40@PAGE
Lloh31:
	add	x20, x20, l_anon.2cd7e48f040dfd2430fbbf3246836027.40@PAGEOFF
	add	x1, sp, #56
	mov	x0, x20
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, x28, #16
	stp	x8, x27, [sp, #56]
	add	x8, x21, #12
	stp	x8, x25, [sp, #72]
	add	x8, x21, #16
	stp	x8, x25, [sp, #88]
	add	x8, x21, #20
	stp	x8, x25, [sp, #104]
	add	x1, sp, #56
	mov	x0, x20
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, x28, #32
	stp	x8, x27, [sp, #56]
	add	x8, x21, #24
	stp	x8, x25, [sp, #72]
	add	x8, x21, #28
	stp	x8, x25, [sp, #88]
	add	x8, x21, #32
	stp	x8, x25, [sp, #104]
	add	x1, sp, #56
	mov	x0, x20
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	bl	__RNvCske4UNIzLImn_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w20, #8
	mov	w0, #96
	mov	w1, #8
	bl	__RNvCske4UNIzLImn_7___rustc12___rust_alloc
	cbz	x0, LBB6_60
	mov	x26, x0
	mov	x21, #0
Lloh32:
	adrp	x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.28@PAGE
Lloh33:
	add	x8, x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.28@PAGEOFF
	mov	w9, #11
	stp	x8, x9, [x0]
	mov	x8, #8
	movk	x8, #1, lsl #32
	mov	w12, #1
	str	x8, [x0, #16]
Lloh34:
	adrp	x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.29@PAGE
Lloh35:
	add	x8, x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.29@PAGEOFF
	str	w12, [x0, #24]
	mov	w9, #12
	mov	x10, #1
	movk	x10, #8, lsl #32
Lloh36:
	adrp	x11, l_anon.2cd7e48f040dfd2430fbbf3246836027.31@PAGE
Lloh37:
	add	x11, x11, l_anon.2cd7e48f040dfd2430fbbf3246836027.31@PAGEOFF
	stp	x8, x9, [x0, #32]
	mov	w8, #15
	mov	x9, #4294967297
	str	x10, [x0, #48]
	mov	w10, #1
	stp	x10, x0, [sp]
	str	w12, [x0, #56]
	sub	x23, x29, #92
Lloh38:
	adrp	x19, l_anon.2cd7e48f040dfd2430fbbf3246836027.13@PAGE
Lloh39:
	add	x19, x19, l_anon.2cd7e48f040dfd2430fbbf3246836027.13@PAGEOFF
	stp	x11, x8, [x0, #64]
	str	x9, [x0, #80]
	str	w20, [x0, #88]
	mov	w20, #3
LBB6_2:
	add	x8, x26, x21
	ldr	x9, [x8]
	cbz	x9, LBB6_40
	ldp	x10, x11, [x8, #8]
	stp	x9, x10, [sp, #24]
	str	x11, [sp, #40]
	ldr	w8, [x8, #24]
	str	w8, [sp, #48]
	lsr	x9, x11, #32
	add	w10, w8, w8, lsl #3
	add	w12, w9, w11
	add	w10, w12, w10
	add	w12, w11, w11, lsl #1
	add	w12, w9, w12
	add	w8, w12, w8, lsl #2
	cmp	w8, w10
	csel	w8, w8, w10, lo
	cset	w10, lo
	add	w9, w9, w9, lsl #1
	sub	w9, w9, w11
	add	w9, w9, w11, lsl #3
	cmp	w9, w8
	mov	w8, #2
	csel	x8, x8, x10, lo
	str	x8, [sp, #16]
	bl	__RNvCske4UNIzLImn_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w0, #72
	mov	w1, #8
	bl	__RNvCske4UNIzLImn_7___rustc12___rust_alloc
	cbz	x0, LBB6_61
	mov	x24, x0
	stp	x20, x0, [sp, #120]
	ldp	w8, w9, [sp, #40]
	ldr	w10, [sp, #48]
	add	w10, w10, w10, lsl #3
	add	w8, w9, w8
	add	w8, w8, w10
	stur	w8, [x29, #-92]
	stp	x28, x27, [sp, #56]
	stp	x23, x25, [sp, #72]
Ltmp0:
	mov	x22, #0
	sub	x8, x29, #128
	add	x1, sp, #56
	mov	x26, x19
	mov	x0, x19
	bl	__RNvNvNtCseduYQEDYcHM_5alloc3fmt6format12format_inner
Ltmp1:
	ldur	q0, [x29, #-128]
	str	q0, [x24]
	ldur	x8, [x29, #-112]
	str	x8, [x24, #16]
	ldp	w8, w9, [sp, #40]
	add	w8, w8, w8, lsl #1
	add	w8, w9, w8
	ldr	w9, [sp, #48]
	add	w8, w8, w9, lsl #2
	stur	w8, [x29, #-92]
Lloh40:
	adrp	x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.12@PAGE+16
Lloh41:
	add	x8, x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.12@PAGEOFF+16
	stp	x8, x27, [sp, #56]
	stp	x23, x25, [sp, #72]
Ltmp2:
	sub	x8, x29, #128
	add	x1, sp, #56
	mov	w22, #1
	mov	x0, x26
	bl	__RNvNvNtCseduYQEDYcHM_5alloc3fmt6format12format_inner
Ltmp3:
	ldur	q0, [x29, #-128]
	stur	q0, [x24, #24]
	ldur	x8, [x29, #-112]
	str	x8, [x24, #40]
	ldp	w8, w9, [sp, #40]
	add	w9, w9, w9, lsl #1
	sub	w9, w9, w8
	add	w8, w9, w8, lsl #3
	stur	w8, [x29, #-92]
Lloh42:
	adrp	x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.12@PAGE+32
Lloh43:
	add	x8, x8, l_anon.2cd7e48f040dfd2430fbbf3246836027.12@PAGEOFF+32
	stp	x8, x27, [sp, #56]
	stp	x23, x25, [sp, #72]
Ltmp4:
	sub	x8, x29, #128
	add	x1, sp, #56
	mov	w22, #2
	mov	x0, x26
	bl	__RNvNvNtCseduYQEDYcHM_5alloc3fmt6format12format_inner
Ltmp5:
	ldur	q0, [x29, #-128]
	str	q0, [x24, #48]
	ldur	q0, [sp, #120]
	stur	q0, [x29, #-128]
	ldp	x23, x8, [x29, #-120]
	str	x8, [x24, #64]
	stur	x20, [x29, #-112]
	cbz	x20, LBB6_20
	mov	x20, x25
	ldr	x25, [x23, #16]
	mov	w9, #72
	sub	x22, x9, #24
	lsr	x8, x22, #3
	mov	x10, #6148914691236517205
	movk	x10, #21846
	mul	x8, x8, x10
	adds	x24, x25, x8
	b.hs	LBB6_59
	add	x8, x23, #24
	ldr	x26, [x23, #8]
	sub	x9, x9, #24
LBB6_10:
	cbz	x9, LBB6_12
	ldr	x10, [x8, #16]
	add	x8, x8, #24
	sub	x9, x9, #24
	adds	x24, x10, x24
	b.lo	LBB6_10
	b	LBB6_59
LBB6_12:
	tbnz	x24, #63, LBB6_56
	cbz	x24, LBB6_30
	bl	__RNvCske4UNIzLImn_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	x0, x24
	mov	w1, #1
	bl	__RNvCske4UNIzLImn_7___rustc12___rust_alloc
	mov	x27, x0
	cbz	x0, LBB6_57
	stp	x24, x27, [sp, #56]
	str	xzr, [sp, #72]
	cmp	x25, x24
	b.hi	LBB6_31
LBB6_16:
	mov	x28, #0
	mov	x8, #0
	cbz	x25, LBB6_18
LBB6_17:
	add	x0, x27, x28
	mov	x1, x26
	mov	x2, x25
	bl	_memcpy
	mov	x8, x28
LBB6_18:
	add	x8, x8, x25
	sub	x25, x24, x8
	mov	w9, #3
	cmp	x9, #1
	b.ne	LBB6_21
Lloh44:
	adrp	x27, __RNvXs1i_NtCs5dyeT9KiOLK_4core3fmtReNtB6_7Display3fmtCs4uY3R60P7Lt_20p4_preference_erases@PAGE
Lloh45:
	add	x27, x27, __RNvXs1i_NtCs5dyeT9KiOLK_4core3fmtReNtB6_7Display3fmtCs4uY3R60P7Lt_20p4_preference_erases@PAGEOFF
Lloh46:
	adrp	x28, l_anon.2cd7e48f040dfd2430fbbf3246836027.12@PAGE
Lloh47:
	add	x28, x28, l_anon.2cd7e48f040dfd2430fbbf3246836027.12@PAGEOFF
	b	LBB6_25
LBB6_20:
	mov	w8, #1
	stp	xzr, x8, [sp, #120]
	str	xzr, [sp, #136]
	ldr	x26, [sp, #8]
	b	LBB6_34
LBB6_21:
	add	x26, x27, x8
	add	x23, x23, #40
Lloh48:
	adrp	x27, __RNvXs1i_NtCs5dyeT9KiOLK_4core3fmtReNtB6_7Display3fmtCs4uY3R60P7Lt_20p4_preference_erases@PAGE
Lloh49:
	add	x27, x27, __RNvXs1i_NtCs5dyeT9KiOLK_4core3fmtReNtB6_7Display3fmtCs4uY3R60P7Lt_20p4_preference_erases@PAGEOFF
Lloh50:
	adrp	x28, l_anon.2cd7e48f040dfd2430fbbf3246836027.12@PAGE
Lloh51:
	add	x28, x28, l_anon.2cd7e48f040dfd2430fbbf3246836027.12@PAGEOFF
LBB6_22:
	subs	x8, x25, #2
	b.lo	LBB6_58
	ldp	x1, x2, [x23, #-8]
	mov	w9, #8224
	strh	w9, [x26]
	subs	x25, x8, x2
	b.lo	LBB6_58
	add	x23, x23, #24
	add	x0, x26, #2
	add	x26, x0, x2
	bl	_memcpy
	subs	x22, x22, #24
	b.ne	LBB6_22
LBB6_25:
	sub	x8, x24, x25
	ldp	x9, x10, [sp, #56]
	ldp	x23, x22, [x29, #-120]
	stp	x9, x10, [sp, #120]
	str	x8, [sp, #136]
	ldr	x26, [sp, #8]
	cbz	x22, LBB6_33
	add	x24, x23, #8
	mov	x25, x20
	mov	w20, #3
	b	LBB6_28
LBB6_27:
	add	x24, x24, #24
	subs	x22, x22, #1
	b.eq	LBB6_34
LBB6_28:
	ldur	x1, [x24, #-8]
	cbz	x1, LBB6_27
	ldr	x0, [x24]
	mov	w2, #1
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
	b	LBB6_27
LBB6_30:
	mov	w27, #1
	stp	x24, x27, [sp, #56]
	str	xzr, [sp, #72]
	cmp	x25, x24
	b.ls	LBB6_16
LBB6_31:
Ltmp7:
	add	x0, sp, #56
	mov	x1, #0
	mov	x2, x25
	mov	w3, #1
	mov	w4, #1
	bl	__RINvNvMs2_NtCseduYQEDYcHM_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs4uY3R60P7Lt_20p4_preference_erases
Ltmp8:
	ldp	x27, x28, [sp, #64]
	b	LBB6_17
LBB6_33:
	mov	x25, x20
	mov	w20, #3
LBB6_34:
	ldur	x8, [x29, #-128]
	cbz	x8, LBB6_36
	add	x8, x8, x8, lsl #1
	lsl	x1, x8, #3
	mov	x0, x23
	mov	w2, #8
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
LBB6_36:
	add	x8, sp, #24
	stp	x8, x27, [sp, #56]
	ldr	x8, [sp, #16]
	add	x8, x28, x8, lsl #4
	add	x9, sp, #40
	str	x9, [sp, #72]
Lloh52:
	adrp	x9, __RNvXsa_NtCs5dyeT9KiOLK_4core5arrayAmj3_NtNtB7_3fmt5Debug3fmtCs4uY3R60P7Lt_20p4_preference_erases@PAGE
Lloh53:
	add	x9, x9, __RNvXsa_NtCs5dyeT9KiOLK_4core5arrayAmj3_NtNtB7_3fmt5Debug3fmtCs4uY3R60P7Lt_20p4_preference_erases@PAGEOFF
	stp	x9, x8, [sp, #80]
	add	x8, sp, #120
	stp	x27, x8, [sp, #96]
Lloh54:
	adrp	x8, __RNvXsq_NtCseduYQEDYcHM_5alloc6stringNtB5_6StringNtNtCs5dyeT9KiOLK_4core3fmt7Display3fmt@PAGE
Lloh55:
	add	x8, x8, __RNvXsq_NtCseduYQEDYcHM_5alloc6stringNtB5_6StringNtNtCs5dyeT9KiOLK_4core3fmt7Display3fmt@PAGEOFF
	str	x8, [sp, #112]
Ltmp18:
	add	x1, sp, #56
Lloh56:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.39@PAGE
Lloh57:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.39@PAGEOFF
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Ltmp19:
	ldr	x1, [sp, #120]
	sub	x23, x29, #92
	cbz	x1, LBB6_39
	ldr	x0, [sp, #128]
	mov	w2, #1
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
LBB6_39:
	add	x21, x21, #32
	cmp	x21, #96
	b.ne	LBB6_2
LBB6_40:
	mov	x0, x26
	mov	w1, #96
	mov	w2, #8
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
Lloh58:
	adrp	x19, l_anon.2cd7e48f040dfd2430fbbf3246836027.16@PAGE
Lloh59:
	add	x19, x19, l_anon.2cd7e48f040dfd2430fbbf3246836027.16@PAGEOFF
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh60:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.32@PAGE
Lloh61:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.32@PAGEOFF
	mov	w1, #123
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh62:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.33@PAGE
Lloh63:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.33@PAGEOFF
	mov	w1, #135
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh64:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.34@PAGE
Lloh65:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.34@PAGEOFF
	mov	w1, #77
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	bl	__RNvCske4UNIzLImn_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w0, #8000
	mov	w1, #8
	bl	__RNvCske4UNIzLImn_7___rustc12___rust_alloc
	cbz	x0, LBB6_63
	mov	x19, x0
	mov	x8, #0
	mov	x9, #-8000
	mov	w10, #11469
	mov	w11, #4093
	mov	w12, #15292
LBB6_42:
	add	w13, w8, #3823
	sub	w14, w10, #3823
	and	x15, x8, #0xffc
	and	x13, x13, #0xfff
	and	x14, x14, #0xffe
	add	x16, x19, x9
	str	x15, [x16, #8000]
	str	x13, [x16, #8008]
	and	x13, x10, x11
	str	x14, [x16, #8016]
	str	x13, [x16, #8024]
	add	x10, x10, x12
	add	x8, x8, x12
	adds	x9, x9, #32
	b.ne	LBB6_42
	mov	x8, #0
LBB6_44:
	ldr	x10, [x19, x8, lsl #3]
	add	w9, w10, w9
	and	x9, x9, #0xfff
	add	x8, x8, #1
	cmp	x8, #1000
	b.ne	LBB6_44
	mov	x20, #0
	mov	x8, #0
	str	x9, [sp, #40]
LBB6_46:
	ldr	x9, [x19, x8, lsl #3]
	add	w9, w9, w20
	and	x20, x9, #0xfff
	add	x8, x8, #1
	cmp	x8, #1000
	b.ne	LBB6_46
	str	x20, [sp, #24]
	movi.2d	v0, #0000000000000000
	mov	x8, #-8000
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB6_48:
	add	x9, x19, x8
	ldr	q4, [x9, #8000]
	ldr	q5, [x9, #8016]
	ldr	q6, [x9, #8032]
	ldr	q7, [x9, #8048]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	adds	x8, x8, #64
	b.ne	LBB6_48
	add.2d	v0, v1, v0
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	addp.2d	d0, v0
	str	d0, [sp, #120]
	movi.2d	v0, #0000000000000000
	mov	x8, #-8000
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB6_50:
	add	x9, x19, x8
	ldr	q4, [x9, #8000]
	ldr	q5, [x9, #8016]
	ldr	q6, [x9, #8032]
	ldr	q7, [x9, #8048]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	adds	x8, x8, #64
	b.ne	LBB6_50
	add.2d	v0, v1, v0
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	addp.2d	d0, v0
	stur	d0, [x29, #-128]
	fmov	x22, d0
Lloh66:
	adrp	x21, __RNvXsd_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impyNtB9_7Display3fmt@GOTPAGE
Lloh67:
	ldr	x21, [x21, __RNvXsd_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impyNtB9_7Display3fmt@GOTPAGEOFF]
	add	x8, sp, #40
	stp	x8, x21, [sp, #56]
Ltmp24:
Lloh68:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.35@PAGE
Lloh69:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.35@PAGEOFF
	add	x1, sp, #56
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Ltmp25:
	ldr	x8, [sp, #40]
	cmp	x8, x20
	cset	w8, eq
	sturb	w8, [x29, #-92]
	add	x8, sp, #24
	stp	x8, x21, [sp, #56]
	sub	x8, x29, #92
Lloh70:
	adrp	x20, __RNvXsg_NtCs5dyeT9KiOLK_4core3fmtbNtB5_7Display3fmt@GOTPAGE
Lloh71:
	ldr	x20, [x20, __RNvXsg_NtCs5dyeT9KiOLK_4core3fmtbNtB5_7Display3fmt@GOTPAGEOFF]
	stp	x8, x20, [sp, #72]
Ltmp26:
Lloh72:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.36@PAGE
Lloh73:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.36@PAGEOFF
	add	x1, sp, #56
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Ltmp27:
	add	x8, sp, #120
	stp	x8, x21, [sp, #56]
Ltmp28:
Lloh74:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.37@PAGE
Lloh75:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.37@PAGEOFF
	add	x1, sp, #56
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Ltmp29:
	ldr	x8, [sp, #120]
	cmp	x8, x22
	cset	w8, eq
	sturb	w8, [x29, #-92]
	sub	x8, x29, #128
	stp	x8, x21, [sp, #56]
	sub	x8, x29, #92
	stp	x8, x20, [sp, #72]
Ltmp30:
Lloh76:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.38@PAGE
Lloh77:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.38@PAGEOFF
	add	x1, sp, #56
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Ltmp31:
	mov	x0, x19
	mov	w1, #8000
	mov	w2, #8
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
	.cfi_def_cfa wsp, 288
	ldp	x29, x30, [sp, #272]
	ldp	x20, x19, [sp, #256]
	ldp	x22, x21, [sp, #240]
	ldp	x24, x23, [sp, #224]
	ldp	x26, x25, [sp, #208]
	ldp	x28, x27, [sp, #192]
	add	sp, sp, #288
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB6_56:
	.cfi_restore_state
	str	xzr, [sp]
LBB6_57:
Ltmp13:
	ldr	x0, [sp]
	mov	x1, x24
	bl	__RNvNtCseduYQEDYcHM_5alloc7raw_vec12handle_error
Ltmp14:
	b	LBB6_62
LBB6_58:
Ltmp10:
Lloh78:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.41@PAGE
Lloh79:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.41@PAGEOFF
Lloh80:
	adrp	x2, l_anon.2cd7e48f040dfd2430fbbf3246836027.4@PAGE
Lloh81:
	add	x2, x2, l_anon.2cd7e48f040dfd2430fbbf3246836027.4@PAGEOFF
	mov	w1, #19
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking9panic_fmt
Ltmp11:
	b	LBB6_62
LBB6_59:
Ltmp15:
Lloh82:
	adrp	x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.5@PAGE
Lloh83:
	add	x0, x0, l_anon.2cd7e48f040dfd2430fbbf3246836027.5@PAGEOFF
Lloh84:
	adrp	x2, l_anon.2cd7e48f040dfd2430fbbf3246836027.6@PAGE
Lloh85:
	add	x2, x2, l_anon.2cd7e48f040dfd2430fbbf3246836027.6@PAGEOFF
	mov	w1, #53
	bl	__RNvNtCs5dyeT9KiOLK_4core6option13expect_failed
Ltmp16:
	b	LBB6_62
LBB6_60:
	mov	w0, #8
	mov	w1, #96
	bl	__RNvNtCseduYQEDYcHM_5alloc5alloc18handle_alloc_error
LBB6_61:
Ltmp21:
	mov	w0, #8
	mov	w1, #72
	bl	__RNvNtCseduYQEDYcHM_5alloc7raw_vec12handle_error
Ltmp22:
LBB6_62:
	brk	#0x1
LBB6_63:
	mov	w0, #8
	mov	w1, #8000
	bl	__RNvNtCseduYQEDYcHM_5alloc7raw_vec12handle_error
LBB6_64:
Ltmp9:
	b	LBB6_72
LBB6_65:
Ltmp32:
	mov	x20, x0
	mov	x0, x19
	mov	w1, #8000
	mov	w2, #8
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
	mov	x0, x20
	bl	__Unwind_Resume
LBB6_66:
Ltmp20:
	mov	x20, x0
	ldr	x1, [sp, #120]
	cbz	x1, LBB6_70
	ldr	x0, [sp, #128]
	mov	w2, #1
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
	b	LBB6_70
LBB6_68:
Ltmp23:
	mov	x20, x0
	b	LBB6_70
LBB6_69:
Ltmp6:
	mov	x20, x0
	str	x22, [sp, #136]
	add	x0, sp, #120
	bl	__RINvNtCs5dyeT9KiOLK_4core3ptr9drop_glueINtNtCseduYQEDYcHM_5alloc3vec3VecNtNtBG_6string6StringEECs4uY3R60P7Lt_20p4_preference_erases
LBB6_70:
	ldr	x19, [sp, #8]
	mov	x0, x19
	mov	w1, #96
	mov	w2, #8
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
	mov	x0, x20
	bl	__Unwind_Resume
LBB6_71:
Ltmp12:
LBB6_72:
	mov	x20, x0
	ldr	x19, [sp, #8]
	ldr	x1, [sp, #56]
	cbz	x1, LBB6_75
	ldr	x0, [sp, #64]
	mov	w2, #1
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
	b	LBB6_75
LBB6_74:
Ltmp17:
	mov	x20, x0
	ldr	x19, [sp, #8]
LBB6_75:
	sub	x0, x29, #128
	bl	__RINvNtCs5dyeT9KiOLK_4core3ptr9drop_glueINtNtCseduYQEDYcHM_5alloc3vec3VecNtNtBG_6string6StringEECs4uY3R60P7Lt_20p4_preference_erases
	mov	x0, x19
	mov	w1, #96
	mov	w2, #8
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
	mov	x0, x20
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpLdrGot	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh38, Lloh39
	.loh AdrpAdd	Lloh36, Lloh37
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpAdd	Lloh46, Lloh47
	.loh AdrpAdd	Lloh44, Lloh45
	.loh AdrpAdd	Lloh50, Lloh51
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpAdd	Lloh56, Lloh57
	.loh AdrpAdd	Lloh54, Lloh55
	.loh AdrpAdd	Lloh52, Lloh53
	.loh AdrpAdd	Lloh64, Lloh65
	.loh AdrpAdd	Lloh62, Lloh63
	.loh AdrpAdd	Lloh60, Lloh61
	.loh AdrpAdd	Lloh58, Lloh59
	.loh AdrpAdd	Lloh68, Lloh69
	.loh AdrpLdrGot	Lloh66, Lloh67
	.loh AdrpAdd	Lloh72, Lloh73
	.loh AdrpLdrGot	Lloh70, Lloh71
	.loh AdrpAdd	Lloh74, Lloh75
	.loh AdrpAdd	Lloh76, Lloh77
	.loh AdrpAdd	Lloh80, Lloh81
	.loh AdrpAdd	Lloh78, Lloh79
	.loh AdrpAdd	Lloh84, Lloh85
	.loh AdrpAdd	Lloh82, Lloh83
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table6:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Lfunc_begin0-Lfunc_begin0
	.uleb128 Ltmp0-Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp5-Ltmp0
	.uleb128 Ltmp6-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp5-Lfunc_begin0
	.uleb128 Ltmp7-Ltmp5
	.byte	0
	.byte	0
	.uleb128 Ltmp7-Lfunc_begin0
	.uleb128 Ltmp8-Ltmp7
	.uleb128 Ltmp9-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp18-Lfunc_begin0
	.uleb128 Ltmp19-Ltmp18
	.uleb128 Ltmp20-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp19-Lfunc_begin0
	.uleb128 Ltmp24-Ltmp19
	.byte	0
	.byte	0
	.uleb128 Ltmp24-Lfunc_begin0
	.uleb128 Ltmp31-Ltmp24
	.uleb128 Ltmp32-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp13-Lfunc_begin0
	.uleb128 Ltmp14-Ltmp13
	.uleb128 Ltmp17-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp10-Lfunc_begin0
	.uleb128 Ltmp11-Ltmp10
	.uleb128 Ltmp12-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp15-Lfunc_begin0
	.uleb128 Ltmp16-Ltmp15
	.uleb128 Ltmp17-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp16-Lfunc_begin0
	.uleb128 Ltmp21-Ltmp16
	.byte	0
	.byte	0
	.uleb128 Ltmp21-Lfunc_begin0
	.uleb128 Ltmp22-Ltmp21
	.uleb128 Ltmp23-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp22-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp22
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__RNvMs4_NtCseduYQEDYcHM_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs4uY3R60P7Lt_20p4_preference_erases:
	.cfi_startproc
	stp	x22, x21, [sp, #-48]!
	.cfi_def_cfa_offset 48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	mov	x19, x0
	mov	x9, #0
	umulh	x11, x5, x3
	mov	w8, #1
	mov	w10, #8
	cmp	xzr, x11
	b.ne	LBB7_10
	mov	x21, x4
	mul	x20, x5, x3
	mov	x11, #-9223372036854775808
	sub	x11, x11, x4
	cmp	x20, x11
	b.hi	LBB7_10
	cbz	x1, LBB7_4
	mul	x1, x5, x1
	mov	x0, x2
	mov	x2, x21
	mov	x3, x20
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_realloc
	cbnz	x0, LBB7_8
	b	LBB7_6
LBB7_4:
	cbz	x20, LBB7_7
	bl	__RNvCske4UNIzLImn_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	x0, x20
	mov	x1, x21
	bl	__RNvCske4UNIzLImn_7___rustc12___rust_alloc
	cbnz	x0, LBB7_8
LBB7_6:
	str	x21, [x19, #8]
	mov	w8, #1
	b	LBB7_9
LBB7_7:
	mov	x0, x21
LBB7_8:
	mov	x8, #0
	str	x0, [x19, #8]
LBB7_9:
	mov	w10, #16
	mov	x9, x20
LBB7_10:
	str	x9, [x19, x10]
	str	x8, [x19]
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	ldp	x22, x21, [sp], #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	ret
	.cfi_endproc

	.p2align	2
__RNvXs1g_NtCs5dyeT9KiOLK_4core3fmtRmNtB6_5Debug3fmtCs4uY3R60P7Lt_20p4_preference_erases:
	.cfi_startproc
	ldr	x0, [x0]
	ldr	w8, [x1, #16]
	tbnz	w8, #25, LBB8_3
	tbnz	w8, #26, LBB8_4
	b	__RNvXs8_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impmNtB9_7Display3fmt
LBB8_3:
	b	__RNvXsu_NtNtCs5dyeT9KiOLK_4core3fmt3nummNtB7_8LowerHex3fmt
LBB8_4:
	b	__RNvXsw_NtNtCs5dyeT9KiOLK_4core3fmt3nummNtB7_8UpperHex3fmt
	.cfi_endproc

	.p2align	2
__RNvXs1i_NtCs5dyeT9KiOLK_4core3fmtReNtB6_7Display3fmtCs4uY3R60P7Lt_20p4_preference_erases:
	.cfi_startproc
	mov	x2, x1
	ldp	x8, x1, [x0]
	mov	x0, x8
	b	__RNvXsi_NtCs5dyeT9KiOLK_4core3fmteNtB5_7Display3fmt
	.cfi_endproc

	.p2align	2
__RNvXsa_NtCs5dyeT9KiOLK_4core5arrayAmj3_NtNtB7_3fmt5Debug3fmtCs4uY3R60P7Lt_20p4_preference_erases:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
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
Lloh86:
	adrp	x20, l_anon.2cd7e48f040dfd2430fbbf3246836027.2@PAGE
Lloh87:
	add	x20, x20, l_anon.2cd7e48f040dfd2430fbbf3246836027.2@PAGEOFF
	add	x0, sp, #8
	add	x1, sp, #24
	mov	x2, x20
	bl	__RNvMs5_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_9DebugList5entry
	add	x8, x19, #4
	str	x8, [sp, #24]
	add	x0, sp, #8
	add	x1, sp, #24
	mov	x2, x20
	bl	__RNvMs5_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_9DebugList5entry
	add	x8, x19, #8
	str	x8, [sp, #24]
	add	x0, sp, #8
	add	x1, sp, #24
	mov	x2, x20
	bl	__RNvMs5_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_9DebugList5entry
	add	x0, sp, #8
	bl	__RNvMs5_NtNtCs5dyeT9KiOLK_4core3fmt8buildersNtB5_9DebugList6finish
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	ret
	.loh AdrpAdd	Lloh86, Lloh87
	.cfi_endproc

	.p2align	2
__RNvXsq_NtCseduYQEDYcHM_5alloc6stringNtB5_6StringNtNtCs5dyeT9KiOLK_4core3fmt7Display3fmt:
	.cfi_startproc
	mov	x2, x1
	ldp	x8, x1, [x0, #8]
	mov	x0, x8
	b	__RNvXsi_NtCs5dyeT9KiOLK_4core3fmteNtB5_7Display3fmt
	.cfi_endproc

	.globl	_direct_widen
	.p2align	2
_direct_widen:
	.cfi_startproc
	cbz	x1, LBB12_3
	cmp	x1, #8
	b.hs	LBB12_4
	mov	x8, #0
	mov	x9, #0
	b	LBB12_7
LBB12_3:
	mov	x8, #0
	mov	x0, x8
	ret
LBB12_4:
	and	x9, x1, #0xffffffffffffff8
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	and	x10, x1, #0xffffffffffffff8
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB12_5:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	subs	x10, x10, #8
	b.ne	LBB12_5
	add.2d	v0, v1, v0
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x1, x9
	b.eq	LBB12_9
LBB12_7:
	sub	x10, x1, x9
	add	x9, x0, x9, lsl #3
LBB12_8:
	ldr	x11, [x9], #8
	add	x8, x11, x8
	subs	x10, x10, #1
	b.ne	LBB12_8
LBB12_9:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_direct_wrap
	.p2align	2
_direct_wrap:
	.cfi_startproc
	mov	x8, #0
	cbz	x1, LBB13_2
LBB13_1:
	ldr	w9, [x0], #8
	add	w8, w9, w8
	and	x8, x8, #0xfff
	subs	x1, x1, #1
	b.ne	LBB13_1
LBB13_2:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_pref_accuracy
	.p2align	2
_pref_accuracy:
	.cfi_startproc
	cbz	x1, LBB14_3
	cmp	x1, #8
	b.hs	LBB14_4
	mov	x8, #0
	mov	x9, #0
	b	LBB14_7
LBB14_3:
	mov	x8, #0
	mov	x0, x8
	ret
LBB14_4:
	and	x9, x1, #0xffffffffffffff8
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	and	x10, x1, #0xffffffffffffff8
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB14_5:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	subs	x10, x10, #8
	b.ne	LBB14_5
	add.2d	v0, v1, v0
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x1, x9
	b.eq	LBB14_9
LBB14_7:
	add	x10, x0, x9, lsl #3
	sub	x9, x1, x9
LBB14_8:
	ldr	x11, [x10], #8
	add	x8, x11, x8
	subs	x9, x9, #1
	b.ne	LBB14_8
LBB14_9:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_pref_speed
	.p2align	2
_pref_speed:
	.cfi_startproc
	mov	x8, #0
	cbz	x1, LBB15_2
LBB15_1:
	ldr	w9, [x0], #8
	add	w8, w9, w8
	and	x8, x8, #0xfff
	subs	x1, x1, #1
	b.ne	LBB15_1
LBB15_2:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	.cfi_startproc
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x3, x1
	sxtw	x2, w0
Lloh88:
	adrp	x8, __RNvCs4uY3R60P7Lt_20p4_preference_erases4main@PAGE
Lloh89:
	add	x8, x8, __RNvCs4uY3R60P7Lt_20p4_preference_erases4main@PAGEOFF
	str	x8, [sp, #8]
Lloh90:
	adrp	x1, l_anon.2cd7e48f040dfd2430fbbf3246836027.7@PAGE
Lloh91:
	add	x1, x1, l_anon.2cd7e48f040dfd2430fbbf3246836027.7@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh90, Lloh91
	.loh AdrpAdd	Lloh88, Lloh89
	.cfi_endproc

	.section	__TEXT,__const
	.p2align	3, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.0:
	.asciz	"\001\000\000\000\001\000\000\000\b\000\000"

	.p2align	3, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.1:
	.asciz	"\b\000\000\000\001\000\000\000\001\000\000"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.2:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNvXs1g_NtCs5dyeT9KiOLK_4core3fmtRmNtB6_5Debug3fmtCs4uY3R60P7Lt_20p4_preference_erases

	.section	__TEXT,__cstring,cstring_literals
l_anon.2cd7e48f040dfd2430fbbf3246836027.3:
	.asciz	"/Users/orgrinrt/.rustup/toolchains/nightly-2026-05-28-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/str.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.4:
	.quad	l_anon.2cd7e48f040dfd2430fbbf3246836027.3
	.asciz	"x\000\000\000\000\000\000\000\310\000\000\000\026\000\000"

	.section	__TEXT,__const
l_anon.2cd7e48f040dfd2430fbbf3246836027.5:
	.ascii	"attempt to join into collection with len > usize::MAX"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.6:
	.quad	l_anon.2cd7e48f040dfd2430fbbf3246836027.3
	.asciz	"x\000\000\000\000\000\000\000\257\000\000\000\n\000\000"

	.p2align	3, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.7:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCs4uY3R60P7Lt_20p4_preference_erases
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cs4uY3R60P7Lt_20p4_preference_erases
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cs4uY3R60P7Lt_20p4_preference_erases

	.section	__TEXT,__const
	.p2align	2, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.8:
	.asciz	"\001\000\000\000\001\000\000\000\t\000\000\000\003\000\000\000\001\000\000\000\004\000\000\000\007\000\000\000\003\000\000\000\000\000\000"

	.section	__TEXT,__literal4,4byte_literals
l_anon.2cd7e48f040dfd2430fbbf3246836027.9:
	.ascii	"wrap"

	.section	__TEXT,__literal8,8byte_literals
l_anon.2cd7e48f040dfd2430fbbf3246836027.10:
	.ascii	"saturate"

	.section	__TEXT,__const
l_anon.2cd7e48f040dfd2430fbbf3246836027.11:
	.ascii	"widen"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.12:
	.quad	l_anon.2cd7e48f040dfd2430fbbf3246836027.9
	.asciz	"\004\000\000\000\000\000\000"
	.quad	l_anon.2cd7e48f040dfd2430fbbf3246836027.10
	.asciz	"\b\000\000\000\000\000\000"
	.quad	l_anon.2cd7e48f040dfd2430fbbf3246836027.11
	.asciz	"\005\000\000\000\000\000\000"

	.section	__TEXT,__cstring,cstring_literals
l_anon.2cd7e48f040dfd2430fbbf3246836027.13:
	.asciz	"\300\001=\300"

	.section	__TEXT,__const
l_anon.2cd7e48f040dfd2430fbbf3246836027.14:
	.ascii	"P4. Preference resolution at const time\n"

l_anon.2cd7e48f040dfd2430fbbf3246836027.15:
	.ascii	"=======================================\n"

l_anon.2cd7e48f040dfd2430fbbf3246836027.16:
	.byte	10

l_anon.2cd7e48f040dfd2430fbbf3246836027.17:
	.ascii	"The cost model, resolved now at runtime for reporting only. The\n"

l_anon.2cd7e48f040dfd2430fbbf3246836027.18:
	.ascii	"emitted code resolved it at compile time; see the asm comparison.\n"

l_anon.2cd7e48f040dfd2430fbbf3246836027.19:
	.ascii	"arm"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.20:
	.quad	l_anon.2cd7e48f040dfd2430fbbf3246836027.19
	.asciz	"\003\000\000\000\000\000\000"

	.section	__TEXT,__literal4,4byte_literals
l_anon.2cd7e48f040dfd2430fbbf3246836027.21:
	.ascii	"time"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.22:
	.quad	l_anon.2cd7e48f040dfd2430fbbf3246836027.21
	.asciz	"\004\000\000\000\000\000\000"

	.section	__TEXT,__const
l_anon.2cd7e48f040dfd2430fbbf3246836027.23:
	.ascii	"space"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.24:
	.quad	l_anon.2cd7e48f040dfd2430fbbf3246836027.23
	.asciz	"\005\000\000\000\000\000\000"

	.section	__TEXT,__const
l_anon.2cd7e48f040dfd2430fbbf3246836027.25:
	.ascii	"error"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.26:
	.quad	l_anon.2cd7e48f040dfd2430fbbf3246836027.25
	.asciz	"\005\000\000\000\000\000\000"

	.section	__TEXT,__const
l_anon.2cd7e48f040dfd2430fbbf3246836027.27:
	.asciz	"\002  \303 \000\000\b\020\000\001 \303 \000\000(\006\000\001 \303 \000\000(\006\000\001 \303 \000\000(\006\000\004   \n"

l_anon.2cd7e48f040dfd2430fbbf3246836027.28:
	.ascii	"prefer time"

l_anon.2cd7e48f040dfd2430fbbf3246836027.29:
	.ascii	"prefer space"

	.p2align	3, 0x0
l_anon.2cd7e48f040dfd2430fbbf3246836027.30:
	.asciz	"\001\000\000\000\b\000\000\000\001\000\000"

l_anon.2cd7e48f040dfd2430fbbf3246836027.31:
	.ascii	"prefer accuracy"

l_anon.2cd7e48f040dfd2430fbbf3246836027.32:
	.ascii	"Three preferences, three different arms, from one table. The\n"

l_anon.2cd7e48f040dfd2430fbbf3246836027.33:
	.ascii	"preference is the whole content of the strategy at this layer, and\n"

l_anon.2cd7e48f040dfd2430fbbf3246836027.34:
	.ascii	"the arm is derived rather than named.\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.2cd7e48f040dfd2430fbbf3246836027.35:
	.asciz	"\023  direct_wrap    = \300\001\n"

l_anon.2cd7e48f040dfd2430fbbf3246836027.36:
	.asciz	"\023  pref_speed     = \300\030   same as direct_wrap: \300\001\n"

l_anon.2cd7e48f040dfd2430fbbf3246836027.37:
	.asciz	"\023  direct_widen   = \300\001\n"

l_anon.2cd7e48f040dfd2430fbbf3246836027.38:
	.asciz	"\023  pref_accuracy  = \300\031   same as direct_widen: \300\001\n"

	.section	__TEXT,__const
l_anon.2cd7e48f040dfd2430fbbf3246836027.39:
	.asciz	"\002  \303 \000\000\b\020\000\t weights \300\004 -> \303 \000\000\b\t\000\002 [\300\002]\n"

l_anon.2cd7e48f040dfd2430fbbf3246836027.40:
	.asciz	"\002  \303 \000\000\b\020\000\001 \303 \000\000(\006\000\001 \303 \000\000(\006\000\001 \303 \000\000(\006\000\001\n"

l_anon.2cd7e48f040dfd2430fbbf3246836027.41:
	.ascii	"mid > len"

.subsections_via_symbols
