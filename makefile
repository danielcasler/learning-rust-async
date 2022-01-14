# ==============================================================================
# Modules support
SHELL := /bin/bash

help:
	cd target/debug && \
	./wickedspeech --help

run:
	cd target/debug && \
	./wickedspeech

.PHONY: help run
