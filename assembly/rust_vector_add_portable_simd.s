	.file	"vector_add_portable_simd.b6d02ebca9e1cb0-cgu.0"
	.section	.rodata.cst32,"aM",@progbits,32
	.p2align	5, 0x0
.LCPI0_0:
	.long	0
	.long	1
	.long	2
	.long	3
	.long	4
	.long	5
	.long	6
	.long	7
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.LCPI0_1:
	.long	8
.LCPI0_2:
	.long	16
.LCPI0_3:
	.long	24
.LCPI0_4:
	.long	274877907
.LCPI0_5:
	.long	1000
.LCPI0_6:
	.long	32
.LCPI0_7:
	.long	48
.LCPI0_8:
	.long	72
	.section	.rodata.cst8,"aM",@progbits,8
	.p2align	3, 0x0
.LCPI0_9:
	.quad	0x8000000000000000
	.section	.text._ZN24vector_add_portable_simd4main17hfc399c4059b9c4c1E,"ax",@progbits
	.hidden	_ZN24vector_add_portable_simd4main17hfc399c4059b9c4c1E
	.globl	_ZN24vector_add_portable_simd4main17hfc399c4059b9c4c1E
	.p2align	4
	.type	_ZN24vector_add_portable_simd4main17hfc399c4059b9c4c1E,@function
_ZN24vector_add_portable_simd4main17hfc399c4059b9c4c1E:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r12
	.cfi_def_cfa_offset 32
	pushq	%rbx
	.cfi_def_cfa_offset 40
	subq	$216, %rsp
	.cfi_def_cfa_offset 256
	.cfi_offset %rbx, -40
	.cfi_offset %r12, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	callq	*_RNvCsj0qrnF3AvHz_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$67108864, %edi
	movl	$4, %esi
	callq	*_RNvCsj0qrnF3AvHz_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB0_22
	vmovdqa	.LCPI0_0(%rip), %ymm0
	vpbroadcastd	.LCPI0_1(%rip), %ymm1
	vpbroadcastd	.LCPI0_2(%rip), %ymm2
	vpbroadcastd	.LCPI0_3(%rip), %ymm9
	vpbroadcastd	.LCPI0_4(%rip), %ymm10
	vpbroadcastd	.LCPI0_5(%rip), %ymm11
	vpbroadcastd	.LCPI0_6(%rip), %ymm12
	movq	%rax, %rbx
	movl	$24, %eax
	.p2align	4
.LBB0_2:
	vpshufd	$245, %ymm0, %ymm5
	vpmuludq	%ymm0, %ymm10, %ymm6
	vpaddd	%ymm1, %ymm0, %ymm3
	vpaddd	%ymm2, %ymm0, %ymm4
	vpmuludq	%ymm5, %ymm10, %ymm5
	vpmuludq	%ymm3, %ymm10, %ymm7
	vpmuludq	%ymm4, %ymm10, %ymm8
	vpshufd	$245, %ymm6, %ymm6
	vpblendd	$170, %ymm5, %ymm6, %ymm5
	vpshufd	$245, %ymm3, %ymm6
	vpshufd	$245, %ymm7, %ymm7
	vpshufd	$245, %ymm8, %ymm8
	vpmuludq	%ymm6, %ymm10, %ymm6
	vpsrld	$6, %ymm5, %ymm5
	vpmulld	%ymm11, %ymm5, %ymm5
	vpblendd	$170, %ymm6, %ymm7, %ymm6
	vpshufd	$245, %ymm4, %ymm7
	vpmuludq	%ymm7, %ymm10, %ymm7
	vpsrld	$6, %ymm6, %ymm6
	vpsubd	%ymm5, %ymm0, %ymm5
	vpmulld	%ymm11, %ymm6, %ymm6
	vcvtdq2ps	%ymm5, %ymm5
	vpblendd	$170, %ymm7, %ymm8, %ymm7
	vpaddd	%ymm0, %ymm9, %ymm8
	vmovups	%ymm5, -96(%rbx,%rax,4)
	vpaddd	%ymm0, %ymm12, %ymm0
	vpsubd	%ymm6, %ymm3, %ymm3
	vpsrld	$6, %ymm7, %ymm6
	vpmuludq	%ymm10, %ymm8, %ymm7
	vpmulld	%ymm11, %ymm6, %ymm6
	vcvtdq2ps	%ymm3, %ymm3
	vpshufd	$245, %ymm7, %ymm7
	vmovups	%ymm3, -64(%rbx,%rax,4)
	vpsubd	%ymm6, %ymm4, %ymm4
	vpshufd	$245, %ymm8, %ymm6
	vpmuludq	%ymm6, %ymm10, %ymm6
	vcvtdq2ps	%ymm4, %ymm4
	vmovups	%ymm4, -32(%rbx,%rax,4)
	vpblendd	$170, %ymm6, %ymm7, %ymm6
	vpsrld	$6, %ymm6, %ymm6
	vpmulld	%ymm11, %ymm6, %ymm6
	vpsubd	%ymm6, %ymm8, %ymm6
	vcvtdq2ps	%ymm6, %ymm6
	vmovups	%ymm6, (%rbx,%rax,4)
	addq	$32, %rax
	cmpq	$16777240, %rax
	jne	.LBB0_2
	vmovdqu	%ymm12, 80(%rsp)
	vmovdqu	%ymm11, 112(%rsp)
	vmovdqu	%ymm10, 144(%rsp)
	vmovdqu	%ymm9, 176(%rsp)
	vzeroupper
	callq	*_RNvCsj0qrnF3AvHz_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$67108864, %edi
	movl	$4, %esi
	callq	*_RNvCsj0qrnF3AvHz_7___rustc12___rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB0_8
	vmovdqa	.LCPI0_0(%rip), %ymm0
	vpbroadcastd	.LCPI0_7(%rip), %ymm1
	vpbroadcastd	.LCPI0_8(%rip), %ymm2
	vmovdqu	176(%rsp), %ymm10
	vmovdqu	144(%rsp), %ymm11
	vmovdqu	112(%rsp), %ymm12
	vmovdqu	80(%rsp), %ymm13
	movq	%rax, %r14
	movl	$24, %eax
	.p2align	4
