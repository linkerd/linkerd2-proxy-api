TARGET = target/debug
ifdef CARGO_RELEASE
	RELEASE = --release
	TARGET = target/release
endif

CARGO = cargo
ifdef CARGO_VERBOSE
	CARGO = cargo --verbose
endif

CURL = curl -s
GIT = git
GO = go
UNZIP = unzip

PROTOC_VERSION := 3.20.0
PROTOC_BASE_URL = https://github.com/google/protobuf/releases/download/v$(PROTOC_VERSION)
export PROTOC ?= target/protoc-$(PROTOC_VERSION)
export PROTOC_NO_VENDOR := 1

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
	$(CURL) -fsSLo $(PROTOC).zip $(PROTOC_URL)
	$(UNZIP) -p $(PROTOC).zip bin/protoc >$(PROTOC)
	rm $(PROTOC).zip
	chmod 755 $(PROTOC)

.PHONY: fetch
fetch: Cargo.toml
	$(CARGO) fetch

.PHONY: rs
rs: fetch $(PROTOC)
	cargo check --all-features --frozen $(RELEASE)

.PHONY: clippy
clippy: fetch $(PROTOC)
	for api in destination http_types identity inbound net tap ; do \
		for kind in arbitrary client server ; do \
			$(CARGO) clippy --frozen $(RELEASE) --features=$$api,$$kind --all-targets ; \
		done ; \
	done

.PHONY: go
go: $(PROTOC)
	@rm -rf go/*
	mkdir -p ./go/destination ./go/http_types ./go/identity ./go/inbound ./go/net ./go/tap
	$(GO) mod download
	$(GO) install google.golang.org/protobuf/cmd/protoc-gen-go@v1.28
	$(GO) install google.golang.org/grpc/cmd/protoc-gen-go-grpc@v1.2
	$(PROTOC) -I proto --go_out=paths=source_relative:./go/destination proto/destination.proto
	$(PROTOC) -I proto --go_out=paths=source_relative:./go/http_types proto/http_types.proto
	$(PROTOC) -I proto --go_out=paths=source_relative:./go/identity proto/identity.proto
	$(PROTOC) -I proto --go_out=paths=source_relative:./go/inbound proto/inbound.proto
	$(PROTOC) -I proto --go_out=paths=source_relative:./go/net proto/net.proto
	$(PROTOC) -I proto --go_out=paths=source_relative:./go/tap proto/tap.proto
	$(PROTOC) -I proto --go-grpc_out=paths=source_relative:./go/destination proto/destination.proto
	$(PROTOC) -I proto --go-grpc_out=paths=source_relative:./go/inbound proto/inbound.proto
	$(PROTOC) -I proto --go-grpc_out=paths=source_relative:./go/identity proto/identity.proto
	$(PROTOC) -I proto --go-grpc_out=paths=source_relative:./go/tap proto/tap.proto
	$(GO) build ./go/...

.PHONY: check-go
check-go: go
	$(GIT) diff-index -p HEAD -- go
	@test 0 -eq $(shell $(GIT) diff-index -p HEAD -- go | wc -l)

.PHONY: all
all: go rs
