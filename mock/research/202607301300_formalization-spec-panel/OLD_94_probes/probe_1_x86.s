	.macosx_version_min 10, 12
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_receipt_x86_full
	.p2align	4
_receipt_x86_full:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	$0, -8(%rbp)
	leaq	-8(%rbp), %rax
	## InlineAsm Start

	stmxcsr	(%rax)

	## InlineAsm End
	testl	$57408, -8(%rbp)
	sete	%cl
	movw	$0, -2(%rbp)
	leaq	-2(%rbp), %rax
	## InlineAsm Start

	fnstcw	(%rax)

	## InlineAsm End
	movzwl	-2(%rbp), %eax
	andl	$3840, %eax
	cmpl	$768, %eax
	sete	%al
	andb	%cl, %al
	popq	%rbp
	retq
	.cfi_endproc

	.globl	_receipt_x86_mxcsr
	.p2align	4
_receipt_x86_mxcsr:
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

	.globl	_receipt_x86_transliterated
	.p2align	4
_receipt_x86_transliterated:
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
	testl	$57344, -4(%rbp)
	sete	%al
	popq	%rbp
	retq
	.cfi_endproc

.subsections_via_symbols
