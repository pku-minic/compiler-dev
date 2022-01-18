# parameters
DOCKER_IMAGE_NAME ?= compiler-dev

# directories & files
TOP_DIR := $(shell pwd)
GIT_ARCHIVE = $(TOP_DIR)/compiler-dev.tar.gz


.PHONY: docker clean

docker: Dockerfile $(GIT_ARCHIVE)
	docker build -t $(DOCKER_IMAGE_NAME) .

clean:
	-rm $(GIT_ARCHIVE)

$(GIT_ARCHIVE):
	cd .. && git archive --format=tar.gz -o $@ master
