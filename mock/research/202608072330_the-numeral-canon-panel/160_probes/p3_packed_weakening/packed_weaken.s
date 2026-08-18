	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECse2wnar8CLDW_13packed_weaken
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECse2wnar8CLDW_13packed_weaken
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECse2wnar8CLDW_13packed_weaken:
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
	adrp	x1, l_anon.c0874902f19a73c72ad4a36e743b681d.0@PAGE
Lloh1:
	add	x1, x1, l_anon.c0874902f19a73c72ad4a36e743b681d.0@PAGEOFF
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
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECse2wnar8CLDW_13packed_weaken:
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
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cse2wnar8CLDW_13packed_weaken:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECse2wnar8CLDW_13packed_weaken
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCse2wnar8CLDW_13packed_weaken:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECse2wnar8CLDW_13packed_weaken
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__RNvCse2wnar8CLDW_13packed_weaken4main
	.globl	__RNvCse2wnar8CLDW_13packed_weaken4main
	.p2align	2
__RNvCse2wnar8CLDW_13packed_weaken4main:
	.cfi_startproc
	sub	sp, sp, #176
	.cfi_def_cfa_offset 176
	stp	x29, x30, [sp, #160]
	add	x29, sp, #160
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	str	xzr, [sp, #96]
	movi.2d	v0, #0000000000000000
	stp	q0, q0, [sp, #64]
	stp	q0, q0, [sp, #32]
	mov	x12, sp
	stp	q0, q0, [sp]
	b	LBB4_2
LBB4_1:
	add	x10, x10, #13
	sub	x9, x9, #13
	add	x8, x8, #3
	add	x11, x11, #1
	cmp	x10, #832
	b.eq	LBB4_5
LBB4_2:
	sub	x13, x8, #101
	cmp	x11, #34
	csel	x13, x8, x13, lo
	lsl	x15, x13, x10
	lsr	x14, x10, #6
	add	x14, x12, x14, lsl #3
	ldr	x16, [x14]
	orr	x15, x16, x15
	str	x15, [x14]
	and	x15, x10, #0x3c
	cmp	x15, #51
	b.ls	LBB4_1
	cmp	x11, #60
	b.hs	LBB4_11
	lsr	x13, x13, x9
	ldr	x15, [x14, #8]
	orr	x13, x15, x13
	str	x13, [x14, #8]
	b	LBB4_1
LBB4_5:
	mov	x8, #0
	mov	x9, #0
	b	LBB4_7
LBB4_6:
	add	x9, x9, #1
	add	x8, x8, #13
	cmp	x9, #64
	b.eq	LBB4_10
LBB4_7:
	and	x10, x8, #0x3c
	cmp	x10, #52
	b.lo	LBB4_6
	cmp	x9, #60
	b.lo	LBB4_6
Lloh2:
	adrp	x2, l_anon.c0874902f19a73c72ad4a36e743b681d.6@PAGE
Lloh3:
	add	x2, x2, l_anon.c0874902f19a73c72ad4a36e743b681d.6@PAGEOFF
	mov	w0, #13
	mov	w1, #13
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
LBB4_10:
	stur	wzr, [x29, #-44]
Lloh4:
	adrp	x8, l_anon.c0874902f19a73c72ad4a36e743b681d.1@PAGE
Lloh5:
	add	x8, x8, l_anon.c0874902f19a73c72ad4a36e743b681d.1@PAGEOFF
Lloh6:
	adrp	x9, __RNvXsi_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impjNtB9_7Display3fmt@GOTPAGE
Lloh7:
	ldr	x9, [x9, __RNvXsi_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impjNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x8, x9, [x29, #-40]
	sub	x8, x29, #44
Lloh8:
	adrp	x9, __RNvXs8_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impmNtB9_7Display3fmt@GOTPAGE
Lloh9:
	ldr	x9, [x9, __RNvXs8_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impmNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x8, x9, [x29, #-24]
Lloh10:
	adrp	x0, l_anon.c0874902f19a73c72ad4a36e743b681d.2@PAGE
Lloh11:
	add	x0, x0, l_anon.c0874902f19a73c72ad4a36e743b681d.2@PAGEOFF
	sub	x1, x29, #40
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w8, #1
	sturb	w8, [x29, #-1]
	sub	x8, x29, #1
Lloh12:
	adrp	x9, __RNvXsg_NtCs5dyeT9KiOLK_4core3fmtbNtB5_7Display3fmt@GOTPAGE
Lloh13:
	ldr	x9, [x9, __RNvXsg_NtCs5dyeT9KiOLK_4core3fmtbNtB5_7Display3fmt@GOTPAGEOFF]
	stp	x8, x9, [x29, #-40]
Lloh14:
	adrp	x0, l_anon.c0874902f19a73c72ad4a36e743b681d.3@PAGE
Lloh15:
	add	x0, x0, l_anon.c0874902f19a73c72ad4a36e743b681d.3@PAGEOFF
	sub	x1, x29, #40
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	.cfi_def_cfa wsp, 176
	ldp	x29, x30, [sp, #160]
	add	sp, sp, #176
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB4_11:
	.cfi_restore_state
Lloh16:
	adrp	x2, l_anon.c0874902f19a73c72ad4a36e743b681d.5@PAGE
Lloh17:
	add	x2, x2, l_anon.c0874902f19a73c72ad4a36e743b681d.5@PAGEOFF
	mov	w0, #13
	mov	w1, #13
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpLdrGot	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpLdrGot	Lloh8, Lloh9
	.loh AdrpLdrGot	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh16, Lloh17
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
Lloh18:
	adrp	x8, __RNvCse2wnar8CLDW_13packed_weaken4main@PAGE
Lloh19:
	add	x8, x8, __RNvCse2wnar8CLDW_13packed_weaken4main@PAGEOFF
	str	x8, [sp, #8]
Lloh20:
	adrp	x1, l_anon.c0874902f19a73c72ad4a36e743b681d.0@PAGE
Lloh21:
	add	x1, x1, l_anon.c0874902f19a73c72ad4a36e743b681d.0@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh18, Lloh19
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.c0874902f19a73c72ad4a36e743b681d.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCse2wnar8CLDW_13packed_weaken
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cse2wnar8CLDW_13packed_weaken
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cse2wnar8CLDW_13packed_weaken

	.section	__TEXT,__literal8,8byte_literals
	.p2align	3, 0x0
l_anon.c0874902f19a73c72ad4a36e743b681d.1:
	.asciz	"@\000\000\000\000\000\000"

	.section	__TEXT,__cstring,cstring_literals
l_anon.c0874902f19a73c72ad4a36e743b681d.2:
	.asciz	"\022packed weakening, \300\013 elements: \300\017 disagreements\n"

l_anon.c0874902f19a73c72ad4a36e743b681d.3:
	.asciz	"$same address through the weakening: \300\001\n"

l_anon.c0874902f19a73c72ad4a36e743b681d.4:
	.asciz	"160_probes/p3_packed_weakening/packed_weaken.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.c0874902f19a73c72ad4a36e743b681d.5:
	.quad	l_anon.c0874902f19a73c72ad4a36e743b681d.4
	.asciz	"/\000\000\000\000\000\000\000K\000\000\000\r\000\000"

	.p2align	3, 0x0
l_anon.c0874902f19a73c72ad4a36e743b681d.6:
	.quad	l_anon.c0874902f19a73c72ad4a36e743b681d.4
	.asciz	"/\000\000\000\000\000\000\000+\000\000\000\023\000\000"

.subsections_via_symbols
