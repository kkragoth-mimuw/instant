declare void @printInt(i32)
define i32 @main() {
	%loc_n = alloca i32
	%r1 = add i32 2, 2
	store i32 %r1, i32* %loc_n
	%n0 = load i32, i32* %loc_n
	call void @printInt(i32 %n0)
	ret i32 0
}