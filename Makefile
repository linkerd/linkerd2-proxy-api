PROTOC_VERSION=3.6.0
PROTOC_BASE_URL=https://github.com/google/protobuf/releases/download/v$(PROTOC_VERSION)
PROTOC=./.protoc-$(PROTOC_VERSION)
PROTOC_GO=$(PROTOC) -I proto --go_out="plugins=grpc:$(GOPATH)/src"

CARGO=cargo

CURL=curl

GO=go
DEP_VERSION=0.4.1
DEP_BASE_URL=https://github.com/golang/dep/releases/download/v$(DEP_VERSION)
DEP=./.dep-$(DEP_VERSION)

UNZIP=unzip
CHMOD=chmod
RM=rm

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S), Linux)
	PROTOC_OS=linux
	DEP_URL=$(DEP_BASE_URL)/dep-linux-amd64
	PROTOC_URL=$(PROTOC_BASE_URL)/protoc-$(PROTOC_VERSION)-linux-x86_64.zip
else
	ifeq ($(UNAME_S), Darwin)
		PROTOC_OS=osx
		DEP_URL=$(DEP_BASE_URL)/dep-darwin-amd64
		PROTOC_URL=$(PROTOC_BASE_URL)/protoc-$(PROTOC_VERSION)-osx-x86_64.zip
	else
		UNAME_O := $(shell uname -o)
		ifeq ($(UNAME_O), Msys)
			PROTOC_OS = osx
			DEP_URL=$(DEP_BASE_URL)/dep-windows-amd64.exe
			PROTOC_URL=$(PROTOC_BASE_URL)/protoc-$(PROTOC_VERSION)-win32.zip
		endif
	endif
endif

$(PROTOC):
	$(CURL) -Lso $(PROTOC).zip $(PROTOC_URL)
	$(UNZIP) -p $(PROTOC).zip bin/protoc >$(PROTOC)
	$(RM) $(PROTOC).zip
	$(CHMOD) 755 $(PROTOC)

$(DEP):
	$(CURL) -Lso $(DEP) $(DEP_URL)
	$(CHMOD) 755 $(DEP)

.PHONY: rs
rs: proto
	$(CARGO) test -p linkerd2-proxy-api

Gopkg.lock: $(DEP) Gopkg.toml
	$(DEP) ensure

vendor: Gopkg.lock
	$(GO) install ./vendor/github.com/golang/protobuf/protoc-gen-go

.PHONY: go
go: vendor proto $(PROTOC)
	$(PROTOC_GO) proto/destination.proto
	$(PROTOC_GO) proto/net.proto
	$(PROTOC_GO) proto/tap.proto

all: go rs
