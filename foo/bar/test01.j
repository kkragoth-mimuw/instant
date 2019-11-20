.class public test01
.super  java/lang/Object
.method public <init>()V
	aload_0
	invokespecial java/lang/Object/<init>()V
	return
.end method
.method public static main([Ljava/lang/String;)V
.limit stack 3
.limit locals 1
getstatic  java/lang/System/out Ljava/io/PrintStream;
	bipush 42
	invokevirtual java/io/PrintStream/println(I)V
	return
.end method
