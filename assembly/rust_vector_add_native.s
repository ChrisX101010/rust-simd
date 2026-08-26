	.file	"lib.c6f551dc431569fb-cgu.0"
	.section	.text._ZN3lib10vector_add17h86d745c9a39cb524E,"ax",@progbits
	.globl	_ZN3lib10vector_add17h86d745c9a39cb524E
	.p2align	4
	.type	_ZN3lib10vector_add17h86d745c9a39cb524E,@function
_ZN3lib10vector_add17h86d745c9a39cb524E:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rsi, 8(%rsp)
	movq	%rcx, 16(%rsp)
	cmpq	%rcx, %rsi
	jne	.LBB0_1
	movq	%rsi, 24(%rsp)
	movq	%r9, 32(%rsp)
	cmpq	%r9, %rsi
	jne	.LBB0_4
	testq	%rsi, %rsi
	je	.LBB0_18
	cmpq	$4, %rsi
	jae	.LBB0_9
	xorl	%eax, %eax
	jmp	.LBB0_8
.LBB0_9:
	movabsq	$2305843009213693920, %rcx
	cmpq	$32, %rsi
	jae	.LBB0_11
	xorl	%eax, %eax
	jmp	.LBB0_15
.LBB0_11:
	movq	%rsi, %r9
	movq	%rsi, %rax
	xorl	%r10d, %r10d
	shrq	$5, %r9
	andq	%rcx, %rax
	shlq	$7, %r9
	.p2align	4
.LBB0_12:
	vmovups	(%rdi,%r10), %ymm0
	vmovups	32(%rdi,%r10), %ymm1
	vmovups	64(%rdi,%r10), %ymm2
	vmovups	96(%rdi,%r10), %ymm3
	vaddps	(%rdx,%r10), %ymm0, %ymm0
	vaddps	32(%rdx,%r10), %ymm1, %ymm1
	vaddps	64(%rdx,%r10), %ymm2, %ymm2
	vaddps	96(%rdx,%r10), %ymm3, %ymm3
	vmovups	%ymm0, (%r8,%r10)
	vmovups	%ymm1, 32(%r8,%r10)
	vmovups	%ymm2, 64(%r8,%r10)
	vmovups	%ymm3, 96(%r8,%r10)
	subq	$-128, %r10
	cmpq	%r10, %r9
	jne	.LBB0_12
	cmpq	%rax, %rsi
	je	.LBB0_18
	testb	$28, %sil
	je	.LBB0_8
.LBB0_15:
	addq	$28, %rcx
	movq	%rax, %r9
	movq	%rcx, %rax
	andq	%rsi, %rax
	.p2align	4
.LBB0_16:
	vmovups	(%rdi,%r9,4), %xmm0
	vaddps	(%rdx,%r9,4), %xmm0, %xmm0
	vmovups	%xmm0, (%r8,%r9,4)
	addq	$4, %r9
	cmpq	%r9, %rax
	jne	.LBB0_16
	jmp	.LBB0_17
.LBB0_1:
	leaq	.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.1(%rip), %r9
	leaq	8(%rsp), %rsi
	leaq	16(%rsp), %rdx
	xorl	%edi, %edi
	xorl	%ecx, %ecx
	callq	*_RINvNtCsgEmfK2I1SDS_4core9panicking13assert_failedjjEB4_@GOTPCREL(%rip)
.LBB0_4:
	leaq	.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.2(%rip), %r9
	leaq	24(%rsp), %rsi
	leaq	32(%rsp), %rdx
	xorl	%edi, %edi
	xorl	%ecx, %ecx
	callq	*_RINvNtCsgEmfK2I1SDS_4core9panicking13assert_failedjjEB4_@GOTPCREL(%rip)
.LBB0_8:
	vmovss	(%rdi,%rax,4), %xmm0
	vaddss	(%rdx,%rax,4), %xmm0, %xmm0
	vmovss	%xmm0, (%r8,%rax,4)
	incq	%rax
.LBB0_17:
	cmpq	%rax, %rsi
	jne	.LBB0_8
.LBB0_18:
	addq	$40, %rsp
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.Lfunc_end0:
	.size	_ZN3lib10vector_add17h86d745c9a39cb524E, .Lfunc_end0-_ZN3lib10vector_add17h86d745c9a39cb524E
	.cfi_endproc

	.type	.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.0,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.0:
	.asciz	"rust/src/lib.rs"
	.size	.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.0, 16

	.type	.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.1,@object
	.section	.data.rel.ro..Lanon.ea8c110565fe96ec1dfc981cbae7af7b.1,"aw",@progbits
	.p2align	3, 0x0
.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.1:
	.quad	.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.0
	.asciz	"\017\000\000\000\000\000\000\000\002\000\000\000\005\000\000"
	.size	.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.1, 24

	.type	.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.2,@object
	.section	.data.rel.ro..Lanon.ea8c110565fe96ec1dfc981cbae7af7b.2,"aw",@progbits
	.p2align	3, 0x0
.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.2:
	.quad	.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.0
	.asciz	"\017\000\000\000\000\000\000\000\003\000\000\000\005\000\000"
	.size	.Lanon.ea8c110565fe96ec1dfc981cbae7af7b.2, 24

	.ident	"rustc version 1.95.0 (59807616e 2026-04-14)"
	.section	".note.GNU-stack","",@progbits
