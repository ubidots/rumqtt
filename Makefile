SHELL=/bin/bash
CARGO_HOME=${HOME}/cargo_homes/ubidots_rumqtt/


.PHONY: help build push all

help:
	    @echo "Makefile commands:"
	    @echo "test"
	    @echo "check"
	    @echo "all"

.DEFAULT_GOAL := all

test:
	CARGO_HOME=${CARGO_HOME} cargo test -- --nocapture

fmt:
	CARGO_HOME=${CARGO_HOME} cargo fmt

clippy:
	CARGO_HOME=${CARGO_HOME} cargo clippy

update:
	CARGO_HOME=${CARGO_HOME} cargo update

check:
	CARGO_HOME=${CARGO_HOME} cargo check --all-targets --profile=test  --workspace --all-features

check_release:
	CARGO_HOME=${CARGO_HOME} cargo check --all-targets --release  --workspace --all-features

validate_code: fmt check clippy test

fetch:
	CARGO_HOME=${CARGO_HOME} cargo fetch

build_release:
	CARGO_HOME=${CARGO_HOME} cargo build --release