.LBB0_5:
	vpaddd	%ymm0, %ymm0, %ymm3
	vpaddd	%ymm3, %ymm0, %ymm3
	vpaddd	%ymm0, %ymm13, %ymm0
	vpshufd	$245, %ymm3, %ymm6
	vpmuludq	%ymm3, %ymm11, %ymm7
	vpaddd	%ymm3, %ymm10, %ymm4
	vpaddd	%ymm1, %ymm3, %ymm5
	vpmuludq	%ymm6, %ymm11, %ymm6
	vpmuludq	%ymm4, %ymm11, %ymm8
	vpmuludq	%ymm5, %ymm11, %ymm9
	vpshufd	$245, %ymm7, %ymm7
	vpblendd	$170, %ymm6, %ymm7, %ymm6
	vpshufd	$245, %ymm4, %ymm7
	vpshufd	$245, %ymm8, %ymm8
	vpshufd	$245, %ymm9, %ymm9
	vpmuludq	%ymm7, %ymm11, %ymm7
	vpsrld	$6, %ymm6, %ymm6
	vpmulld	%ymm12, %ymm6, %ymm6
	vpblendd	$170, %ymm7, %ymm8, %ymm7
	vpshufd	$245, %ymm5, %ymm8
	vpmuludq	%ymm11, %ymm8, %ymm8
	vpsrld	$6, %ymm7, %ymm7
	vpsubd	%ymm6, %ymm3, %ymm6
	vpaddd	%ymm2, %ymm3, %ymm3
	vpmulld	%ymm12, %ymm7, %ymm7
	vcvtdq2ps	%ymm6, %ymm6
	vpblendd	$170, %ymm8, %ymm9, %ymm8
	vmovups	%ymm6, -96(%r14,%rax,4)
	vpsubd	%ymm7, %ymm4, %ymm4
	vpsrld	$6, %ymm8, %ymm7
	vpmuludq	%ymm3, %ymm11, %ymm8
	vpmulld	%ymm12, %ymm7, %ymm7
	vcvtdq2ps	%ymm4, %ymm4
	vpshufd	$245, %ymm8, %ymm8
	vmovups	%ymm4, -64(%r14,%rax,4)
	vpsubd	%ymm7, %ymm5, %ymm5
	vpshufd	$245, %ymm3, %ymm7
	vpmuludq	%ymm7, %ymm11, %ymm7
	vcvtdq2ps	%ymm5, %ymm5
	vmovups	%ymm5, -32(%r14,%rax,4)
	vpblendd	$170, %ymm7, %ymm8, %ymm7
	vpsrld	$6, %ymm7, %ymm7
	vpmulld	%ymm12, %ymm7, %ymm7
	vpsubd	%ymm7, %ymm3, %ymm3
	vcvtdq2ps	%ymm3, %ymm3
	vmovups	%ymm3, (%r14,%rax,4)
	addq	$32, %rax
	cmpq	$16777240, %rax
	jne	.LBB0_5
	vzeroupper
	callq	*_RNvCsj0qrnF3AvHz_7___rustc35___rust_no_alloc_shim_is_unstable_v2@GOTPCREL(%rip)
	movl	$67108864, %edi
	movl	$4, %esi
	callq	*_RNvCsj0qrnF3AvHz_7___rustc19___rust_alloc_zeroed@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB0_7
	movq	%rax, %r15
	xorl	%eax, %eax
	.p2align	4
