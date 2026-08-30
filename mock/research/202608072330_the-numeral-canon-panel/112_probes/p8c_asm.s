	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECshVuqdzI1OeQ_32p8c_the_per_node_discharge_check
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECshVuqdzI1OeQ_32p8c_the_per_node_discharge_check
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECshVuqdzI1OeQ_32p8c_the_per_node_discharge_check:
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
	adrp	x1, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.0@PAGE
Lloh1:
	add	x1, x1, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.0@PAGEOFF
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
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECshVuqdzI1OeQ_32p8c_the_per_node_discharge_check:
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
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0CshVuqdzI1OeQ_32p8c_the_per_node_discharge_check:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECshVuqdzI1OeQ_32p8c_the_per_node_discharge_check
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCshVuqdzI1OeQ_32p8c_the_per_node_discharge_check:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECshVuqdzI1OeQ_32p8c_the_per_node_discharge_check
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__RNvCshVuqdzI1OeQ_32p8c_the_per_node_discharge_check4main
	.globl	__RNvCshVuqdzI1OeQ_32p8c_the_per_node_discharge_check4main
	.p2align	2
__RNvCshVuqdzI1OeQ_32p8c_the_per_node_discharge_check4main:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	sub	sp, sp, #192
	.cfi_def_cfa_offset 192
	stp	x28, x27, [sp, #96]
	stp	x26, x25, [sp, #112]
	stp	x24, x23, [sp, #128]
	stp	x22, x21, [sp, #144]
	stp	x20, x19, [sp, #160]
	stp	x29, x30, [sp, #176]
	add	x29, sp, #176
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
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.1@PAGE
Lloh3:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.1@PAGEOFF
	mov	w1, #131
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	bl	__RNvCske4UNIzLImn_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w0, #70
	mov	w1, #1
	bl	__RNvCske4UNIzLImn_7___rustc12___rust_alloc
	cbz	x0, LBB4_8
	mov	x8, #4340410370284600380
	orr	x8, x8, #0x1111111111111111
	stp	x8, x8, [x0]
	ldr	q0, [x0]
	stp	q0, q0, [x0, #16]
	str	q0, [x0, #48]
	mov	w8, #15677
	strh	w8, [x0, #68]
	mov	w8, #15677
	movk	w8, #15677, lsl #16
	str	w8, [x0, #64]
	mov	w8, #70
	stp	x8, x0, [sp, #16]
	str	x8, [sp, #32]
	add	x8, sp, #16
Lloh4:
	adrp	x9, __RNvXsq_NtCseduYQEDYcHM_5alloc6stringNtB5_6StringNtNtCs5dyeT9KiOLK_4core3fmt7Display3fmt@PAGE
Lloh5:
	add	x9, x9, __RNvXsq_NtCseduYQEDYcHM_5alloc6stringNtB5_6StringNtNtCs5dyeT9KiOLK_4core3fmt7Display3fmt@PAGEOFF
	stp	x8, x9, [sp]
Ltmp0:
Lloh6:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.2@PAGE
Lloh7:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.2@PAGEOFF
	mov	x1, sp
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Ltmp1:
	ldr	x1, [sp, #16]
	cbz	x1, LBB4_4
	ldr	x0, [sp, #24]
	mov	w2, #1
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
LBB4_4:
Lloh8:
	adrp	x19, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.3@PAGE
Lloh9:
	add	x19, x19, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.3@PAGEOFF
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh10:
	adrp	x22, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.4@PAGE
Lloh11:
	add	x22, x22, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.4@PAGEOFF
Lloh12:
	adrp	x21, __RNvXs9_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3implNtB9_7Display3fmt@GOTPAGE
Lloh13:
	ldr	x21, [x21, __RNvXs9_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3implNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x22, x21, [sp, #16]
Lloh14:
	adrp	x8, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.5@PAGE
Lloh15:
	add	x8, x8, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.5@PAGEOFF
	stp	x8, x21, [sp, #32]
Lloh16:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.6@PAGE
Lloh17:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.6@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh18:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.7@PAGE
Lloh19:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.7@PAGEOFF
	mov	w1, #77
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	stp	x22, x21, [sp, #16]
Lloh20:
	adrp	x20, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.8@PAGE
Lloh21:
	add	x20, x20, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.8@PAGEOFF
	stp	x20, x21, [sp, #32]
Lloh22:
	adrp	x23, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.9@PAGE
Lloh23:
	add	x23, x23, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.9@PAGEOFF
Lloh24:
	adrp	x24, __RNvXsg_NtCs5dyeT9KiOLK_4core3fmtbNtB5_7Display3fmt@GOTPAGE
Lloh25:
	ldr	x24, [x24, __RNvXsg_NtCs5dyeT9KiOLK_4core3fmtbNtB5_7Display3fmt@GOTPAGEOFF]
	stp	x23, x24, [sp, #48]
	stp	x23, x24, [sp, #64]
Lloh26:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.10@PAGE
Lloh27:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.10@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	stp	x22, x21, [sp, #16]
Lloh28:
	adrp	x8, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.11@PAGE
Lloh29:
	add	x8, x8, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.11@PAGEOFF
	stp	x8, x21, [sp, #32]
Lloh30:
	adrp	x25, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.12@PAGE
Lloh31:
	add	x25, x25, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.12@PAGEOFF
	stp	x25, x24, [sp, #48]
	stp	x23, x24, [sp, #64]
Lloh32:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.13@PAGE
Lloh33:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.13@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	strb	wzr, [sp]
Lloh34:
	adrp	x8, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.14@PAGE
Lloh35:
	add	x8, x8, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.14@PAGEOFF
	stp	x8, x21, [sp, #16]
	stp	x20, x21, [sp, #32]
	mov	x26, sp
	stp	x26, x24, [sp, #48]
Lloh36:
	adrp	x20, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.15@PAGE
Lloh37:
	add	x20, x20, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.15@PAGEOFF
	add	x1, sp, #16
	mov	x0, x20
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh38:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.16@PAGE
Lloh39:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.16@PAGEOFF
	mov	w1, #141
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh40:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.17@PAGE
Lloh41:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.17@PAGEOFF
	mov	w1, #147
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh42:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.18@PAGE
Lloh43:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.18@PAGEOFF
	mov	w1, #79
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	stp	x22, x21, [sp, #16]
Lloh44:
	adrp	x28, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.19@PAGE
Lloh45:
	add	x28, x28, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.19@PAGEOFF
	stp	x28, x21, [sp, #32]
	stp	x25, x24, [sp, #48]
Lloh46:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.20@PAGE
Lloh47:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.20@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	stp	x22, x21, [sp, #16]
Lloh48:
	adrp	x27, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.21@PAGE
Lloh49:
	add	x27, x27, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.21@PAGEOFF
	stp	x27, x21, [sp, #32]
	stp	x25, x24, [sp, #48]
Lloh50:
	adrp	x8, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.22@PAGE
Lloh51:
	add	x8, x8, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.22@PAGEOFF
	stp	x8, x21, [sp, #64]
Lloh52:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.23@PAGE
Lloh53:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.23@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	strb	wzr, [sp]
Lloh54:
	adrp	x8, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.24@PAGE
Lloh55:
	add	x8, x8, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.24@PAGEOFF
	stp	x8, x21, [sp, #16]
	stp	x28, x21, [sp, #32]
	stp	x26, x24, [sp, #48]
	add	x1, sp, #16
	mov	x0, x20
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh56:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.25@PAGE
Lloh57:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.25@PAGEOFF
	mov	w1, #141
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh58:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.26@PAGE
Lloh59:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.26@PAGEOFF
	mov	w1, #139
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh60:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.27@PAGE
Lloh61:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.27@PAGEOFF
	mov	w1, #139
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh62:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.28@PAGE
Lloh63:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.28@PAGEOFF
	mov	w1, #139
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	stp	x22, x21, [sp, #16]
	stp	x27, x21, [sp, #32]
	stp	x23, x24, [sp, #48]
Lloh64:
	adrp	x8, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.29@PAGE
Lloh65:
	add	x8, x8, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.29@PAGEOFF
Lloh66:
	adrp	x20, __RNvXsi_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impjNtB9_7Display3fmt@GOTPAGE
Lloh67:
	ldr	x20, [x20, __RNvXsi_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impjNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x8, x20, [sp, #64]
Lloh68:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.30@PAGE
Lloh69:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.30@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh70:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.31@PAGE
Lloh71:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.31@PAGEOFF
	mov	w1, #143
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh72:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.32@PAGE
Lloh73:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.32@PAGEOFF
	mov	w1, #145
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w19, #0
	mov	x21, #0
	mov	x23, #0
	mov	x24, #0
LBB4_5:
	mov	x0, x19
	mov	w1, #0
	mov	w2, #0
	bl	_affine_gated_diff
	and	w22, w19, #0xff
	cmp	w22, w0, uxtb
	cinc	x23, x23, ne
	mov	x0, x19
	mov	w1, #0
	mov	w2, #0
	bl	_corner_gated_diff
	cmp	w22, w0, uxtb
	cinc	x24, x24, ne
	mov	x0, x19
	mov	w1, #1
	mov	w2, #1
	bl	_affine_gated_diff
	cmp	w22, w0, uxtb
	cinc	x23, x23, ne
	mov	x0, x19
	mov	w1, #1
	mov	w2, #1
	bl	_corner_gated_diff
	cmp	w22, w0, uxtb
	cinc	x24, x24, ne
	mov	x0, x19
	mov	w1, #2
	mov	w2, #2
	bl	_affine_gated_diff
	cmp	w22, w0, uxtb
	cinc	x23, x23, ne
	mov	x0, x19
	mov	w1, #2
	mov	w2, #2
	bl	_corner_gated_diff
	cmp	w22, w0, uxtb
	cinc	x24, x24, ne
	mov	x0, x19
	mov	w1, #3
	mov	w2, #3
	bl	_affine_gated_diff
	cmp	w22, w0, uxtb
	cinc	x23, x23, ne
	mov	x0, x19
	mov	w1, #3
	mov	w2, #3
	bl	_corner_gated_diff
	cmp	w22, w0, uxtb
	cinc	x24, x24, ne
	mov	x0, x19
	mov	w1, #4
	mov	w2, #4
	bl	_affine_gated_diff
	cmp	w22, w0, uxtb
	cinc	x23, x23, ne
	mov	x0, x19
	mov	w1, #4
	mov	w2, #4
	bl	_corner_gated_diff
	cmp	w22, w0, uxtb
	cinc	x24, x24, ne
	mov	x0, x19
	mov	w1, #5
	mov	w2, #5
	bl	_affine_gated_diff
	cmp	w22, w0, uxtb
	cinc	x23, x23, ne
	mov	x0, x19
	mov	w1, #5
	mov	w2, #5
	bl	_corner_gated_diff
	cmp	w22, w0, uxtb
	cinc	x24, x24, ne
	add	x21, x21, #7
	mov	x0, x19
	mov	w1, #6
	mov	w2, #6
	bl	_affine_gated_diff
	cmp	w22, w0, uxtb
	cinc	x23, x23, ne
	mov	x0, x19
	mov	w1, #6
	mov	w2, #6
	bl	_corner_gated_diff
	cmp	w22, w0, uxtb
	cinc	x24, x24, ne
	cmp	w22, #6
	csinc	w19, w22, w19, eq
	b.eq	LBB4_7
	and	w8, w19, #0xff
	cmp	w8, #7
	b.lo	LBB4_5
LBB4_7:
	str	x21, [sp]
	stp	x23, x24, [sp, #80]
	add	x8, sp, #80
	stp	x8, x20, [sp, #16]
	mov	x19, sp
	stp	x19, x20, [sp, #32]
Lloh74:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.33@PAGE
Lloh75:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.33@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, sp, #88
	stp	x8, x20, [sp, #16]
	stp	x19, x20, [sp, #32]
Lloh76:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.34@PAGE
Lloh77:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.34@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh78:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.35@PAGE
Lloh79:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.35@PAGEOFF
	mov	w1, #135
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh80:
	adrp	x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.36@PAGE
Lloh81:
	add	x0, x0, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.36@PAGEOFF
	mov	w1, #133
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	.cfi_def_cfa wsp, 192
	ldp	x29, x30, [sp, #176]
	ldp	x20, x19, [sp, #160]
	ldp	x22, x21, [sp, #144]
	ldp	x24, x23, [sp, #128]
	ldp	x26, x25, [sp, #112]
	ldp	x28, x27, [sp, #96]
	add	sp, sp, #192
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
LBB4_8:
	.cfi_restore_state
	mov	w0, #1
	mov	w1, #70
	bl	__RNvNtCseduYQEDYcHM_5alloc7raw_vec12handle_error
LBB4_9:
Ltmp2:
	mov	x19, x0
	ldr	x1, [sp, #16]
	cbz	x1, LBB4_11
	ldr	x0, [sp, #24]
	mov	w2, #1
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
LBB4_11:
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh72, Lloh73
	.loh AdrpAdd	Lloh70, Lloh71
	.loh AdrpAdd	Lloh68, Lloh69
	.loh AdrpLdrGot	Lloh66, Lloh67
	.loh AdrpAdd	Lloh64, Lloh65
	.loh AdrpAdd	Lloh62, Lloh63
	.loh AdrpAdd	Lloh60, Lloh61
	.loh AdrpAdd	Lloh58, Lloh59
	.loh AdrpAdd	Lloh56, Lloh57
	.loh AdrpAdd	Lloh54, Lloh55
	.loh AdrpAdd	Lloh52, Lloh53
	.loh AdrpAdd	Lloh50, Lloh51
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpAdd	Lloh46, Lloh47
	.loh AdrpAdd	Lloh44, Lloh45
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpAdd	Lloh38, Lloh39
	.loh AdrpAdd	Lloh36, Lloh37
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpLdrGot	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpLdrGot	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh80, Lloh81
	.loh AdrpAdd	Lloh78, Lloh79
	.loh AdrpAdd	Lloh76, Lloh77
	.loh AdrpAdd	Lloh74, Lloh75
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table4:
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
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp1
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__RNvXsq_NtCseduYQEDYcHM_5alloc6stringNtB5_6StringNtNtCs5dyeT9KiOLK_4core3fmt7Display3fmt:
	.cfi_startproc
	mov	x2, x1
	ldp	x8, x1, [x0, #8]
	mov	x0, x8
	b	__RNvXsi_NtCs5dyeT9KiOLK_4core3fmteNtB5_7Display3fmt
	.cfi_endproc

	.globl	_affine_gated_diff
	.p2align	2
_affine_gated_diff:
	.cfi_startproc
	add	w8, w1, w0
	sub	w0, w8, w2
	ret
	.cfi_endproc

	.globl	_corner_gated_diff
	.p2align	2
_corner_gated_diff:
	.cfi_startproc
	and	w8, w0, #0xff
	add	w8, w8, w1, uxtb
	mov	w9, #255
	cmp	w8, #255
	csel	w8, w8, w9, lo
	subs	w8, w8, w2, uxtb
	csel	w0, wzr, w8, lo
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
Lloh82:
	adrp	x8, __RNvCshVuqdzI1OeQ_32p8c_the_per_node_discharge_check4main@PAGE
Lloh83:
	add	x8, x8, __RNvCshVuqdzI1OeQ_32p8c_the_per_node_discharge_check4main@PAGEOFF
	str	x8, [sp, #8]
Lloh84:
	adrp	x1, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.0@PAGE
Lloh85:
	add	x1, x1, l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.0@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh84, Lloh85
	.loh AdrpAdd	Lloh82, Lloh83
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCshVuqdzI1OeQ_32p8c_the_per_node_discharge_check
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0CshVuqdzI1OeQ_32p8c_the_per_node_discharge_check
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0CshVuqdzI1OeQ_32p8c_the_per_node_discharge_check

	.section	__TEXT,__const
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.1:
	.ascii	"p8c. The per-node discharge check, recursing over the grade type\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.2:
	.asciz	"\300\001\n"

	.section	__TEXT,__const
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.3:
	.byte	10

	.section	__TEXT,__literal4,4byte_literals
	.p2align	2, 0x0
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.4:
	.space	4

	.p2align	2, 0x0
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.5:
	.asciz	"\017\000\000"

	.section	__TEXT,__cstring,cstring_literals
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.6:
	.asciz	"\016  container: [\300\002, \300\002]\n"

	.section	__TEXT,__const
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.7:
	.ascii	"  WIDE declaration, x and y in 0..=14\n"

	.section	__TEXT,__literal4,4byte_literals
	.p2align	2, 0x0
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.8:
	.asciz	"\034\000\000"

	.section	__TEXT,__cstring,cstring_literals
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.9:
	.space	1

	.section	__TEXT,__const
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.10:
	.asciz	"\031    x + y            -> [\303 \000\000(\003\000\002, \303 \000\000(\003\000\016]   root-only \300\f   per-node \300\001\n"

	.section	__TEXT,__literal4,4byte_literals
	.p2align	2, 0x0
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.11:
	.asciz	"\016\000\000"

	.section	__TEXT,__const
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.12:
	.byte	1

l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.13:
	.asciz	"\031    (x + y) - y      -> [\303 \000\000(\003\000\002, \303 \000\000(\003\000\016]   root-only \300\f   per-node \300\001\n"

	.section	__TEXT,__literal4,4byte_literals
	.p2align	2, 0x0
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.14:
	.ascii	"\362\377\377\377"

	.section	__TEXT,__const
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.15:
	.asciz	"\031    corner, same term-> [\303 \000\000(\003\000\002, \303 \000\000(\003\000\016]   licenses  \300\001\n"

l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.16:
	.ascii	"    p8b licensed this term. The per-node check refuses it, correctly,\n"

l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.17:
	.ascii	"    because the intermediate x + y reaches 28 and the range stops at 15.\n"

l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.18:
	.ascii	"  NARROW declaration, x and y in 0..=6\n"

	.section	__TEXT,__literal4,4byte_literals
	.p2align	2, 0x0
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.19:
	.asciz	"\f\000\000"

	.section	__TEXT,__const
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.20:
	.asciz	"\031    x + y            -> [\303 \000\000(\003\000\002, \303 \000\000(\003\000\r]   per-node \300\001\n"

	.section	__TEXT,__literal4,4byte_literals
	.p2align	2, 0x0
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.21:
	.asciz	"\006\000\000"

	.p2align	2, 0x0
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.22:
	.asciz	"\003\000\000"

	.section	__TEXT,__const
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.23:
	.asciz	"\031    (x + y) - y      -> [\303 \000\000(\003\000\002, \303 \000\000(\003\000\r]   per-node \300\n   radius \300\001\n"

	.section	__TEXT,__literal4,4byte_literals
	.p2align	2, 0x0
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.24:
	.ascii	"\372\377\377\377"

	.section	__TEXT,__const
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.25:
	.ascii	"    the affine advantage survives the per-node discipline: the affine\n"

l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.26:
	.ascii	"    root cancels y and lands in [0, 6]; the corner root keeps both y\n"

l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.27:
	.ascii	"    occurrences and lands in [-6, 12], whose lower bound is outside.\n"

l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.28:
	.ascii	"  a deeper composition at the narrow declaration, ((x+y)+x) - (x+y):\n"

	.section	__TEXT,__literal8,8byte_literals
	.p2align	3, 0x0
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.29:
	.asciz	"\002\000\000\000\000\000\000"

	.section	__TEXT,__const
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.30:
	.asciz	"\031    affine           -> [\303 \000\000(\003\000\002, \303 \000\000(\003\000\r]   per-node \300\021   vector length \300\001\n"

l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.31:
	.ascii	"    (the exact value of that term is x, so [0, 6] is the tight answer)\n"

l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.32:
	.ascii	"  BEHAVIOUR at the narrow declaration, on the declared term with c = b:\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.33:
	.asciz	"+    affine-gated arm differs from exact on \300\001/\300\001\n"

l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.34:
	.asciz	"+    corner-gated arm differs from exact on \300\001/\300\001\n"

	.section	__TEXT,__const
l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.35:
	.ascii	"    (both are 0: the corner arm is correct too, it is just slower,\n"

l_anon.13f2af87d4cd55c6ed657be95ec4ea4e.36:
	.ascii	"     because it refused the licence and took the saturating path)\n"

	.globl	_general_diff
_general_diff = _corner_gated_diff
	.globl	_bare_diff
_bare_diff = _affine_gated_diff
.subsections_via_symbols
