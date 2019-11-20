declare void @printInt(i32)
define i32 @main() {
	%loc_a = alloca i32
	store i32 0, i32* %loc_a
	%loc_b = alloca i32
	store i32 1, i32* %loc_b
	%loc_c = alloca i32
	store i32 0, i32* %loc_c
	%loc_d = alloca i32
	store i32 1, i32* %loc_d
	%loc_e = alloca i32
	store i32 0, i32* %loc_e
	%loc_f = alloca i32
	store i32 1, i32* %loc_f
	%loc_g = alloca i32
	store i32 0, i32* %loc_g
	%loc_h = alloca i32
	store i32 1, i32* %loc_h
	%a0 = load i32, i32* %loc_a
	%b0 = load i32, i32* %loc_b
	%r1 = mul i32 %a0, %b0
	%c0 = load i32, i32* %loc_c
	%d0 = load i32, i32* %loc_d
	%r2 = mul i32 %c0, %d0
	%r3 = add i32 %r1, %r2
	%e0 = load i32, i32* %loc_e
	%r4 = add i32 %r3, %e0
	%f0 = load i32, i32* %loc_f
	%r5 = add i32 %r4, %f0
	%g0 = load i32, i32* %loc_g
	%r6 = add i32 %r5, %g0
	%h0 = load i32, i32* %loc_h
	%r7 = add i32 %r6, %h0
	call void @printInt(i32 %r7)
	store i32 1, i32* %loc_a
	store i32 2, i32* %loc_b
	store i32 1, i32* %loc_c
	store i32 2, i32* %loc_d
	store i32 1, i32* %loc_e
	store i32 2, i32* %loc_f
	store i32 1, i32* %loc_g
	store i32 2, i32* %loc_h
	%loc_i = alloca i32
	store i32 1, i32* %loc_i
	%loc_j = alloca i32
	store i32 2, i32* %loc_j
	%loc_k = alloca i32
	store i32 1, i32* %loc_k
	%loc_l = alloca i32
	store i32 2, i32* %loc_l
	%loc_m = alloca i32
	store i32 1, i32* %loc_m
	%loc_n = alloca i32
	store i32 2, i32* %loc_n
	%a1 = load i32, i32* %loc_a
	%r8 = mul i32 2, %a1
	%b1 = load i32, i32* %loc_b
	%r9 = sdiv i32 %b1, 2
	%r10 = add i32 %r8, %r9
	%c1 = load i32, i32* %loc_c
	%r11 = add i32 %r10, %c1
	%d1 = load i32, i32* %loc_d
	%r12 = add i32 %r11, %d1
	%e1 = load i32, i32* %loc_e
	%r13 = add i32 %r12, %e1
	%f1 = load i32, i32* %loc_f
	%r14 = add i32 %r13, %f1
	%g1 = load i32, i32* %loc_g
	%r15 = add i32 %r14, %g1
	%h1 = load i32, i32* %loc_h
	%r16 = add i32 %r15, %h1
	%i0 = load i32, i32* %loc_i
	%r17 = add i32 %r16, %i0
	%j0 = load i32, i32* %loc_j
	%r18 = sdiv i32 %j0, 2
	%r19 = add i32 %r17, %r18
	%k0 = load i32, i32* %loc_k
	%r20 = add i32 %r19, %k0
	%l0 = load i32, i32* %loc_l
	%r21 = add i32 %r20, %l0
	%m0 = load i32, i32* %loc_m
	%r22 = add i32 %r21, %m0
	%n0 = load i32, i32* %loc_n
	%r23 = add i32 %r22, %n0
	%r24 = sdiv i32 %r23, 10
	call void @printInt(i32 %r24)
	ret i32 0
}