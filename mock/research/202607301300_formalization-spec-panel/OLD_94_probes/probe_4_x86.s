	.macosx_version_min 10, 12
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_receipt_fast_math
	.p2align	4
_receipt_fast_math:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	$0, -4(%rbp)
	leaq	-4(%rbp), %rax
	## InlineAsm Start

	stmxcsr	(%rax)

	## InlineAsm End
	movl	-4(%rbp), %eax
	notl	%eax
	testl	$57408, %eax
	sete	%al
	popq	%rbp
	retq
	.cfi_endproc

	.globl	_receipt_ieee_default
	.p2align	4
_receipt_ieee_default:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	$0, -4(%rbp)
	leaq	-4(%rbp), %rax
	## InlineAsm Start

	stmxcsr	(%rax)

	## InlineAsm End
	testl	$57408, -4(%rbp)
	sete	%al
	popq	%rbp
	retq
	.cfi_endproc

.subsections_via_symbols
