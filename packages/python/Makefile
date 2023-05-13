MAKEFILE_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))

.PHONY: all default test
all: default

default:
	cd ${MAKEFILE_DIR}
	yarn install
	CFLAGS=-std=c99 yarn build-release
	# strip ./*./node

test:
	jest
