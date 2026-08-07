	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_ambient
	.p2align	2
_ambient:
	.cfi_startproc
	add	w0, w1, w0
	ret
	.cfi_endproc

	.globl	_bare
_bare = _ambient
	.globl	_hot_scope
_hot_scope = _ambient
.subsections_via_symbols
