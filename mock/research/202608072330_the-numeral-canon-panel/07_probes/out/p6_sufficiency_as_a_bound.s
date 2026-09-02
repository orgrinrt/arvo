	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCslBZzTB4pgIt_25p6_sufficiency_as_a_bound12call_b_equal
	.p2align	2
__RNvCslBZzTB4pgIt_25p6_sufficiency_as_a_bound12call_b_equal:
	.cfi_startproc
	cbz	x2, LBB0_3
	lsl	x8, x2, #3
LBB0_2:
	ldr	x9, [x1], #8
	adds	x9, x0, x9
	csinv	x0, x9, xzr, lo
	subs	x8, x8, #8
	b.ne	LBB0_2
LBB0_3:
	ret
	.cfi_endproc

	.globl	__RNvCslBZzTB4pgIt_25p6_sufficiency_as_a_bound9fold_bare
	.p2align	2
__RNvCslBZzTB4pgIt_25p6_sufficiency_as_a_bound9fold_bare:
	.cfi_startproc
	cbz	x2, LBB1_3
	lsl	x8, x2, #3
LBB1_2:
	ldr	x9, [x1], #8
	adds	x9, x0, x9
	csinv	x0, x9, xzr, lo
	subs	x8, x8, #8
	b.ne	LBB1_2
LBB1_3:
	ret
	.cfi_endproc

	.globl	__RNvCslBZzTB4pgIt_25p6_sufficiency_as_a_bound6call_a
__RNvCslBZzTB4pgIt_25p6_sufficiency_as_a_bound6call_a = __RNvCslBZzTB4pgIt_25p6_sufficiency_as_a_bound12call_b_equal
	.globl	__RNvCslBZzTB4pgIt_25p6_sufficiency_as_a_bound6call_b
__RNvCslBZzTB4pgIt_25p6_sufficiency_as_a_bound6call_b = __RNvCslBZzTB4pgIt_25p6_sufficiency_as_a_bound12call_b_equal
.subsections_via_symbols
