build:
	cargo xbuild

image:
	cargo bootimage

run-interactive: image
	# cargo xrun
	qemu-system-x86_64 \
	    -drive format=raw,file=target/x86_64-unknown-raw/debug/bootimage-mtos.bin \
	    -serial file:serial.txt \
	    -device isa-debug-exit,iobase=0xf4,iosize=0x04

integration-tests:
	bootimage test

run-background: image
	qemu-system-x86_64 \
	    -drive format=raw,file=target/x86_64-unknown-raw/debug/bootimage-mtos.bin \
	    -serial mon:stdio \
	    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
	    -display none
