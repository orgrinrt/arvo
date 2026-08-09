	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_ambient_scope
	.p2align	2
_ambient_scope:
	.cfi_startproc
	add	w0, w1, w0
	ret
	.cfi_endproc

	.globl	_bare
_bare = _ambient_scope
	.globl	_declared_under_hot
_declared_under_hot = _ambient_scope
	.globl	_hot_scope
_hot_scope = _ambient_scope
.subsections_via_symbols
