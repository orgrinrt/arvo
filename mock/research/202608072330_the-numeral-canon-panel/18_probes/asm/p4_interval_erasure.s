	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_p4_raw_pair_add
	.p2align	2
_p4_raw_pair_add:
	.cfi_startproc
	add	w0, w2, w0
	add	w1, w3, w1
	ret
	.cfi_endproc

	.globl	_p4_raw_scalar_add
	.p2align	2
_p4_raw_scalar_add:
	.cfi_startproc
	add	w0, w1, w0
	ret
	.cfi_endproc

	.globl	_p4_raw_wide_pair_add
	.p2align	2
_p4_raw_wide_pair_add:
	.cfi_startproc
	adds	x9, x4, x0
	adc	x10, x5, x1
	adds	x11, x6, x2
	stp	x9, x10, [x8]
	adc	x9, x7, x3
	stp	x11, x9, [x8, #16]
	ret
	.cfi_endproc

	.globl	_p4_wide_scalar_add
	.p2align	2
_p4_wide_scalar_add:
	.cfi_startproc
	adds	x0, x2, x0
	adc	x1, x3, x1
	ret
	.cfi_endproc

	.globl	_p4_typed_interval_add
_p4_typed_interval_add = _p4_raw_pair_add
	.globl	_p4_typed_scalar_add
_p4_typed_scalar_add = _p4_raw_scalar_add
	.globl	_p4_wide_interval_add
_p4_wide_interval_add = _p4_raw_wide_pair_add
.subsections_via_symbols
