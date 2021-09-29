# vet
vet\:check: # Check error [synonym: check]
	@cargo check --all --verbose
.PHONY: vet\:check

check: vet\:check
.PHONY: check

vet\:format: # Show format diff [synonym: vet:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: vet\:format

vet\:fmt: vet\:format
.PHONY: vet\:fmt

format: vet\:format
.PHONY: format

fmt: vet\:format
.PHONY: fmt

vet\:lint: # Show suggestions relates to hygiene [synonym: lint]
	@cargo clippy --all-targets
.PHONY: vet\:lint

lint: vet\:lint
.PHONY: lint

vet\:all: check fmt lint # Run all vet targets
.PHONY: vet\:all

vet: vet\:check # Alias for vet:check
.PHONY: vet

# test
test\:debug: # Run unit tests with debug feature
	@cargo test --features debug -- --nocapture
.PHONY: test\:debug

test: test\:debug # Alias for test:debug
.PHONY: test

test\:release: # Run unit tests without enabling debug feature
	@cargo test -- --nocapture
.PHONY: test\:release


# build
build\:debug: # Run packages [synonym: build]
	cargo build --features debug
.PHONY: build\:debug

build: build\:debug # Alias for build:debug
.PHONY: build

build\:release: # Build packages with release mode
	cargo build --release
.PHONY: build\:release

# utility
watch: # Start a process to watch (require cargo-watch)
	cargo watch --exec 'run' --delay 0.3
.PHONY: watch

clean: # Remove built artifacts
	@rm -fr dst/*.html
	@cargo clean
.PHONY: clean

help: # Display this message
	@set -uo pipefail; \
	grep --extended-regexp '^[0-9a-z\:\\\%]+: ' \
		$(firstword $(MAKEFILE_LIST)) | \
		grep --extended-regexp ' # ' | \
		sed --expression='s/\([a-z0-9\-\:\ ]*\): \([a-z0-9\-\:\ ]*\) #/\1: #/g' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": # "}; \
			{printf "\033[38;05;222m%-14s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL = build:release
default: beild\:release
