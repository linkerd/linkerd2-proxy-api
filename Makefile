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
PROTOC_GO = $(PROTOC) -I proto --go_out="plugins=grpc:."

MODULE_NAME = github.com/linkerd/linkerd2-proxy-api

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S), Linux)
	DEP_URL=$(DEP_BASE_URL)/dep-linux-amd64
	PROTOC_URL=$(PROTOC_BASE_URL)/protoc-$(PROTOC_VERSION)-linux-x86_64.zip
else
	ifeq ($(UNAME_S), Darwin)
		DEP_URL=$(DEP_BASE_URL)/dep-darwin-amd64
		PROTOC_URL=$(PROTOC_BASE_URL)/protoc-$(PROTOC_VERSION)-osx-x86_64.zip
	else
		UNAME_O := $(shell uname -o)
		ifeq ($(UNAME_O), Msys)
			DEP_URL=$(DEP_BASE_URL)/dep-windows-amd64.exe
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

$(CHECK_GO_SETUP):
	rm -rf go/*
	cp -r $(MODULE_NAME)/go/* go/

Cargo.lock: Cargo.toml rs/Cargo.toml
	$(CARGO) fetch

.PHONY: rs
rs: Cargo.lock
	$(CARGO_TEST)

.PHONY: go-build
go-build:
	$(GO) build ./...

.PHONY: go
go: go-build $(PROTOC)
	$(GO) install github.com/golang/protobuf/protoc-gen-go
	$(PROTOC_GO) proto/destination.proto
	$(PROTOC_GO) proto/http_types.proto
	$(PROTOC_GO) proto/identity.proto
	$(PROTOC_GO) proto/net.proto
	$(PROTOC_GO) proto/tap.proto
	@rm -rf github.com/

.PHONY: check-go
check-go: go $(CHECK_GO_SETUP)
	@test 0 -eq $(shell $(GIT) diff-index -p HEAD -- go |wc -l)

.PHONY: all
all: go rs