.LBB0_12:
	vmovups	(%rbx,%rax,4), %ymm0
	vaddps	(%r14,%rax,4), %ymm0, %ymm0
	vmovups	%ymm0, (%r15,%rax,4)
	addq	$8, %rax
	cmpq	$16777215, %rax
	jbe	.LBB0_12
	vmovsd	.LCPI0_9(%rip), %xmm0
	xorl	%eax, %eax
	.p2align	4
.LBB0_14:
	vmovss	(%r15,%rax,4), %xmm1
	incq	%rax
	vcvtss2sd	%xmm1, %xmm1, %xmm1
	vaddsd	%xmm1, %xmm0, %xmm0
	cmpq	$16777216, %rax
	jne	.LBB0_14
	movq	_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h4b12ff4455b942daE@GOTPCREL(%rip), %rdx
	leaq	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.2(%rip), %rax
	leaq	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.5(%rip), %rcx
	leaq	8(%rsp), %r12
	vmovsd	%xmm0, 72(%rsp)
	movq	%rax, 8(%rsp)
	movq	%rcx, 24(%rsp)
	movq	$2, 32(%rsp)
	movq	$0, 56(%rsp)
	movq	%r12, 40(%rsp)
	movq	$1, 48(%rsp)
	movq	%rdx, 16(%rsp)
.Ltmp0:
	leaq	24(%rsp), %rdi
	vzeroupper
	callq	*_ZN3std2io5stdio6_print17h30e17c2858b8ae61E@GOTPCREL(%rip)
.Ltmp1:
	movq	_ZN4core3fmt5float52_$LT$impl$u20$core..fmt..Display$u20$for$u20$f64$GT$3fmt17h8d923e611dd470a3E@GOTPCREL(%rip), %rcx
	leaq	72(%rsp), %rax
	leaq	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.7(%rip), %rdx
	movq	%rax, 8(%rsp)
	movq	%rdx, 24(%rsp)
	movq	$2, 32(%rsp)
	movq	%rcx, 16(%rsp)
	leaq	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.8(%rip), %rcx
	movq	%rcx, 56(%rsp)
	movq	$1, 64(%rsp)
	movq	%r12, 40(%rsp)
	movq	$1, 48(%rsp)
.Ltmp2:
	leaq	24(%rsp), %rdi
	callq	*_ZN3std2io5stdio6_print17h30e17c2858b8ae61E@GOTPCREL(%rip)
.Ltmp3:
	movq	_RNvCsj0qrnF3AvHz_7___rustc14___rust_dealloc@GOTPCREL(%rip), %r12
	movl	$67108864, %esi
	movl	$4, %edx
	movq	%r15, %rdi
	callq	*%r12
	movl	$67108864, %esi
	movl	$4, %edx
	movq	%r14, %rdi
	callq	*%r12
	movl	$67108864, %esi
	movl	$4, %edx
	movq	%rbx, %rdi
	callq	*%r12
	addq	$216, %rsp
	.cfi_def_cfa_offset 40
	popq	%rbx
	.cfi_def_cfa_offset 32
	popq	%r12
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.LBB0_22:
	.cfi_def_cfa_offset 256
	leaq	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.11(%rip), %rdx
	movl	$4, %edi
	movl	$67108864, %esi
	callq	*_ZN5alloc7raw_vec12handle_error17h7d94a8ed9798168aE@GOTPCREL(%rip)
.LBB0_8:
.Ltmp8:
	leaq	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.11(%rip), %rdx
	movl	$4, %edi
	movl	$67108864, %esi
	callq	*_ZN5alloc7raw_vec12handle_error17h7d94a8ed9798168aE@GOTPCREL(%rip)
.Ltmp9:
	jmp	.LBB0_9
.LBB0_7:
.Ltmp5:
	leaq	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.1(%rip), %rdx
	movl	$4, %edi
	movl	$67108864, %esi
	callq	*_ZN5alloc7raw_vec12handle_error17h7d94a8ed9798168aE@GOTPCREL(%rip)
.Ltmp6:
.LBB0_9:
	ud2
