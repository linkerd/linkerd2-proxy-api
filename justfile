# See https://just.systems/man/

##
## General recipes
##

default: rs-fetch rs-deny gen lint rs-test

lint: md-lint gen-check rs-clippy rs-docs

md-lint:
    markdownlint-cli2 '**/*.md' '!**/node_modules' '!**/target'

# Generate Go & Rust bindings from protobuf.
gen: rs-gen go-gen

# Regenerate bindings and error if they don't match what is already in version
# control.
gen-check: rs-gen-check go-gen-check

# The `protoc` binary to use for both Go & Rust.
export PROTOC := env_var_or_default("PROTOC", "protoc")

# Used by Rust's `prost` to disallow compiling protoc.
export PROTOC_NO_VENDOR := "1"

##
## Rust recipes
##

export RUST_BACKTRACE := env_var_or_default("RUST_BACKTRACE", "short")

cargo-toolchain := ""
_cargo := 'just-cargo' + if cargo-toolchain != '' { ' toolchain=' + cargo-toolchain } else { '' }

features := "all"
_features := if features == "all" { "--all-features" } else { "--features=" + features }

# Fetch Rust dependencies
rs-fetch:
    {{ _cargo }} fetch --locked

# Check Rust code formatting
rs-check-fmt:
    {{ _cargo }} fmt -- --check

# Check Rust code compilation
rs-check *flags:
    {{ _cargo }} check --all-targets {{ _features }} {{ flags }}

alias clippy := rs-clippy

# Lint Rust code
rs-clippy *flags:
    {{ _cargo }} clippy --all-targets {{ _features }} {{ flags }}

# Audit Rust dependencies with `cargo-deny`
rs-deny *args:
    cargo-deny {{ _features }} check {{ args}}

# Generate Rust documentation for this crate.
rs-docs:
    {{ _cargo }} doc --no-deps {{ _features }}

# Generate Rust bindings from protobuf.
rs-gen:
    cargo run --example=gen

# Regenerate Rust bindings and error if they don't match what is already in
# version control.
rs-gen-check: rs-gen
    #!/usr/bin/env sh
    if [ $(git diff-index -p HEAD -- src/gen | wc -l) -gt 0 ]; then
        echo 'rust bindings are not up to date' >&2
        git diff-index -p HEAD -- src/gen >&2
        exit 1
    fi

# Build Rust tests.
rs-test-build *flags:
    {{ _cargo }} test-build {{ _features }} {{ flags }}

# Run Rust tests.
rs-test *flags:
    {{ _cargo }} test {{ _features }} {{ flags }}

# Public the Rust crate to crates.io.
rs-publish *flags:
    cargo publish {{ _features }} {{ flags }}

##
## Go recipes
##

# Fetch Go dependencies.
go-mod:
    go mod tidy
    go mod download

# Errors if `go mod tidy` changes the manifests
go-mod-check: go-mod
    #!/usr/bin/env sh
    if [ $(git diff-index -p HEAD -- go.{mod,sum} | wc -l) -gt 0 ]; then
        echo 'go.mod can be tidied' >&2
        git diff-index -p HEAD -- go.{mod,sum} >&2
        exit 1
    fi

# Install go tools needed to generate bindings.
go-tools:
    @sed -nE 's/\s+_ "(.*)"/\1/p' tools.go | xargs -t go install

# Generate Go bindings from protobuf.
go-gen: go-tools
    #!/usr/bin/env bash
    set -eu
    # Delete the directory to ensure module deletions are honored.
    echo rm -rf go
    rm -rf go
    # Find non-vendored protobuf files in the `proto` dir.
    for path in $(find proto -maxdepth 2 -name '*.proto'); do
        # Extract the name of the module.
        name="${path%.proto}"
        name="${name#proto/}"
        # Create a directory for the module.
        echo mkdir -p go/$name
        mkdir -p go/$name
        # Generate basic protobuf bindings for the module.
        echo {{ PROTOC }} -I proto --go_out=paths=source_relative:./go/$name $path
        {{ PROTOC }} -I proto --go_out=paths=source_relative:./go/$name $path
        # If the protobuf includes a service definition, generate gRPC bindings as well.
        if grep -q '^service ' "$path" >/dev/null; then
            echo {{ PROTOC }} -I proto --go-grpc_out=paths=source_relative:./go/$name $path
            {{ PROTOC }} -I proto --go-grpc_out=paths=source_relative:./go/$name $path
        fi
    done

go-build:
    go build ./go/...

# Regenerate Go bindings and error if they don't match what is already in
# version control.
go-gen-check: go-gen
    #!/usr/bin/env sh
    if [ $(git diff-index -p HEAD -- go | wc -l) -gt 0 ]; then
        echo 'go bindings are not up to date' >&2
        git diff-index -p HEAD -- go >&2
        exit 1
    fi

# vim: set ft=make :
