	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch:
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
	adrp	x1, l_anon.1850c20ba08a1e1daade71fd430eecce.0@PAGE
Lloh1:
	add	x1, x1, l_anon.1850c20ba08a1e1daade71fd430eecce.0@PAGEOFF
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
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch:
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
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__RNvCse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch4main
	.globl	__RNvCse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch4main
	.p2align	2
__RNvCse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch4main:
	.cfi_startproc
	sub	sp, sp, #128
	.cfi_def_cfa_offset 128
	stp	x24, x23, [sp, #64]
	stp	x22, x21, [sp, #80]
	stp	x20, x19, [sp, #96]
	stp	x29, x30, [sp, #112]
	add	x29, sp, #112
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	mov	w8, #520
	stp	x8, x8, [sp]
	str	x8, [sp, #16]
	mov	x8, sp
Lloh2:
	adrp	x19, __RNvXsd_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impyNtB9_7Display3fmt@GOTPAGE
Lloh3:
	ldr	x19, [x19, __RNvXsd_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impyNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x8, x19, [sp, #48]
Lloh4:
	adrp	x0, l_anon.1850c20ba08a1e1daade71fd430eecce.1@PAGE
Lloh5:
	add	x0, x0, l_anon.1850c20ba08a1e1daade71fd430eecce.1@PAGEOFF
	add	x1, sp, #48
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, sp, #8
	stp	x8, x19, [sp, #48]
Lloh6:
	adrp	x0, l_anon.1850c20ba08a1e1daade71fd430eecce.2@PAGE
Lloh7:
	add	x0, x0, l_anon.1850c20ba08a1e1daade71fd430eecce.2@PAGEOFF
	add	x1, sp, #48
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, sp, #16
	stp	x8, x19, [sp, #48]
Lloh8:
	adrp	x0, l_anon.1850c20ba08a1e1daade71fd430eecce.3@PAGE
Lloh9:
	add	x0, x0, l_anon.1850c20ba08a1e1daade71fd430eecce.3@PAGEOFF
	add	x1, sp, #48
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	ldp	x8, x9, [sp]
	cmp	x8, x9
	ldr	x8, [sp, #16]
Lloh10:
	adrp	x10, l_anon.1850c20ba08a1e1daade71fd430eecce.4@PAGE
Lloh11:
	add	x10, x10, l_anon.1850c20ba08a1e1daade71fd430eecce.4@PAGEOFF
Lloh12:
	adrp	x20, l_anon.1850c20ba08a1e1daade71fd430eecce.5@PAGE
Lloh13:
	add	x20, x20, l_anon.1850c20ba08a1e1daade71fd430eecce.5@PAGEOFF
	ccmp	x9, x8, #0, eq
	csel	x8, x20, x10, eq
	mov	w9, #21
	mov	w21, #3
	csel	x9, x21, x9, eq
	stp	x8, x9, [sp, #32]
	add	x22, sp, #32
Lloh14:
	adrp	x23, __RNvXs1i_NtCs5dyeT9KiOLK_4core3fmtReNtB6_7Display3fmtCse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch@PAGE
Lloh15:
	add	x23, x23, __RNvXs1i_NtCs5dyeT9KiOLK_4core3fmtReNtB6_7Display3fmtCse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch@PAGEOFF
	stp	x22, x23, [sp, #48]
Lloh16:
	adrp	x0, l_anon.1850c20ba08a1e1daade71fd430eecce.6@PAGE
Lloh17:
	add	x0, x0, l_anon.1850c20ba08a1e1daade71fd430eecce.6@PAGEOFF
	add	x1, sp, #48
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w8, #21000
	str	x8, [sp, #24]
	add	x8, sp, #24
	stp	x8, x19, [sp, #48]
Lloh18:
	adrp	x0, l_anon.1850c20ba08a1e1daade71fd430eecce.7@PAGE
Lloh19:
	add	x0, x0, l_anon.1850c20ba08a1e1daade71fd430eecce.7@PAGEOFF
	add	x1, sp, #48
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	ldr	x8, [sp, #24]
	ldr	x9, [sp, #8]
Lloh20:
	adrp	x10, l_anon.1850c20ba08a1e1daade71fd430eecce.8@PAGE
Lloh21:
	add	x10, x10, l_anon.1850c20ba08a1e1daade71fd430eecce.8@PAGEOFF
	cmp	x8, x9
	csel	x8, x10, x20, eq
	mov	w9, #22
	csel	x9, x9, x21, eq
	stp	x8, x9, [sp, #32]
	stp	x22, x23, [sp, #48]
Lloh22:
	adrp	x0, l_anon.1850c20ba08a1e1daade71fd430eecce.9@PAGE
Lloh23:
	add	x0, x0, l_anon.1850c20ba08a1e1daade71fd430eecce.9@PAGEOFF
	add	x1, sp, #48
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh24:
	adrp	x0, l_anon.1850c20ba08a1e1daade71fd430eecce.10@PAGE
Lloh25:
	add	x0, x0, l_anon.1850c20ba08a1e1daade71fd430eecce.10@PAGEOFF
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh26:
	adrp	x0, l_anon.1850c20ba08a1e1daade71fd430eecce.11@PAGE
Lloh27:
	add	x0, x0, l_anon.1850c20ba08a1e1daade71fd430eecce.11@PAGEOFF
	mov	w1, #133
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh28:
	adrp	x0, l_anon.1850c20ba08a1e1daade71fd430eecce.12@PAGE
Lloh29:
	add	x0, x0, l_anon.1850c20ba08a1e1daade71fd430eecce.12@PAGEOFF
	mov	w1, #135
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh30:
	adrp	x0, l_anon.1850c20ba08a1e1daade71fd430eecce.13@PAGE
Lloh31:
	add	x0, x0, l_anon.1850c20ba08a1e1daade71fd430eecce.13@PAGEOFF
	mov	w1, #135
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh32:
	adrp	x0, l_anon.1850c20ba08a1e1daade71fd430eecce.14@PAGE
Lloh33:
	add	x0, x0, l_anon.1850c20ba08a1e1daade71fd430eecce.14@PAGEOFF
	mov	w1, #125
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	.cfi_def_cfa wsp, 128
	ldp	x29, x30, [sp, #112]
	ldp	x20, x19, [sp, #96]
	ldp	x22, x21, [sp, #80]
	ldp	x24, x23, [sp, #64]
	add	sp, sp, #128
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	ret
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
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
	.loh AdrpLdrGot	Lloh2, Lloh3
	.cfi_endproc

	.p2align	2
__RNvXs1i_NtCs5dyeT9KiOLK_4core3fmtReNtB6_7Display3fmtCse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch:
	.cfi_startproc
	mov	x2, x1
	ldp	x8, x1, [x0]
	mov	x0, x8
	b	__RNvXsi_NtCs5dyeT9KiOLK_4core3fmteNtB5_7Display3fmt
	.cfi_endproc

	.globl	_fold_const
	.p2align	2
_fold_const:
	.cfi_startproc
	mov	x8, #0
	cbz	x1, LBB6_2
LBB6_1:
	ldr	w9, [x0], #8
	add	w8, w9, w8
	and	x8, x8, #0xfff
	subs	x1, x1, #1
	b.ne	LBB6_1
LBB6_2:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_fold_direct
	.p2align	2
_fold_direct:
	.cfi_startproc
	mov	x8, #0
	cbz	x1, LBB7_2
LBB7_1:
	ldr	w9, [x0], #8
	add	w8, w9, w8
	and	x8, x8, #0xfff
	subs	x1, x1, #1
	b.ne	LBB7_1
LBB7_2:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_fold_runtime
	.p2align	2
_fold_runtime:
	.cfi_startproc
	ldp	w8, w9, [x2]
	ldr	w10, [x2, #8]
	add	w11, w10, w10, lsl #3
	add	w12, w9, w8
	add	w11, w12, w11
	add	w12, w8, w8, lsl #1
	add	w12, w9, w12
	add	w10, w12, w10, lsl #2
	cmp	w10, w11
	csel	w10, w10, w11, lo
	cset	w11, lo
	add	w9, w9, w9, lsl #1
	sub	w9, w9, w8
	add	w8, w9, w8, lsl #3
	cmp	w8, w10
	mov	w8, #2
	csel	x8, x8, x11, lo
	cbz	x8, LBB8_5
	cmp	x8, #1
	b.ne	LBB8_7
	mov	x8, #0
	cbz	x1, LBB8_16
	mov	w9, #4095
LBB8_4:
	ldr	x10, [x0], #8
	add	x8, x10, x8
	cmp	x8, #4095
	csel	x8, x8, x9, lo
	subs	x1, x1, #1
	b.ne	LBB8_4
	b	LBB8_16
LBB8_5:
	mov	x8, #0
	cbz	x1, LBB8_16
LBB8_6:
	ldr	w9, [x0], #8
	add	w8, w9, w8
	and	x8, x8, #0xfff
	subs	x1, x1, #1
	b.ne	LBB8_6
	b	LBB8_16
LBB8_7:
	cbz	x1, LBB8_10
	cmp	x1, #8
	b.hs	LBB8_11
	mov	x8, #0
	mov	x9, #0
	b	LBB8_14
LBB8_10:
	mov	x0, #0
	ret
LBB8_11:
	and	x9, x1, #0xffffffffffffff8
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	and	x10, x1, #0xffffffffffffff8
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB8_12:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	subs	x10, x10, #8
	b.ne	LBB8_12
	add.2d	v0, v1, v0
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x1, x9
	b.eq	LBB8_16
LBB8_14:
	add	x10, x0, x9, lsl #3
	sub	x9, x1, x9
LBB8_15:
	ldr	x11, [x10], #8
	add	x8, x11, x8
	subs	x9, x9, #1
	b.ne	LBB8_15
LBB8_16:
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
Lloh34:
	adrp	x8, __RNvCse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch4main@PAGE
Lloh35:
	add	x8, x8, __RNvCse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch4main@PAGEOFF
	str	x8, [sp, #8]
Lloh36:
	adrp	x1, l_anon.1850c20ba08a1e1daade71fd430eecce.0@PAGE
Lloh37:
	add	x1, x1, l_anon.1850c20ba08a1e1daade71fd430eecce.0@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh36, Lloh37
	.loh AdrpAdd	Lloh34, Lloh35
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.1850c20ba08a1e1daade71fd430eecce.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cse9yKKSDdgpF_42c1_a_runtime_selection_must_leave_a_branch

	.section	__TEXT,__cstring,cstring_literals
l_anon.1850c20ba08a1e1daade71fd430eecce.1:
	.asciz	"\017fold_const   = \300\001\n"

l_anon.1850c20ba08a1e1daade71fd430eecce.2:
	.asciz	"\017fold_runtime = \300(  (same weighting, arriving at runtime)\n"

l_anon.1850c20ba08a1e1daade71fd430eecce.3:
	.asciz	"\017fold_direct  = \300\032  (the arm, hand written)\n"

	.section	__TEXT,__const
l_anon.1850c20ba08a1e1daade71fd430eecce.4:
	.ascii	"NO, the premise fails"

l_anon.1850c20ba08a1e1daade71fd430eecce.5:
	.ascii	"yes"

	.section	__TEXT,__cstring,cstring_literals
l_anon.1850c20ba08a1e1daade71fd430eecce.6:
	.asciz	"\016values agree: \300\001\n"

l_anon.1850c20ba08a1e1daade71fd430eecce.7:
	.asciz	"2fold_runtime at a weighting preferring accuracy = \300\001\n"

	.section	__TEXT,__const
l_anon.1850c20ba08a1e1daade71fd430eecce.8:
	.ascii	"NO, the model is inert"

	.section	__TEXT,__cstring,cstring_literals
l_anon.1850c20ba08a1e1daade71fd430eecce.9:
	.asciz	"\027the weighting selects: \300\001\n"

	.section	__TEXT,__const
l_anon.1850c20ba08a1e1daade71fd430eecce.10:
	.byte	10

l_anon.1850c20ba08a1e1daade71fd430eecce.11:
	.ascii	"The verdict is in the assembly, not here. Count conditional jumps\n"

l_anon.1850c20ba08a1e1daade71fd430eecce.12:
	.ascii	"in `fold_const` and in `fold_runtime`: the control passes when the\n"

l_anon.1850c20ba08a1e1daade71fd430eecce.13:
	.ascii	"first has none and the second has them, and fails when they match,\n"

l_anon.1850c20ba08a1e1daade71fd430eecce.14:
	.ascii	"because then the comparison cannot tell the two apart at all.\n"

.subsections_via_symbols
