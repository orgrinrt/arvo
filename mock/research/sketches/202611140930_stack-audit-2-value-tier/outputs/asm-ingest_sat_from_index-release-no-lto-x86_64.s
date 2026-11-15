_RNvCs3KavzfkEBcd_24stack_audit_2_value_tier21ingest_sat_from_index:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$24, %rsp
	.cfi_def_cfa_offset 80
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rcx, %rbx
	cmpq	%rcx, %rsi
	cmovbq	%rsi, %rbx
	testq	%rbx, %rbx
	je	.LBB148_3
	movq	%rdx, %r14
	movq	%rdi, %r15
	movl	$8191, %r13d
	xorl	%r12d, %r12d
	.p2align	4
.LBB148_2:
	leaq	1(%r12), %rbp
	movzwl	(%r15,%r12,2), %esi
	leaq	8(%rsp), %rdi
	callq	*_RNvNtCsUkwczUb7yC_12arvo_numeral4host8unsigned@GOTPCREL(%rip)
	movq	8(%rsp), %rax
	cmpq	$8191, %rax
	cmovaeq	%r13, %rax
	cmpb	$0, 16(%rsp)
	movl	$0, %ecx
	cmovnel	%ecx, %eax
	movw	%ax, (%r14,%r12,2)
	movq	%rbp, %r12
	cmpq	%rbp, %rbx
	jne	.LBB148_2
.LBB148_3:
	addq	$24, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end148:
	.size	_RNvCs3KavzfkEBcd_24stack_audit_2_value_tier21ingest_sat_from_index, .Lfunc_end148-_RNvCs3KavzfkEBcd_24stack_audit_2_value_tier21ingest_sat_from_index
