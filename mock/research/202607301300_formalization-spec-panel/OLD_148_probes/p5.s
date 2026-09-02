	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm12loop_by_hand
	.p2align	2
__RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm12loop_by_hand:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	cbz	x1, LBB0_8
	lsl	x10, x1, #2
	sub	x8, x10, #4
	lsr	x8, x8, #2
	cmp	x3, x8
	csel	x8, x3, x8, lo
	cmp	x8, #16
	b.hs	LBB0_3
	mov	x8, #0
	mov	x9, x0
	b	LBB0_5
LBB0_3:
	add	x8, x8, #1
	ands	x9, x8, #0xf
	mov	w11, #16
	csel	x9, x11, x9, eq
	sub	x8, x8, x9
	add	x9, x0, x8, lsl #2
	add	x11, x2, #32
	add	x12, x0, #32
	mov	x13, x8
LBB0_4:
	ldp	q0, q1, [x12, #-32]
	ldp	q2, q3, [x12], #64
	shl.4s	v0, v0, #5
	shl.4s	v1, v1, #5
	shl.4s	v2, v2, #5
	shl.4s	v3, v3, #5
	stp	q0, q1, [x11, #-32]
	stp	q2, q3, [x11], #64
	subs	x13, x13, #16
	b.ne	LBB0_4
LBB0_5:
	add	x10, x0, x10
LBB0_6:
	cmp	x3, x8
	b.eq	LBB0_9
	ldr	w11, [x9], #4
	lsl	w11, w11, #5
	str	w11, [x2, x8, lsl #2]
	add	x8, x8, #1
	cmp	x9, x10
	b.ne	LBB0_6
LBB0_8:
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB0_9:
	.cfi_restore_state
Lloh0:
	adrp	x2, l_anon.da06694469f19e69c3a48d0e5e2df7e9.1@PAGE
Lloh1:
	add	x2, x2, l_anon.da06694469f19e69c3a48d0e5e2df7e9.1@PAGEOFF
	mov	x0, x3
	mov	x1, x3
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh0, Lloh1
	.cfi_endproc

	.globl	__RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm13loop_via_from
	.p2align	2
__RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm13loop_via_from:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	cbz	x1, LBB1_8
	lsl	x10, x1, #2
	sub	x8, x10, #4
	lsr	x8, x8, #2
	cmp	x3, x8
	csel	x8, x3, x8, lo
	cmp	x8, #16
	b.hs	LBB1_3
	mov	x8, #0
	mov	x9, x0
	b	LBB1_5
LBB1_3:
	add	x8, x8, #1
	ands	x9, x8, #0xf
	mov	w11, #16
	csel	x9, x11, x9, eq
	sub	x8, x8, x9
	add	x9, x0, x8, lsl #2
	add	x11, x2, #32
	add	x12, x0, #32
	mov	x13, x8
LBB1_4:
	ldp	q0, q1, [x12, #-32]
	ldp	q2, q3, [x12], #64
	shl.4s	v0, v0, #5
	shl.4s	v1, v1, #5
	shl.4s	v2, v2, #5
	shl.4s	v3, v3, #5
	stp	q0, q1, [x11, #-32]
	stp	q2, q3, [x11], #64
	subs	x13, x13, #16
	b.ne	LBB1_4
LBB1_5:
	add	x10, x0, x10
LBB1_6:
	cmp	x3, x8
	b.eq	LBB1_9
	ldr	w11, [x9], #4
	lsl	w11, w11, #5
	str	w11, [x2, x8, lsl #2]
	add	x8, x8, #1
	cmp	x9, x10
	b.ne	LBB1_6
LBB1_8:
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB1_9:
	.cfi_restore_state
Lloh2:
	adrp	x2, l_anon.da06694469f19e69c3a48d0e5e2df7e9.2@PAGE
Lloh3:
	add	x2, x2, l_anon.da06694469f19e69c3a48d0e5e2df7e9.2@PAGEOFF
	mov	x0, x3
	mov	x1, x3
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh2, Lloh3
	.cfi_endproc

	.globl	__RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm14loop_via_embed
	.p2align	2
__RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm14loop_via_embed:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	cbz	x1, LBB2_8
	lsl	x10, x1, #2
	sub	x8, x10, #4
	lsr	x8, x8, #2
	cmp	x3, x8
	csel	x8, x3, x8, lo
	cmp	x8, #16
	b.hs	LBB2_3
	mov	x8, #0
	mov	x9, x0
	b	LBB2_5
LBB2_3:
	add	x8, x8, #1
	ands	x9, x8, #0xf
	mov	w11, #16
	csel	x9, x11, x9, eq
	sub	x8, x8, x9
	add	x9, x0, x8, lsl #2
	add	x11, x2, #32
	add	x12, x0, #32
	mov	x13, x8
LBB2_4:
	ldp	q0, q1, [x12, #-32]
	ldp	q2, q3, [x12], #64
	shl.4s	v0, v0, #5
	shl.4s	v1, v1, #5
	shl.4s	v2, v2, #5
	shl.4s	v3, v3, #5
	stp	q0, q1, [x11, #-32]
	stp	q2, q3, [x11], #64
	subs	x13, x13, #16
	b.ne	LBB2_4
LBB2_5:
	add	x10, x0, x10
LBB2_6:
	cmp	x3, x8
	b.eq	LBB2_9
	ldr	w11, [x9], #4
	lsl	w11, w11, #5
	str	w11, [x2, x8, lsl #2]
	add	x8, x8, #1
	cmp	x9, x10
	b.ne	LBB2_6
LBB2_8:
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB2_9:
	.cfi_restore_state
Lloh4:
	adrp	x2, l_anon.da06694469f19e69c3a48d0e5e2df7e9.3@PAGE
Lloh5:
	add	x2, x2, l_anon.da06694469f19e69c3a48d0e5e2df7e9.3@PAGEOFF
	mov	x0, x3
	mov	x1, x3
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh4, Lloh5
	.cfi_endproc

	.globl	__RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm14scalar_by_hand
	.p2align	2
__RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm14scalar_by_hand:
	.cfi_startproc
	lsl	w0, w0, #5
	ret
	.cfi_endproc

	.section	__TEXT,__cstring,cstring_literals
l_anon.da06694469f19e69c3a48d0e5e2df7e9.0:
	.asciz	"p5_erasure_and_by_value_arm.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.da06694469f19e69c3a48d0e5e2df7e9.1:
	.quad	l_anon.da06694469f19e69c3a48d0e5e2df7e9.0
	.asciz	"\036\000\000\000\000\000\000\000\255\000\000\000\t\000\000"

	.p2align	3, 0x0
l_anon.da06694469f19e69c3a48d0e5e2df7e9.2:
	.quad	l_anon.da06694469f19e69c3a48d0e5e2df7e9.0
	.asciz	"\036\000\000\000\000\000\000\000\264\000\000\000\t\000\000"

	.p2align	3, 0x0
l_anon.da06694469f19e69c3a48d0e5e2df7e9.3:
	.quad	l_anon.da06694469f19e69c3a48d0e5e2df7e9.0
	.asciz	"\036\000\000\000\000\000\000\000\273\000\000\000\t\000\000"

	.globl	__RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm15scalar_via_from
__RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm15scalar_via_from = __RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm14scalar_by_hand
	.globl	__RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm16scalar_via_embed
__RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm16scalar_via_embed = __RNvCs7lDxcrvSw32_27p5_erasure_and_by_value_arm14scalar_by_hand
.subsections_via_symbols
