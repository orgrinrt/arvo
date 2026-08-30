	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_lowering12sum_run_iter
	.p2align	2
__RNvCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_lowering12sum_run_iter:
	.cfi_startproc
	ldr	x9, [x0, #512]
	mov	w8, #64
	cmp	x9, #64
	csel	x8, x9, x8, lo
	cbz	x9, LBB0_3
	lsl	x10, x8, #3
	sub	x8, x10, #8
	cmp	x8, #56
	b.hs	LBB0_4
	mov	x8, #0
	mov	x9, x0
	b	LBB0_7
LBB0_3:
	mov	x8, #0
	mov	x0, x8
	ret
LBB0_4:
	lsr	x8, x8, #3
	add	x11, x8, #1
	and	x12, x11, #0x3ffffffffffffff8
	add	x9, x0, x12, lsl #3
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	and	x13, x11, #0x3ffffffffffffff8
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB0_5:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	subs	x13, x13, #8
	b.ne	LBB0_5
	add.2d	v0, v1, v0
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x11, x12
	b.eq	LBB0_9
LBB0_7:
	add	x10, x0, x10
LBB0_8:
	ldr	x11, [x9], #8
	add	x8, x11, x8
	cmp	x9, x10
	b.ne	LBB0_8
LBB0_9:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	__RNvCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_lowering15sum_run_clamped
	.p2align	2
__RNvCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_lowering15sum_run_clamped:
	.cfi_startproc
	ldr	x8, [x0, #512]
	mov	w9, #64
	cmp	x8, #64
	csel	x9, x8, x9, lo
	cbz	x8, LBB1_14
	cmp	x8, #8
	b.hs	LBB1_3
	mov	x8, #0
	mov	x10, #0
	b	LBB1_12
LBB1_3:
	and	x10, x9, #0x78
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	cmp	x10, #8
	b.eq	LBB1_11
	ldp	q4, q5, [x0, #64]
	ldp	q6, q7, [x0, #96]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #16
	b.eq	LBB1_11
	ldp	q4, q5, [x0, #128]
	ldp	q6, q7, [x0, #160]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #24
	b.eq	LBB1_11
	ldp	q4, q5, [x0, #192]
	ldp	q6, q7, [x0, #224]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #32
	b.eq	LBB1_11
	ldp	q4, q5, [x0, #256]
	ldp	q6, q7, [x0, #288]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #40
	b.eq	LBB1_11
	ldp	q4, q5, [x0, #320]
	ldp	q6, q7, [x0, #352]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #48
	b.eq	LBB1_11
	ldp	q4, q5, [x0, #384]
	ldp	q6, q7, [x0, #416]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #56
	b.eq	LBB1_11
	ldp	q4, q5, [x0, #448]
	ldp	q6, q7, [x0, #480]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
LBB1_11:
	add.2d	v0, v1, v0
	add.2d	v1, v3, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x9, x10
	b.eq	LBB1_14
LBB1_12:
	sub	x9, x9, x10
	add	x10, x0, x10, lsl #3
LBB1_13:
	ldr	x11, [x10], #8
	add	x8, x11, x8
	subs	x9, x9, #1
	b.ne	LBB1_13
LBB1_14:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	__RNvCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_lowering16sum_run_unproven
	.p2align	2
__RNvCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_lowering16sum_run_unproven:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	ldr	x9, [x0, #512]
	cbz	x9, LBB2_4
	sub	x8, x9, #65
	cmn	x8, #64
	b.lo	LBB2_17
	cmp	x9, #8
	b.hs	LBB2_5
	mov	x8, #0
	mov	x10, #0
	b	LBB2_14
LBB2_4:
	mov	x8, #0
	b	LBB2_16
LBB2_5:
	and	x10, x9, #0x78
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	cmp	x10, #8
	b.eq	LBB2_13
	ldp	q4, q5, [x0, #64]
	ldp	q6, q7, [x0, #96]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #16
	b.eq	LBB2_13
	ldp	q4, q5, [x0, #128]
	ldp	q6, q7, [x0, #160]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #24
	b.eq	LBB2_13
	ldp	q4, q5, [x0, #192]
	ldp	q6, q7, [x0, #224]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #32
	b.eq	LBB2_13
	ldp	q4, q5, [x0, #256]
	ldp	q6, q7, [x0, #288]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #40
	b.eq	LBB2_13
	ldp	q4, q5, [x0, #320]
	ldp	q6, q7, [x0, #352]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #48
	b.eq	LBB2_13
	ldp	q4, q5, [x0, #384]
	ldp	q6, q7, [x0, #416]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #56
	b.eq	LBB2_13
	ldp	q4, q5, [x0, #448]
	ldp	q6, q7, [x0, #480]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
LBB2_13:
	add.2d	v0, v1, v0
	add.2d	v1, v3, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x9, x10
	b.eq	LBB2_16
LBB2_14:
	sub	x9, x9, x10
	add	x10, x0, x10, lsl #3
LBB2_15:
	ldr	x11, [x10], #8
	add	x8, x11, x8
	subs	x9, x9, #1
	b.ne	LBB2_15
LBB2_16:
	mov	x0, x8
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB2_17:
	.cfi_restore_state
Lloh0:
	adrp	x2, l_anon.566f5b36a5b8e830c7b16729286bcad0.1@PAGE
Lloh1:
	add	x2, x2, l_anon.566f5b36a5b8e830c7b16729286bcad0.1@PAGEOFF
	mov	w0, #64
	mov	w1, #64
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh0, Lloh1
	.cfi_endproc

	.globl	__RNvCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_lowering8sum_full
	.p2align	2
__RNvCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_lowering8sum_full:
	.cfi_startproc
	stp	d9, d8, [sp, #-16]!
	.cfi_def_cfa_offset 16
	.cfi_offset b8, -8
	.cfi_offset b9, -16
	ldp	q0, q1, [x0, #256]
	ldp	q2, q3, [x0]
	ldp	q4, q5, [x0, #384]
	ldp	q6, q7, [x0, #320]
	ldp	q16, q17, [x0, #64]
	ldp	q18, q19, [x0, #448]
	ldp	q20, q21, [x0, #288]
	ldp	q22, q23, [x0, #32]
	ldp	q24, q25, [x0, #416]
	ldp	q26, q27, [x0, #352]
	ldp	q28, q29, [x0, #96]
	ldp	q30, q31, [x0, #480]
	ldp	q8, q9, [x0, #224]
	add.2d	v31, v9, v31
	add.2d	v27, v29, v27
	ldp	q29, q9, [x0, #160]
	add.2d	v27, v27, v31
	add.2d	v25, v9, v25
	add.2d	v21, v23, v21
	add.2d	v21, v21, v25
	ldp	q23, q25, [x0, #192]
	add.2d	v21, v21, v27
	add.2d	v19, v25, v19
	add.2d	v7, v17, v7
	add.2d	v7, v7, v19
	ldp	q17, q19, [x0, #128]
	add.2d	v5, v19, v5
	add.2d	v1, v3, v1
	add.2d	v1, v1, v5
	add.2d	v1, v1, v7
	add.2d	v1, v1, v21
	add.2d	v3, v8, v30
	add.2d	v5, v28, v26
	add.2d	v3, v5, v3
	add.2d	v5, v29, v24
	add.2d	v7, v22, v20
	add.2d	v5, v7, v5
	add.2d	v3, v5, v3
	add.2d	v5, v23, v18
	add.2d	v6, v16, v6
	add.2d	v5, v6, v5
	add.2d	v4, v17, v4
	add.2d	v0, v2, v0
	add.2d	v0, v0, v4
	add.2d	v0, v0, v5
	add.2d	v0, v0, v3
	add.2d	v0, v0, v1
	addp.2d	d0, v0
	fmov	x0, d0
	ldp	d9, d8, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore b8
	.cfi_restore b9
	ret
	.cfi_endproc

	.globl	__RNvCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_lowering9sum_slice
	.p2align	2
__RNvCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_lowering9sum_slice:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	cbz	x2, LBB4_4
	sub	x8, x2, #1
	cmp	x1, x8
	b.ls	LBB4_11
	cmp	x2, #8
	b.hs	LBB4_5
	mov	x8, #0
	mov	x9, #0
	b	LBB4_8
LBB4_4:
	mov	x8, #0
	b	LBB4_10
LBB4_5:
	and	x9, x2, #0xfffffffffffffff8
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	and	x10, x2, #0xfffffffffffffff8
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB4_6:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	subs	x10, x10, #8
	b.ne	LBB4_6
	add.2d	v0, v1, v0
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x2, x9
	b.eq	LBB4_10
LBB4_8:
	sub	x10, x2, x9
	add	x9, x0, x9, lsl #3
LBB4_9:
	ldr	x11, [x9], #8
	add	x8, x11, x8
	subs	x10, x10, #1
	b.ne	LBB4_9
LBB4_10:
	mov	x0, x8
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB4_11:
	.cfi_restore_state
Lloh2:
	adrp	x2, l_anon.566f5b36a5b8e830c7b16729286bcad0.2@PAGE
Lloh3:
	add	x2, x2, l_anon.566f5b36a5b8e830c7b16729286bcad0.2@PAGEOFF
	mov	x0, x1
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh2, Lloh3
	.cfi_endproc

	.globl	__RNvMCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_loweringNtB2_10BoundedRun13sum_via_slice
	.p2align	2
__RNvMCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_loweringNtB2_10BoundedRun13sum_via_slice:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	ldr	x1, [x0, #512]
	cmp	x1, #65
	b.hs	LBB5_11
	cbz	x1, LBB5_4
	lsl	x10, x1, #3
	sub	x8, x10, #8
	cmp	x8, #56
	b.hs	LBB5_5
	mov	x8, #0
	mov	x9, x0
	b	LBB5_8
LBB5_4:
	mov	x8, #0
	b	LBB5_10
LBB5_5:
	lsr	x8, x8, #3
	add	x11, x8, #1
	and	x12, x11, #0x3ffffffffffffff8
	add	x9, x0, x12, lsl #3
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	and	x13, x11, #0x3ffffffffffffff8
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB5_6:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	subs	x13, x13, #8
	b.ne	LBB5_6
	add.2d	v0, v1, v0
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x11, x12
	b.eq	LBB5_10
LBB5_8:
	add	x10, x0, x10
LBB5_9:
	ldr	x11, [x9], #8
	add	x8, x11, x8
	cmp	x9, x10
	b.ne	LBB5_9
LBB5_10:
	mov	x0, x8
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB5_11:
	.cfi_restore_state
Lloh4:
	adrp	x3, l_anon.566f5b36a5b8e830c7b16729286bcad0.3@PAGE
Lloh5:
	add	x3, x3, l_anon.566f5b36a5b8e830c7b16729286bcad0.3@PAGEOFF
	mov	x0, #0
	mov	w2, #64
	bl	__RNvNtNtCs5dyeT9KiOLK_4core5slice5index16slice_index_fail
	.loh AdrpAdd	Lloh4, Lloh5
	.cfi_endproc

	.globl	__RNvMCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_loweringNtB2_10BoundedRun3sum
	.p2align	2
__RNvMCscVIUETtTdXK_43p7_does_the_capacity_bound_survive_loweringNtB2_10BoundedRun3sum:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	ldr	x9, [x0, #512]
	cbz	x9, LBB6_4
	sub	x8, x9, #65
	cmn	x8, #64
	b.lo	LBB6_17
	cmp	x9, #8
	b.hs	LBB6_5
	mov	x8, #0
	mov	x10, #0
	b	LBB6_14
LBB6_4:
	mov	x8, #0
	b	LBB6_16
LBB6_5:
	and	x10, x9, #0x78
	ldp	q0, q1, [x0]
	ldp	q2, q3, [x0, #32]
	cmp	x10, #8
	b.eq	LBB6_13
	ldp	q4, q5, [x0, #64]
	ldp	q6, q7, [x0, #96]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #16
	b.eq	LBB6_13
	ldp	q4, q5, [x0, #128]
	ldp	q6, q7, [x0, #160]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #24
	b.eq	LBB6_13
	ldp	q4, q5, [x0, #192]
	ldp	q6, q7, [x0, #224]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #32
	b.eq	LBB6_13
	ldp	q4, q5, [x0, #256]
	ldp	q6, q7, [x0, #288]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #40
	b.eq	LBB6_13
	ldp	q4, q5, [x0, #320]
	ldp	q6, q7, [x0, #352]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #48
	b.eq	LBB6_13
	ldp	q4, q5, [x0, #384]
	ldp	q6, q7, [x0, #416]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	cmp	x10, #56
	b.eq	LBB6_13
	ldp	q4, q5, [x0, #448]
	ldp	q6, q7, [x0, #480]
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
LBB6_13:
	add.2d	v0, v1, v0
	add.2d	v1, v3, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x9, x10
	b.eq	LBB6_16
LBB6_14:
	sub	x9, x9, x10
	add	x10, x0, x10, lsl #3
LBB6_15:
	ldr	x11, [x10], #8
	add	x8, x11, x8
	subs	x9, x9, #1
	b.ne	LBB6_15
LBB6_16:
	mov	x0, x8
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB6_17:
	.cfi_restore_state
Lloh6:
	adrp	x2, l_anon.566f5b36a5b8e830c7b16729286bcad0.4@PAGE
Lloh7:
	add	x2, x2, l_anon.566f5b36a5b8e830c7b16729286bcad0.4@PAGEOFF
	mov	w0, #64
	mov	w1, #64
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh6, Lloh7
	.cfi_endproc

	.section	__TEXT,__cstring,cstring_literals
l_anon.566f5b36a5b8e830c7b16729286bcad0.0:
	.asciz	"p7_does_the_capacity_bound_survive_lowering.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.566f5b36a5b8e830c7b16729286bcad0.1:
	.quad	l_anon.566f5b36a5b8e830c7b16729286bcad0.0
	.asciz	".\000\000\000\000\000\000\000D\000\000\000 \000\000"

	.p2align	3, 0x0
l_anon.566f5b36a5b8e830c7b16729286bcad0.2:
	.quad	l_anon.566f5b36a5b8e830c7b16729286bcad0.0
	.asciz	".\000\000\000\000\000\000\0001\000\000\000 \000\000"

	.p2align	3, 0x0
l_anon.566f5b36a5b8e830c7b16729286bcad0.3:
	.quad	l_anon.566f5b36a5b8e830c7b16729286bcad0.0
	.asciz	".\000\000\000\000\000\000\000\223\000\000\000\035\000\000"

	.p2align	3, 0x0
l_anon.566f5b36a5b8e830c7b16729286bcad0.4:
	.quad	l_anon.566f5b36a5b8e830c7b16729286bcad0.0
	.asciz	".\000\000\000\000\000\000\000\211\000\000\000$\000\000"

.subsections_via_symbols
