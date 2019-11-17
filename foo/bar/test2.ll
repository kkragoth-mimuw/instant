declare void @printInt(i32)

define i32 @main() {
	%loc_n = alloca i32
	store i32 100, i32* %loc_n
	%loc_k = alloca i32
	%n1 = load i32, i32* loc_n
	%n2 = load i32, i32* loc_n
	%r1 = mul i32 %n1, %n2
	%r2 = add i32 %r1, 1
	store i32 %r2, i32* %loc_k
	%k1 = load i32, i32* loc_k
	call void @printInt(i32 %k1)
}