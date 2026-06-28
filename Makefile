PORT := 3000
TRUNK_VERSION := 0.22.0-beta.1
CARGO := $(shell rustup which cargo)
RUSTC := $(shell rustup which rustc)
TRUNK ?= trunk
WASM_CC ?= $(shell if [ -x /opt/homebrew/opt/llvm/bin/clang ]; then printf '%s' /opt/homebrew/opt/llvm/bin/clang; elif [ -x /usr/local/opt/llvm/bin/clang ]; then printf '%s' /usr/local/opt/llvm/bin/clang; else command -v clang; fi)
WASM_AR ?= $(shell if [ -x /opt/homebrew/opt/llvm/bin/llvm-ar ]; then printf '%s' /opt/homebrew/opt/llvm/bin/llvm-ar; elif [ -x /usr/local/opt/llvm/bin/llvm-ar ]; then printf '%s' /usr/local/opt/llvm/bin/llvm-ar; else command -v ar; fi)
CARGO_ENV := env CC_wasm32_unknown_unknown=$(WASM_CC) AR_wasm32_unknown_unknown=$(WASM_AR)
TRUNK_ENV := env -u NO_COLOR CC_wasm32_unknown_unknown=$(WASM_CC) AR_wasm32_unknown_unknown=$(WASM_AR) CARGO=$(CARGO) RUSTC=$(RUSTC)

.PHONY: build check dev install

check:
	$(CARGO_ENV) RUSTC=$(RUSTC) $(CARGO) check --target wasm32-unknown-unknown

build:
	$(TRUNK_ENV) $(TRUNK) build --color never

dev:
	$(TRUNK_ENV) $(TRUNK) serve --address 127.0.0.1 --port $(PORT) --color never

install:
	$(CARGO) install trunk --version $(TRUNK_VERSION) --locked
