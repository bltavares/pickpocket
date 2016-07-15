CARGO := cargo
CLIPPY_COMMAND := rustup run nightly cargo clippy --release
CLIPPY_ARGS := -Dclippy

BINARIES := $(patsubst %.rs,%,$(notdir $(wildcard src/bin/*.rs)))

default: test

test:
	$(CARGO) test

BINARIES_CHECK_TARGETS := $(addprefix check-,$(BINARIES))
$(BINARIES_CHECK_TARGETS):
	$(CARGO) check --bin $(patsubst check-%,%,$@)
check-lib:
	$(CARGO) check --lib
check: | check-lib $(BINARIES_CHECK_TARGETS)

BINARIES_LINT_TARGETS := $(addprefix lint-,$(BINARIES))
$(BINARIES_LINT_TARGETS):
	$(CLIPPY_COMMAND) --bin $(patsubst lint-%,%,$@) -- $(CLIPPY_ARGS)
lint-lib:
	$(CLIPPY_COMMAND) --lib -- $(CLIPPY_ARGS)
lint: | lint-lib $(BINARIES_LINT_TARGETS)

outdated:
	$(CARGO) outdated -R

install:
	@-$(CARGO) uninstall pickpocket
	$(CARGO) install

clean:
	$(CARGO) clean

fmt:
	$(CARGO) fmt -- --write-mode overwrite

help:
	@echo "Available options:"
	@echo "  - check: Quickly validate all binaries compiles"
	@echo "  - install: Installs the project using cargo"
	@echo "  - lint: Lint all binaries against clippy"
	@echo "  - outdated: List outdated dependency information"
	@echo "  - test: Run cargo test"

.PHONY: help test lint check outdated $(BINARIES_CHECK_TARGETS) $(BINARIES_LINT_TARGETS) install clean
