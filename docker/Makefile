# parameters
DOCKER ?= docker
DOCKER_BUILD_TARGET ?= compiler-dev
DOCKER_IMAGE_NAME ?= compiler-dev

# directories
TOP_DIR := $(shell pwd)
REPO_DIR := $(realpath $(TOP_DIR)/..)
KOOPAC_DIR := $(REPO_DIR)/koopac
KOOPAC_DIR_NAME := koopac

# files
KOOPAC_SRCS := $(shell find $(KOOPAC_DIR)/src -name "*.rs")
KOOPAC_SRCS += $(KOOPAC_DIR)/Cargo.toml
AUTOTEST_SRC := $(REPO_DIR)/autotest/autotest

# targets
KOOPAC := $(TOP_DIR)/koopac.tar.gz
AUTOTEST := $(TOP_DIR)/autotest

.PHONY: docker clean

docker: Dockerfile $(KOOPAC) $(AUTOTEST)
	$(DOCKER) build --target $(DOCKER_BUILD_TARGET) -t $(DOCKER_IMAGE_NAME) .

clean:
	-rm $(KOOPAC) $(AUTOTEST)

$(KOOPAC): $(KOOPAC_SRCS)
	cd .. && git archive --format=tar.gz -o $@ HEAD:$(KOOPAC_DIR_NAME)

$(AUTOTEST): $(AUTOTEST_SRC)
	cp $^ $@