.LBB0_10:
.Ltmp7:
	movq	%rax, %r12
	jmp	.LBB0_19
.LBB0_20:
.Ltmp10:
	movq	%rax, %r12
	jmp	.LBB0_21
.LBB0_18:
.Ltmp4:
	movl	$67108864, %esi
	movl	$4, %edx
	movq	%r15, %rdi
	movq	%rax, %r12
	callq	*_RNvCsj0qrnF3AvHz_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB0_19:
	movl	$67108864, %esi
	movl	$4, %edx
	movq	%r14, %rdi
	callq	*_RNvCsj0qrnF3AvHz_7___rustc14___rust_dealloc@GOTPCREL(%rip)
.LBB0_21:
	movl	$67108864, %esi
	movl	$4, %edx
	movq	%rbx, %rdi
	callq	*_RNvCsj0qrnF3AvHz_7___rustc14___rust_dealloc@GOTPCREL(%rip)
	movq	%r12, %rdi
	callq	_Unwind_Resume@PLT
.Lfunc_end0:
	.size	_ZN24vector_add_portable_simd4main17hfc399c4059b9c4c1E, .Lfunc_end0-_ZN24vector_add_portable_simd4main17hfc399c4059b9c4c1E
	.cfi_endproc
	.section	.gcc_except_table._ZN24vector_add_portable_simd4main17hfc399c4059b9c4c1E,"a",@progbits
	.p2align	2, 0x0
GCC_except_table0:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp3-.Ltmp0
	.uleb128 .Ltmp4-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp3-.Lfunc_begin0
	.uleb128 .Ltmp8-.Ltmp3
	.byte	0
	.byte	0
	.uleb128 .Ltmp8-.Lfunc_begin0
	.uleb128 .Ltmp9-.Ltmp8
	.uleb128 .Ltmp10-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp5-.Lfunc_begin0
	.uleb128 .Ltmp6-.Ltmp5
	.uleb128 .Ltmp7-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp6-.Lfunc_begin0
	.uleb128 .Lfunc_end0-.Ltmp6
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0

	.section	.text._ZN3std2rt10lang_start17ha6d0a8a2d42c066dE,"ax",@progbits
	.hidden	_ZN3std2rt10lang_start17ha6d0a8a2d42c066dE
	.globl	_ZN3std2rt10lang_start17ha6d0a8a2d42c066dE
	.p2align	4
	.type	_ZN3std2rt10lang_start17ha6d0a8a2d42c066dE,@function
_ZN3std2rt10lang_start17ha6d0a8a2d42c066dE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movl	%ecx, %r8d
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, (%rsp)
	leaq	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.9(%rip), %rsi
	movq	%rsp, %rdi
	callq	*_ZN3std2rt19lang_start_internal17h7d5fc939eaf0226fE@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end1:
	.size	_ZN3std2rt10lang_start17ha6d0a8a2d42c066dE, .Lfunc_end1-_ZN3std2rt10lang_start17ha6d0a8a2d42c066dE
	.cfi_endproc

	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h45c413496d2a9962E","ax",@progbits
	.p2align	4
	.type	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h45c413496d2a9962E,@function
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h45c413496d2a9962E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h89b27790cdf9ac54E
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h45c413496d2a9962E, .Lfunc_end2-_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h45c413496d2a9962E
	.cfi_endproc

	.section	.text._ZN3std3sys9backtrace28__rust_begin_short_backtrace17h89b27790cdf9ac54E,"ax",@progbits
	.p2align	4
	.type	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h89b27790cdf9ac54E,@function
_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h89b27790cdf9ac54E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	*%rdi
	#APP
	#NO_APP
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end3:
	.size	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h89b27790cdf9ac54E, .Lfunc_end3-_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h89b27790cdf9ac54E
	.cfi_endproc

	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h0ecb4dc98dd90a6fE","ax",@progbits
	.p2align	4
	.type	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h0ecb4dc98dd90a6fE,@function
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h0ecb4dc98dd90a6fE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h89b27790cdf9ac54E
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end4:
	.size	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h0ecb4dc98dd90a6fE, .Lfunc_end4-_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h0ecb4dc98dd90a6fE
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsi, %rcx
	movslq	%edi, %rdx
	leaq	_ZN24vector_add_portable_simd4main17hfc399c4059b9c4c1E(%rip), %rax
	leaq	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.9(%rip), %rsi
	movq	%rsp, %rdi
	xorl	%r8d, %r8d
	movq	%rax, (%rsp)
	callq	*_ZN3std2rt19lang_start_internal17h7d5fc939eaf0226fE@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end5:
	.size	main, .Lfunc_end5-main
	.cfi_endproc

	.type	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.0,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.47e6ef81e525f3fb835cf40af6fb2779.0:
	.asciz	"rust/src/bin/vector_add_portable_simd.rs"
	.size	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.0, 41

	.type	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.1,@object
	.section	.data.rel.ro..Lanon.47e6ef81e525f3fb835cf40af6fb2779.1,"aw",@progbits
	.p2align	3, 0x0
