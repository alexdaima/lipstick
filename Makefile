MAKEFILE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))

.PHONY: all default update
all: default

default: python

setup:
	yarn global add jest jest-cli
	cargo install tree-sitter-cli

python:
	cp ${MAKEFILE_DIR}/submodules/nvim-treesitter/queries/python/highlights.scm ${MAKEFILE_DIR}/parsers/python/queries/highlights.scm
	cp ${MAKEFILE_DIR}/submodules/nvim-treesitter/queries/python/injections.scm ${MAKEFILE_DIR}/parsers/python/queries/injections.scm
	cp ${MAKEFILE_DIR}/submodules/nvim-treesitter/queries/python/locals.scm ${MAKEFILE_DIR}/parsers/python/queries/locals.scm
	cp ${MAKEFILE_DIR}/submodules/tree-sitter-python/queries/tags.scm ${MAKEFILE_DIR}/parsers/python/queries/tags.scm
	cp ${MAKEFILE_DIR}/submodules/tree-sitter-python/grammar.js ${MAKEFILE_DIR}/parsers/python/grammar.js
	cp ${MAKEFILE_DIR}/submodules/tree-sitter-python/src/grammar.json ${MAKEFILE_DIR}/parsers/python/src/grammar.json
	cp ${MAKEFILE_DIR}/submodules/tree-sitter-python/src/node-types.json ${MAKEFILE_DIR}/parsers/python/src/node-types.json
	cp ${MAKEFILE_DIR}/submodules/tree-sitter-python/src/scanner.cc ${MAKEFILE_DIR}/parsers/python/src/scanner.cc
	cd ${MAKEFILE_DIR}/parsers/python && tree-sitter generate
	cd ${MAKEFILE_DIR}/packages/python && make
	cd ${MAKEFILE_DIR}/packages/python && make test

