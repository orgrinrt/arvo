	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_arvo16
	.p2align	2
_arvo16:
	.cfi_startproc
	add	w0, w1, w0
	ret
	.cfi_endproc

	.globl	_arvo64
	.p2align	2
_arvo64:
	.cfi_startproc
	add	x0, x1, x0
	ret
	.cfi_endproc

	.globl	_native16
_native16 = _arvo16
	.globl	_native64
_native64 = _arvo64
.subsections_via_symbols
