MODE ?= 1

ifeq ($(MODE), 1)
  mode := --release
  mode_dir := release
else
  mode := 
  mode_dir := debug
endif

target := x86_64-unknown-linux-musl
package_name := $(shell basename $(shell pwd))
target_dir := target/$(target)/$(mode_dir)
rustc_flags := -C debuginfo=2
ir_dir := ir

build:
	cargo build $(mode) --target $(target)

ir: build
	mkdir -p $(ir_dir)
	objdump -d $(target_dir)/$(package_name) > $(ir_dir)/$(package_name).asm
#   cargo rustc --target $(target) $(mode) -- --emit=asm
#   cp $(target_dir)/deps/$(package_name)*.s $(package_name).s
	cargo rustc --target $(target) $(mode) -- --emit=llvm-ir $(rustc_flags)
	cp $(target_dir)/deps/$(package_name)*.ll $(ir_dir)/llvm.ll
	dwarfdump $(target_dir)/$(package_name) > $(ir_dir)/debuginfo.txt
	cargo rustc --target $(target) $(mode) -- -Z unpretty=hir > $(ir_dir)/hir.rs 
	cargo rustc --target $(target) $(mode) -- -Z unpretty=mir > $(ir_dir)/mir.rs 
	
clean:
	@cargo clean
	@rm -r $(ir_dir)