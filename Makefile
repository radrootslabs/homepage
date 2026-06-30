PORT := 3000
TRUNK_VERSION := 0.22.0-beta.1
ENV_FILE := .env
KIT_STATE_DIR := src/components/ui/_kit
CARGO := $(shell rustup which cargo)
RUSTC := $(shell rustup which rustc)
TRUNK ?= trunk
WASM_CC ?= $(shell if [ -x /opt/homebrew/opt/llvm/bin/clang ]; then printf '%s' /opt/homebrew/opt/llvm/bin/clang; elif [ -x /usr/local/opt/llvm/bin/clang ]; then printf '%s' /usr/local/opt/llvm/bin/clang; else command -v clang; fi)
WASM_AR ?= $(shell if [ -x /opt/homebrew/opt/llvm/bin/llvm-ar ]; then printf '%s' /opt/homebrew/opt/llvm/bin/llvm-ar; elif [ -x /usr/local/opt/llvm/bin/llvm-ar ]; then printf '%s' /usr/local/opt/llvm/bin/llvm-ar; else command -v ar; fi)
CARGO_ENV := env CC_wasm32_unknown_unknown=$(WASM_CC) AR_wasm32_unknown_unknown=$(WASM_AR)
TRUNK_ENV := env -u NO_COLOR CC_wasm32_unknown_unknown=$(WASM_CC) AR_wasm32_unknown_unknown=$(WASM_AR) CARGO=$(CARGO) RUSTC=$(RUSTC)
LOAD_ENV := set -eu; set -a; . ./$(ENV_FILE); set +a
LEPTOS_UI_KIT := "$${LEPTOS_UI_KIT_ROOT}/bin/leptos_ui_kit"

.PHONY: build check dev install kit-plan kit-apply kit-check kit-migrate-state

check:
	$(CARGO_ENV) RUSTC=$(RUSTC) $(CARGO) check --target wasm32-unknown-unknown

build:
	$(TRUNK_ENV) $(TRUNK) build --color never

dev:
	$(TRUNK_ENV) $(TRUNK) serve --address 127.0.0.1 --port $(PORT) --color never

install:
	RUSTC=$(RUSTC) $(CARGO) install trunk --version $(TRUNK_VERSION) --locked
	@$(LOAD_ENV); RUSTC=$(RUSTC) $(CARGO) install \
		--git "$${LEPTOS_UI_KIT_GIT_URL}" \
		--rev "$${LEPTOS_UI_KIT_REV}" \
		--locked \
		--force \
		--root "$${LEPTOS_UI_KIT_ROOT}" \
		"$${LEPTOS_UI_KIT_PACKAGE}"

kit-plan:
	@$(LOAD_ENV); $(LEPTOS_UI_KIT) info --json
	@$(LOAD_ENV); $(LEPTOS_UI_KIT) sync --dry-run --json

kit-apply:
	@$(LOAD_ENV); $(LEPTOS_UI_KIT) sync
	@$(LOAD_ENV); $(LEPTOS_UI_KIT) doctor --strict --json

kit-check:
	@$(LOAD_ENV); $(LEPTOS_UI_KIT) sync --dry-run --json
	@$(LOAD_ENV); $(LEPTOS_UI_KIT) doctor --strict --json

kit-migrate-state:
	@$(LOAD_ENV); $(LEPTOS_UI_KIT) migrate state-dir $(KIT_STATE_DIR)
