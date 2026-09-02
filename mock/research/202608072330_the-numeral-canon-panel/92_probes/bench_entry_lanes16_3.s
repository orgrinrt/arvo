
libbench_satfold_lanes16_3.dylib:	file format mach-o arm64

Disassembly of section __TEXT,__text:

00000000000008c4 <_bench_entry>:
	sub	sp, sp, #0x30
	stp	x29, x30, [sp, #0x20]
	add	x29, sp, #0x20
	mov	w9, #0x1b58             ; =7000
	cmp	x2, x9
	b.eq	0xbc4 <_bench_entry+0x300>
	mov	w9, #0x2710             ; =10000
	cmp	x2, x9
	b.eq	0xa38 <_bench_entry+0x174>
	mov	w9, #0x2ee0             ; =12000
	cmp	x2, x9
	b.ne	0xd14 <_bench_entry+0x450>
	mov	x9, #0x0                ; =0
	mrs	x10, CNTVCT_EL0
	mov	w12, #0x1000            ; =4096
	mov	w11, #0xff              ; =255
	mov	x13, #0x0               ; =0
	movi.2d	v0, #0000000000000000
	ldr	q1, [x0, x13]
	uqadd.16b	v0, v0, v1
	add	x13, x13, #0x10
	cmp	x13, #0x1, lsl #12      ; =0x1000
	b.ne	0x90c <_bench_entry+0x48>
	umov.b	w13, v0[0]
	umov.b	w14, v0[1]
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	umov.b	w14, v0[2]
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	umov.b	w14, v0[3]
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	umov.b	w14, v0[4]
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	umov.b	w14, v0[5]
	add	w13, w13, w14, uxtb
	umov.b	w14, v0[6]
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	umov.b	w14, v0[7]
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	umov.b	w14, v0[8]
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	umov.b	w14, v0[9]
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	umov.b	w14, v0[10]
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	umov.b	w14, v0[11]
	add	w13, w13, w14, uxtb
	umov.b	w14, v0[12]
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	umov.b	w14, v0[13]
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	umov.b	w14, v0[14]
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	umov.b	w14, v0[15]
	add	w13, w13, w14, uxtb
	cmp	w13, #0xff
	csel	w13, w13, w11, lo
	eor	x9, x13, x9, ror #57
	add	x0, x0, #0x1, lsl #12   ; =0x1000
	cmp	x12, #0x7, lsl #12      ; =0x7000
	add	x12, x12, #0x1, lsl #12 ; =0x1000
	b.ls	0x904 <_bench_entry+0x40>
	str	x9, [x1]
	mrs	x9, CNTVCT_EL0
	sub	x9, x9, x10
	b	0xd00 <_bench_entry+0x43c>
	mov	x10, #0x0               ; =0
	mrs	x9, CNTVCT_EL0
	add	x11, x0, #0x80
	mov	w14, #0xf0              ; =240
	mov	w12, #0xff              ; =255
	mov	w13, #0x7f00            ; =32512
	ldp	q0, q1, [x11, #-0x80]
	uqadd.16b	v0, v0, v1
	ldp	q1, q2, [x11, #-0x60]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	ldp	q1, q2, [x11, #-0x40]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	ldp	q1, q2, [x11, #-0x20]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	ldp	q1, q2, [x11]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	ldp	q1, q2, [x11, #0x20]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	ldp	q1, q2, [x11, #0x40]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	ldp	q1, q2, [x11, #0x60]
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	umov.b	w15, v0[0]
	umov.b	w16, v0[1]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	umov.b	w16, v0[2]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	umov.b	w16, v0[3]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	umov.b	w16, v0[4]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	umov.b	w16, v0[5]
	add	w15, w15, w16, uxtb
	umov.b	w16, v0[6]
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	add	w15, w15, w16, uxtb
	umov.b	w16, v0[7]
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	umov.b	w16, v0[8]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	umov.b	w16, v0[9]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	umov.b	w16, v0[10]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	umov.b	w16, v0[11]
	add	w15, w15, w16, uxtb
	umov.b	w16, v0[12]
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	add	w15, w15, w16, uxtb
	umov.b	w16, v0[13]
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	umov.b	w16, v0[14]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	umov.b	w16, v0[15]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w12, lo
	eor	x10, x15, x10, ror #57
	add	x15, x14, #0x10
	add	x14, x14, #0x100
	add	x11, x11, #0x100
	cmp	x15, x13
	b.ls	0xa50 <_bench_entry+0x18c>
	str	x10, [x1]
	mrs	x10, CNTVCT_EL0
	b	0xcfc <_bench_entry+0x438>
	mov	x11, #0x0               ; =0
	mov	x10, #0x0               ; =0
	mrs	x9, CNTVCT_EL0
	add	x12, x0, #0x20
	mov	w13, #0xff              ; =255
	mov	w14, #0x7fc0            ; =32704
	ldp	q0, q1, [x12, #-0x20]
	uqadd.16b	v0, v0, v1
	ldp	q1, q2, [x12], #0x40
	uqadd.16b	v0, v0, v1
	uqadd.16b	v0, v0, v2
	umov.b	w15, v0[0]
	umov.b	w16, v0[1]
	add	w15, w15, w16, uxtb
	umov.b	w16, v0[2]
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	umov.b	w16, v0[3]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	umov.b	w16, v0[4]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	umov.b	w16, v0[5]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	umov.b	w16, v0[6]
	add	w15, w15, w16, uxtb
	umov.b	w16, v0[7]
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	add	w15, w15, w16, uxtb
	umov.b	w16, v0[8]
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	umov.b	w16, v0[9]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	umov.b	w16, v0[10]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	umov.b	w16, v0[11]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	umov.b	w16, v0[12]
	add	w15, w15, w16, uxtb
	umov.b	w16, v0[13]
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	add	w15, w15, w16, uxtb
	umov.b	w16, v0[14]
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	umov.b	w16, v0[15]
	add	w15, w15, w16, uxtb
	cmp	w15, #0xff
	csel	w15, w15, w13, lo
	eor	x10, x15, x10, ror #57
	add	x11, x11, #0x40
	cmp	x11, x14
	b.ls	0xbdc <_bench_entry+0x318>
	str	x10, [x1]
	mrs	x10, CNTVCT_EL0
	sub	x9, x10, x9
	stp	x9, xzr, [x8]
	stp	xzr, xzr, [x8, #0x10]
	ldp	x29, x30, [sp, #0x20]
	add	sp, sp, #0x30
	ret
	str	x2, [sp, #0x8]
	add	x8, sp, #0x8
	adrp	x9, 0x2c000 <__RNvXs2_NtNtCs5dyeT9KiOLK_4core3str5lossyNtB5_10Utf8ChunksNtNtNtNtB9_4iter6traits8iterator8Iterator4next+0x4>
	add	x9, x9, #0xc78
	stp	x8, x9, [sp, #0x10]
	adrp	x0, 0x32000 <GCC_except_table452+0x64>
	add	x0, x0, #0x890
	adrp	x2, 0x40000 <dyld_stub_binder+0x40000>
	add	x2, x2, #0x10
	add	x1, sp, #0x10
	bl	0x31344 <__RNvNtCs5dyeT9KiOLK_4core9panicking9panic_fmt>
	brk	#0x1
	bl	0x312f8 <__RNvNtCs5dyeT9KiOLK_4core9panicking19panic_cannot_unwind>
