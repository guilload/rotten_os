LD = $(HOME)/lib/binutils/i686/bin/i686-elf-ld
NASM = nasm
QEMU = qemu-system-i386 -curses

RUSTC = rustc
RUSTCLIB = $(HOME)/lib/rust/i686-unknown-linux-gnu
RUSTCFLAGS := -O -L $(RUSTCLIB) --crate-type lib
RUSTCTARGET = i686-unknown-linux-gnu

LIBCORE = $(RUSTCLIB)/core.o
RLIBC = $(RUSTCLIB)/rlibc.o

SRCDIR = $(shell pwd)
BUILDDIR = $(SRCDIR)/build

ASMFILES = $(wildcard src/asm/*.asm)
ASMOBJECTS = $(patsubst src/asm/%.asm,$(BUILDDIR)/%.asm.o,$(ASMFILES))

OBJECTS = $(ASMOBJECTS) $(BUILDDIR)/rottenOS.o


.SUFFIXES: .asm .o .rs

.PHONY: clean run


all: clean $(BUILDDIR) $(BUILDDIR)/rottenOS.bin

$(BUILDDIR):
	mkdir -p $(BUILDDIR)

$(BUILDDIR)/%.asm.o: src/asm/%.asm
	$(NASM) -f elf32 -Wall -o $@ $<

$(BUILDDIR)/rottenOS.o: src/main.rs
	$(RUSTC) $(RUSTCFLAGS) --target $(RUSTCTARGET) -o $@ --emit obj $<

$(BUILDDIR)/rottenOS.bin: src/linker.ld $(OBJECTS) $(LIBCORE) $(RLIBC)
	$(LD) -m elf_i386 -o $@ -T $^

clean:
	rm -rf $(BUILDDIR)

debug: $(BUILDDIR)/rottenOS.bin
	$(QEMU) -s -S -kernel $<

run: $(BUILDDIR)/rottenOS.bin
	$(QEMU) -d int -D /tmp/qemu.log -kernel $<
