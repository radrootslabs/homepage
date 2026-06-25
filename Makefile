PORT := 3000
TRUNK_VERSION := 0.22.0-beta.1
CARGO := $(shell rustup which cargo)
RUSTC := $(shell rustup which rustc)
TRUNK ?= trunk
TRUNK_ENV := env -u NO_COLOR CARGO=$(CARGO) RUSTC=$(RUSTC)

.PHONY: build check dev install

check:
	RUSTC=$(RUSTC) $(CARGO) check --target wasm32-unknown-unknown

build:
	$(TRUNK_ENV) $(TRUNK) build --color never

dev:
	$(TRUNK_ENV) $(TRUNK) serve --address 127.0.0.1 --port $(PORT) --color never

install:
	$(CARGO) install trunk --version $(TRUNK_VERSION) --locked
