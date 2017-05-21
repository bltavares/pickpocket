CARGO := cargo
CLIPPY_COMMAND := cargo +nightly clippy

BINARIES := $(patsubst %.rs,%,$(notdir $(wildcard src/bin/*.rs)))
BINARIES_CHECK_TARGETS := $(addprefix check-,$(BINARIES))
BINARIES_LINT_TARGETS := $(addprefix lint-,$(BINARIES))

default: test

.PHONY: test # Run cargo test
test:
	$(CARGO) test

.PHONY: $(BINARIES_CHECK_TARGETS)
$(BINARIES_CHECK_TARGETS):
	$(CARGO) check --bin $(patsubst check-%,%,$@)

.PHONY: check-lib
check-lib:
	$(CARGO) check --lib

.PHONY: check # Quickly validate all binaries compiles
check: | check-lib $(BINARIES_CHECK_TARGETS)

.PHONY: lint # Lint all binaries against clippy
lint:
	$(CLIPPY_COMMAND)

.PHONY: outdated # List outdated dependency information
outdated:
	$(CARGO) outdated -R

.PHONY: install # Installs the project using cargo
install:
	@-$(CARGO) uninstall pickpocket
	$(CARGO) install

.PHONY: clean # Cleanup older compilation results
clean:
	$(CARGO) clean

.PHONY: fmt # Formats the source files using rustfmt
fmt:
	$(CARGO) fmt -- --write-mode overwrite

.PHONY: help # Shows the acailable tasks
help:
	@echo "Available options:"
	@grep '^.PHONY: [^#]\+ #' Makefile | cut -d: -f2- | sed 's/#/-/' | sort
