RUST=rust
RUSTC=rustc
RUSTFLAGS=-O -Z debug-info
VERSION=0.1-pre

libhttp_so=build/libhttp-20af9b1d3441fe5a-$(VERSION).so
libhttp_files=\
		      src/libhttp/lib.rs \
		      src/libhttp/buffer.rs \
		      src/libhttp/common.rs \
		      src/libhttp/generated/read_method.rs \
		      src/libhttp/generated/status.rs \
		      $(wildcard src/libhttp/headers/*.rs) \
		      $(wildcard src/libhttp/client/*.rs) \
		      $(wildcard src/libhttp/server/*.rs) \
		      src/libhttp/memstream.rs \
		      src/libhttp/method.rs \
		      src/libhttp/rfc2616.rs

all: $(libhttp_so) examples

src/libhttp/codegen/codegen: $(wildcard src/libhttp/codegen/*.rs)
	$(RUSTC) $(RUSTFLAGS) $@.rs

src/libhttp/generated/%.rs: src/libhttp/codegen/codegen
	src/libhttp/codegen/codegen $(patsubst src/libhttp/generated/%,%,$@)

$(libhttp_so): $(libhttp_files)
	mkdir -p build/
	$(RUSTC) $(RUSTFLAGS) src/libhttp/lib.rs --out-dir=build

build/%:: src/%.rs $(libhttp_so)
	mkdir -p '$(dir $@)'
	$(RUSTC) $(RUSTFLAGS) $< -o $@ -L build/

examples: build/examples/apache_fake build/examples/hello_world build/examples/info build/examples/client/client

build/tests: $(libhttp_files)
	$(RUSTC) $(RUSTFLAGS) --test -o build/tests src/libhttp/lib.rs

check: build/tests
	build/tests --test

clean:
	rm -rf src/libhttp/generated/ src/libhttp/codegen/codegen
	rm -rf build/

.PHONY: all examples clean check
