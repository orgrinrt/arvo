	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
	.cfi_startproc
LBB0_1:
	b	LBB0_1
	.cfi_endproc

	.globl	_derived16
	.p2align	2
_derived16:
	.cfi_startproc
	add	w8, w1, w0
	and	w0, w8, #0xffff
	ret
	.cfi_endproc

	.globl	_derived64
	.p2align	2
_derived64:
	.cfi_startproc
	add	x0, x1, x0
	ret
	.cfi_endproc

	.globl	_native16
_native16 = _derived16
	.globl	_native64
_native64 = _derived64
.subsections_via_symbols
