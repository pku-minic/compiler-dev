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
KOOPAC_NAME := koopac.tar.gz
KOOPAC := $(TOP_DIR)/$(KOOPAC_NAME)
AUTOTEST_NAME := autotest
AUTOTEST := $(TOP_DIR)/$(AUTOTEST_NAME)
BUILD_CONTEXT := $(TOP_DIR)/update-alternatives-clang.sh
BUILD_CONTEXT += $(KOOPAC) $(AUTOTEST)

# docker
DOCKER_BUILD_FLAGS := --target $(DOCKER_BUILD_TARGET) -t $(DOCKER_IMAGE_NAME)
DOCKER_BUILD_FLAGS += --build-arg KOOPAC=$(KOOPAC_NAME)
DOCKER_BUILD_FLAGS += --build-arg AUTOTEST=$(AUTOTEST_NAME)

.PHONY: docker clean

docker: $(TOP_DIR)/Dockerfile $(BUILD_CONTEXT)
	$(DOCKER) build $(DOCKER_BUILD_FLAGS) .

clean:
	-rm $(KOOPAC) $(AUTOTEST)

$(KOOPAC): $(KOOPAC_SRCS)
	cd .. && git archive --format=tar.gz -o $@ HEAD:$(KOOPAC_DIR_NAME)

$(AUTOTEST): $(AUTOTEST_SRC)
	cp $^ $@
