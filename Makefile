CLIPPY_COMMAND=rustup run nightly cargo clippy --release
CLIPPY_ARGS=-Dclippy

BINARIES=$(patsubst %.rs,%,$(notdir $(wildcard src/bin/*.rs)))
BINARIES_LINT_TARGETS=$(addprefix lint-,$(BINARIES))
BINARIES_CHECK_TARGETS=$(addprefix check-,$(BINARIES))

test:
	cargo test

$(BINARIES_CHECK_TARGETS):
	cargo check --bin $(patsubst check-%,%,$@)
check-lib:
	cargo check --lib
check: | check-lib $(BINARIES_CHECK_TARGETS)

$(BINARIES_LINT_TARGETS):
	$(CLIPPY_COMMAND) --bin $(patsubst lint-%,%,$@) -- $(CLIPPY_ARGS)
lint-lib:
	$(CLIPPY_COMMAND) --lib -- $(CLIPPY_ARGS)
lint: | lint-lib $(BINARIES_LINT_TARGETS)

install:
	@-cargo uninstall pickpocket
	cargo install

help:
	@echo "Available options:"
	@echo "  - test: Run cargo test"
	@echo "  - check: Quickly validate all binaries compiles"
	@echo "  - lint: Lint all binaries against clippy"
	@echo "  - install: Installs the project using cargo"

.PHONY: help test lint check $(BINARIES_CHECK_TARGETS) $(BINARIES_LINT_TARGETS)
