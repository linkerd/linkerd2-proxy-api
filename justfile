# See https://just.systems/man/

export RUST_BACKTRACE := env_var_or_default("RUST_BACKTRACE", "short")
export RUSTFLAGS := env_var_or_default("RUSTFLAGS", "-D warnings -A deprecated")
export PROTOC_NO_VENDOR := "1"

toolchain := ""
cargo := "cargo" + if toolchain != "" { " +" + toolchain } else { "" }

# If we're running in Github Actions and cargo-action-fmt is installed, then add
# a command suffix that formats errors.
_fmt := if env_var_or_default("GITHUB_ACTIONS", "") != "true" { "" } else {
    ```
    if command -v cargo-action-fmt >/dev/null 2>&1; then
        echo "--message-format=json | cargo-action-fmt"
    fi
    ```
}

default: rs-fetch rs-check-fmt rs-deny rs-clippy rs-docs rs-test go-check

rs-fetch:
    {{ cargo }} fetch --locked

rs-check-fmt:
    {{ cargo }} fmt -- --check

rs-check *flags:
    {{ cargo }} clippy --frozen --all-targets {{ flags }} {{ _fmt }}

rs-clippy *flags:
    {{ cargo }} clippy --frozen --all-targets {{ flags }} {{ _fmt }}

rs-deny:
    {{ cargo }} deny --all-features check

rs-docs:
    {{ cargo }} doc --frozen --no-deps --all-features {{ _fmt }}

rs-test-build *flags:
    {{ cargo }} test --no-run --frozen {{ flags }} {{ _fmt }}

rs-test *flags:
    {{ cargo }} test --frozen {{ flags }}

rs-publish *flags:
    {{ cargo }} publish --all-features {{ flags }}

go-setup:
    if ! command -v protoc >/dev/null 2>&1; then echo "protoc not found" >&2 ; exit 1 ; fi
    go mod download
    go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.28
    go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@v1.2

go-gen: go-setup
    @rm -rf go/*
    mkdir -p ./go/destination ./go/http_types ./go/identity ./go/inbound ./go/net ./go/tap
    protoc -I proto --go_out=paths=source_relative:./go/destination proto/destination.proto
    protoc -I proto --go_out=paths=source_relative:./go/http_types proto/http_types.proto
    protoc -I proto --go_out=paths=source_relative:./go/identity proto/identity.proto
    protoc -I proto --go_out=paths=source_relative:./go/inbound proto/inbound.proto
    protoc -I proto --go_out=paths=source_relative:./go/net proto/net.proto
    protoc -I proto --go_out=paths=source_relative:./go/tap proto/tap.proto
    protoc -I proto --go-grpc_out=paths=source_relative:./go/destination proto/destination.proto
    protoc -I proto --go-grpc_out=paths=source_relative:./go/inbound proto/inbound.proto
    protoc -I proto --go-grpc_out=paths=source_relative:./go/identity proto/identity.proto
    protoc -I proto --go-grpc_out=paths=source_relative:./go/tap proto/tap.proto
    go build ./go/...

go-check: go-gen
    #!/usr/bin/env sh
    if [ $(git diff-index -p HEAD -- go | wc -l) -gt 0 ]; then
      echo 'go files are not up to date' >&2
      git diff-index -p HEAD -- go >&2
      exit 1
    fi

# vim: set ft=make :