.Lanon.47e6ef81e525f3fb835cf40af6fb2779.1:
	.quad	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.0
	.asciz	"(\000\000\000\000\000\000\000'\000\000\000\023\000\000"
	.size	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.1, 24

	.type	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.2,@object
	.section	.rodata.cst8,"aM",@progbits,8
	.p2align	3, 0x0
.Lanon.47e6ef81e525f3fb835cf40af6fb2779.2:
	.asciz	"\000\000\000\001\000\000\000"
	.size	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.2, 8

	.type	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.3,@object
	.section	.rodata..Lanon.47e6ef81e525f3fb835cf40af6fb2779.3,"a",@progbits
.Lanon.47e6ef81e525f3fb835cf40af6fb2779.3:
	.ascii	"elements: "
	.size	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.3, 10

	.type	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.4,@object
	.section	.rodata..Lanon.47e6ef81e525f3fb835cf40af6fb2779.4,"a",@progbits
.Lanon.47e6ef81e525f3fb835cf40af6fb2779.4:
	.byte	10
	.size	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.4, 1

	.type	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.5,@object
	.section	.data.rel.ro..Lanon.47e6ef81e525f3fb835cf40af6fb2779.5,"aw",@progbits
	.p2align	3, 0x0
.Lanon.47e6ef81e525f3fb835cf40af6fb2779.5:
	.quad	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.3
	.asciz	"\n\000\000\000\000\000\000"
	.quad	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.4
	.asciz	"\001\000\000\000\000\000\000"
	.size	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.5, 32

	.type	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.6,@object
	.section	.rodata..Lanon.47e6ef81e525f3fb835cf40af6fb2779.6,"a",@progbits
.Lanon.47e6ef81e525f3fb835cf40af6fb2779.6:
	.ascii	"checksum: "
	.size	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.6, 10

	.type	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.7,@object
	.section	.data.rel.ro..Lanon.47e6ef81e525f3fb835cf40af6fb2779.7,"aw",@progbits
	.p2align	3, 0x0
.Lanon.47e6ef81e525f3fb835cf40af6fb2779.7:
	.quad	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.6
	.asciz	"\n\000\000\000\000\000\000"
	.quad	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.4
	.asciz	"\001\000\000\000\000\000\000"
	.size	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.7, 32

	.type	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.8,@object
	.section	.rodata..Lanon.47e6ef81e525f3fb835cf40af6fb2779.8,"a",@progbits
	.p2align	3, 0x0
.Lanon.47e6ef81e525f3fb835cf40af6fb2779.8:
	.asciz	"\000\000\003"
	.zero	12
	.asciz	"\002"
	.zero	14
	.ascii	"\000\000\000\000\000\000\000\000 \000\000\360"
	.zero	4
	.size	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.8, 48

	.type	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.9,@object
	.section	.data.rel.ro..Lanon.47e6ef81e525f3fb835cf40af6fb2779.9,"aw",@progbits
	.p2align	3, 0x0
.Lanon.47e6ef81e525f3fb835cf40af6fb2779.9:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h0ecb4dc98dd90a6fE
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h45c413496d2a9962E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h45c413496d2a9962E
	.size	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.9, 48

	.type	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.10,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.47e6ef81e525f3fb835cf40af6fb2779.10:
	.asciz	"/rustc/7c275d09ea6b953d2cca169667184a7214bd14c7/library/core/src/iter/traits/iterator.rs"
	.size	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.10, 89

	.type	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.11,@object
	.section	.data.rel.ro..Lanon.47e6ef81e525f3fb835cf40af6fb2779.11,"aw",@progbits
	.p2align	3, 0x0
.Lanon.47e6ef81e525f3fb835cf40af6fb2779.11:
	.quad	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.10
	.asciz	"X\000\000\000\000\000\000\000\353\007\000\000\t\000\000"
	.size	.Lanon.47e6ef81e525f3fb835cf40af6fb2779.11, 24

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.ident	"rustc version 1.92.0-nightly (7c275d09e 2025-09-18)"
	.section	".note.GNU-stack","",@progbits
