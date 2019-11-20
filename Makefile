all: build

build:
	cargo build --release
	cp target/release/insc_jvm .
	cp target/release/insc_llvm .