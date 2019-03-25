build:
	bootimage build

run-interactive: build
	qemu-system-x86_64 \
	    -drive format=raw,file=target/x86_64-unknown-raw/debug/bootimage-mtos.bin \
	    -serial file:serial.txt \
	    -device isa-debug-exit,iobase=0xf4,iosize=0x04

integration-tests: build
	bootimage test

run-background: build
	qemu-system-x86_64 \
	    -drive format=raw,file=target/x86_64-unknown-raw/debug/bootimage-mtos.bin \
	    -serial mon:stdio \
	    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
	    -display none
