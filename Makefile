LD = $(HOME)/lib/binutils/i686/bin/i686-elf-ld
NASM = nasm
QEMU = qemu-system-i386 -curses

RUSTC = rustc
RUSTLIB = $(HOME)/lib/rust/i686-unknown-linux-gnu/
RUSTFLAGS := -O -L $(RUSTLIB) --crate-type lib

SRCDIR = $(shell pwd)
BUILDDIR = $(SRCDIR)/build

TARGET = i686-unknown-linux-gnu


.SUFFIXES: .asm .o .rs

.PHONY: clean run


all: $(BUILDDIR) $(BUILDDIR)/rottenOS.bin

$(BUILDDIR):
	mkdir -p $(BUILDDIR)

$(BUILDDIR)/boot.asm.o: src/asm/boot.asm
	$(NASM) -f elf32 -Wall -o $@ $<

$(BUILDDIR)/rottenOS.o: src/main.rs
	$(RUSTC) $(RUSTFLAGS) --target $(TARGET) -o $@ --emit obj $<

$(BUILDDIR)/rottenOS.bin: src/linker.ld $(BUILDDIR)/rottenOS.o $(BUILDDIR)/boot.asm.o
	$(LD) -m elf_i386 -o $@ -T $^

run: $(BUILDDIR)/rottenOS.bin
	$(QEMU) -kernel $<

clean:
	rm -rf $(BUILDDIR)
