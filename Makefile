# blockz makefile

# Use bash for running shell commands.
SHELL = /bin/bash

# Make cargo only run for a certain package.
PKG ?= blockz blockz_derive
PKG_EXPANDED = $(shell for pkg in $(PKG); do printf "%s" "--package $${pkg} "; done)

# Flag that targets artifacts in release mode.
RELEASE ?= 0
RELEASE_EXPANDED = $(shell if [[ "$(RELEASE)" == "1" ]]; then printf "%s" "--release "; fi)

# Target triple that cargo should run for.
TARGET ?=
TARGET_EXPANDED = $(shell if [[ "$(TARGET)" != "" ]]; then printf "%s" "--target $(TARGET) "; fi)

# Flag that enables verbose output in cargo commands.
VERBOSE ?= 0
VERBOSE_EXPANDED = $(shell if [[ "$(VERBOSE)" == "1" ]]; then printf "%s" "--verbose "; fi)

# Flags to be passed to cargo bench.
BENCHFLAGS ?=
# Cargo bench.
CBENCH = cargo bench \
	$(VERBOSE_EXPANDED) \
	$(RELEASE_EXPANDED) \
	$(PKG_EXPANDED) \
	$(TARGET_EXPANDED) \
	$(BENCHFLAGS)

# Flags to be passed to cargo build.
BUILDFLAGS ?=
# Cargo build.
CBUILD = cargo build \
	$(VERBOSE_EXPANDED) \
	$(RELEASE_EXPANDED) \
	$(PKG_EXPANDED) \
	$(TARGET_EXPANDED) \
	$(BUILDFLAGS)

# Flags to be passed to cargo clippy.
CLIPPYFLAGS ?=
# Cargo clippy.
CCLIPPY = cargo clippy \
	$(VERBOSE_EXPANDED) \
	$(RELEASEm_EXPANDED) \
	$(PKG_EXPANDED) \
	$(TARGET_EXPANDED) \
	$(CLIPPYFLAGS)

# Flags to be passed to cargo check.
CHECKFLAGS ?=
# Cargo check.
CCHECK = cargo check \
	$(VERBOSE_EXPANDED) \
	$(RELEASE_EXPANDED) \
	$(PKG_EXPANDED) \
	$(TARGET_EXPANDED) \
	$(CHECKFLAGS)

# Flags to be passed to cargo clean.
CLEANFLAGS ?=
# Cargo clean.
CCLEAN = cargo clean \
	$(VERBOSE_EXPANDED) \
	$(RELEASE_EXPANDED) \
	$(PKG_EXPANDED) \
	$(TARGET_EXPANDED) \
	$(CLEANFLAGS)

# Flags to be passed to cargo doc.
DOCFLAGS ?=
# Cargo doc.
CDOC = cargo doc \
	$(VERBOSE_EXPANDED) \
	$(RELEASE_EXPANDED) \
	$(PKG_EXPANDED) \
	$(TARGET_EXPANDED) \
	$(DOCFLAGS)

# Flags to be passed to cargo fmt.
FMTFLAGS ?=
# Cargo fmt.
CFMT = cargo fmt \
	$(VERBOSE_EXPANDED) \
	$(RELEASE_EXPANDED) \
	$(PKG_EXPANDED) \
	$(TARGET_EXPANDED) \
	$(FMTFLAGS)

# Flags to be passed to cargo test.
TESTFLAGS ?=
# Cargo test.
CTEST = cargo test \
	$(VERBOSE_EXPANDED) \
	$(RELEASE_EXPANDED) \
	$(PKG_EXPANDED) \
	$(TARGET_EXPANDED) \
	$(TESTFLAGS)

# Default recipe.
all: clippy

bench:
	$(CBENCH)
.PHONY: bench

build:
	$(CBUILD)
.PHONY: build

clippy:
	$(CCLIPPY)
.PHONY: clippy

check:
	$(CCHECK)
.PHONY: check

clean:
	$(CCLEAN)
.PHONY: clean

doc:
	$(CDOC)
.PHONY: build

fmt:
	$(CFMT)
.PHONY: fmt

test: singleton configuration

# Tests

singleton:
	$(CTEST) --features singleton
.PHONY: singleton

configuration:
	$(CTEST) --features configuration
.PHONY: configuration
