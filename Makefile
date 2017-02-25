DOCKERFILES := $(wildcard docker/Dockerfile.*)
IMAGE_TARGETS := $(addprefix .target/,$(addsuffix .image,$(DOCKERFILES:docker/Dockerfile.%=%)))
PUSH_TARGETS := $(addprefix .target/,$(addsuffix .push,$(DOCKERFILES:docker/Dockerfile.%=%)))

.DEFAULT: all
.PHONY: all push-all

all: $(IMAGE_TARGETS)
push-all: $(PUSH_TARGETS)

image-%: .target/%.image
push-%: .target/%.push

.target/%.image: docker/install.sh docker/Dockerfile.%
	docker build -t dflemstr/rust-buildenv:$(notdir $(@:.image=)) -f docker/Dockerfile.$(notdir $(@:.image=)) docker
	mkdir -p .target
	touch $(@)

.target/%.push: .target/%.image
	docker push dflemstr/rust-buildenv:$(notdir $(@:.push=))
	mkdir -p .target
	touch $(@)
