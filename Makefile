TARGET = target/debug
ifdef CARGO_RELEASE
	RELEASE = --release
	TARGET = target/release
endif

CARGO = cargo
ifdef CARGO_VERBOSE
	CARGO = cargo --verbose
endif

CARGO_TEST = $(CARGO) test --locked $(RELEASE) --all-features

CURL = curl -s
GIT = git
GO = go
UNZIP = unzip

PROTOC_VERSION = 3.6.0
PROTOC_BASE_URL = https://github.com/google/protobuf/releases/download/v$(PROTOC_VERSION)
PROTOC = target/protoc-$(PROTOC_VERSION)
PROTOC_GO = $(PROTOC) -I proto --go_out=plugins=grpc:.

MODULE_NAME = github.com/linkerd/linkerd2-proxy-api

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S), Linux)
	PROTOC_URL=$(PROTOC_BASE_URL)/protoc-$(PROTOC_VERSION)-linux-x86_64.zip
else
	ifeq ($(UNAME_S), Darwin)
		PROTOC_URL=$(PROTOC_BASE_URL)/protoc-$(PROTOC_VERSION)-osx-x86_64.zip
	else
		UNAME_O := $(shell uname -o)
		ifeq ($(UNAME_O), Msys)
			PROTOC_URL=$(PROTOC_BASE_URL)/protoc-$(PROTOC_VERSION)-win32.zip
		endif
	endif
endif

$(PROTOC):
	mkdir -p $(TARGET)
	$(CURL) -Lo $(PROTOC).zip $(PROTOC_URL)
	$(UNZIP) -p $(PROTOC).zip bin/protoc >$(PROTOC)
	rm $(PROTOC).zip
	chmod 755 $(PROTOC)

Cargo.lock: Cargo.toml rs/Cargo.toml
	$(CARGO) fetch

.PHONY: rs
rs: Cargo.lock
	$(CARGO_TEST)

.PHONY: go
go: $(PROTOC)
	$(GO) get github.com/golang/protobuf/protoc-gen-go
	$(PROTOC_GO) proto/destination.proto
	$(PROTOC_GO) proto/http_types.proto
	$(PROTOC_GO) proto/identity.proto
	$(PROTOC_GO) proto/net.proto
	$(PROTOC_GO) proto/tap.proto
	@rm -rf go/*
	@mv $(MODULE_NAME)/go/ .
	@rm -rf github.com/
	$(GO) build ./go/...

.PHONY: check-go
check-go: go
	@test 0 -eq $(shell $(GIT) diff-index -p HEAD -- go | wc -l)

.PHONY: all
all: go rs
