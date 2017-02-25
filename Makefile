DOCKERFILES := $(wildcard docker/Dockerfile.*)
IMAGE_TARGETS := $(addprefix .target/,$(addsuffix .image,$(DOCKERFILES:docker/Dockerfile.%=%)))
PUSH_TARGETS := $(addprefix .target/,$(addsuffix .push,$(DOCKERFILES:docker/Dockerfile.%=%)))

.DEFAULT: all
.PHONY: all

all: $(IMAGE_TARGETS)
push-all: $(PUSH_TARGETS)

.target/%.image: docker/install.sh docker/Dockerfile.%
	docker build -t dflemstr/rust-buildenv:$(notdir $(@:.image=)) -f docker/Dockerfile.$(notdir $(@:.image=)) docker

.target/%.push: .target/%.image
	docker push dflemstr/rust-buildenv:$(notdir $(@:.push=))
