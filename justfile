# See https://just.systems/man/

##
## General recipes
##

default: rs-fetch rs-deny gen rs-check-fmt rs-clippy rs-docs rs-test

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
export RUSTFLAGS := env_var_or_default("RUSTFLAGS", "-D warnings -A deprecated")

cargo_toolchain := ""
cargo := "cargo" + if cargo_toolchain != "" { " +" + cargo_toolchain } else { "" }

# If we're running in Github Actions and cargo-action-fmt is installed, then add
# a command suffix that formats errors.
_fmt_cargo := if env_var_or_default("GITHUB_ACTIONS", "") != "true" { "" } else {
    ```
    if command -v cargo-action-fmt >/dev/null 2>&1; then
        echo "--message-format=json | cargo-action-fmt"
    fi
    ```
}

# Fetch Rust dependencies
rs-fetch:
    {{ cargo }} fetch --locked

# Check Rust code formatting
rs-check-fmt:
    {{ cargo }} fmt -- --check

# Check Rust code compilation
rs-check *flags:
    {{ cargo }} check --frozen --all-targets {{ flags }} {{ _fmt_cargo }}

# Lint Rust code
rs-clippy *flags:
    {{ cargo }} clippy --frozen --all-targets {{ flags }} {{ _fmt_cargo }}

# Audit Rust dependencies with `cargo-deny`
rs-deny *args:
    {{ cargo }} deny --all-features check {{ args}}

# Generate Rust documentation for this crate.
rs-docs:
    {{ cargo }} doc --frozen --no-deps --all-features {{ _fmt_cargo }}

# Generate Rust bindings from protobuf.
rs-gen:
    {{ cargo }} run --frozen --example=gen

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
    {{ cargo }} test --no-run --frozen {{ flags }} {{ _fmt_cargo }}

# Run Rust tests.
rs-test *flags:
    {{ cargo }} test --frozen {{ flags }}

# Public the Rust crate to crates.io.
rs-publish *flags:
    {{ cargo }} publish --all-features {{ flags }}

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

# Print all (transitive) dependencies of the specified go package.
go-mod-tree root='github.com/linkerd/linkerd2-proxy-api':
    #!/usr/bin/env bash
    set -eu
    declare -r GRAPH=$(go mod graph)
    deps_of() {
        local pkg=$1
        echo "$GRAPH" | awk '$1 == "'"$pkg"'" { print $2 }' | sort | uniq
    }
    declare -A PKGS=()
    subtree() {
        local pkg=$1
        local namepfx=${2:-}
        local pfx=${3:-}
        # If the package has already been printed, then print it with an
        # asterisk and skip printing its dependencies
        if (( ${PKGS[$pkg]:-0} )); then
            echo "$namepfx$pkg (*)"
        else
            # Otherwise, print the package, mark it as printed, and the
            # dependency tree for each of its depndencies
            echo "$namepfx$pkg"
            PKGS[$pkg]=1
            local dep=''
            for d in $(deps_of "$pkg") ; do
                if [ -n "$dep" ]; then subtree "$dep" "$pfx├── " "$pfx│   " ; fi
                dep=$d
            done
            if [ -n "$dep" ]; then subtree "$dep" "$pfx└── " "$pfx    " ; fi
        fi
    }
    if [[ '{{ root }}' == *@* ]]; then
        # The root specifies an exact version, so print its dependencies.
        subtree '{{ root }}'
    elif [ -n "$(echo "$GRAPH" | awk '$1 == "{{ root }}" { print $0 }' | head -n 1)" ]; then
        # The root does not specify an exact version, but that is how the
        # package is listed in go.mod.
        subtree '{{ root }}'
    else
        # The root does not specify an exact version, find all versions of the
        # package and print their depdencies.
        first=1
        for pkg in $(echo "$GRAPH" | awk '{ print $1 }' | sort | uniq) ; do
            if [[ "$pkg" == '{{ root }}'@* ]]; then
                if (( $first )); then first=0; else echo; fi
                subtree "$pkg"
            fi
        done
    fi

# Print all (transitive) dependenents of the specified go package.
go-mod-why dep:
    #!/usr/bin/env bash
    set -eu
    declare -r GRAPH=$(go mod graph)
    depends_on() {
        local pkg=$1
        echo "$GRAPH" | awk '$2 == "'"$pkg"'" { print $1 }' | sort | uniq
    }
    declare -A PKGS=()
    supertree() {
        local pkg=$1
        local namepfx=${2:-}
        local pfx=${3:-}
        if (( ${PKGS[$pkg]:-0} )); then
            echo "$namepfx$pkg (*)"
        else
            PKGS[$pkg]=1
            echo "$namepfx$pkg"
            local parent=''
            for p in $(depends_on "$pkg") ; do
                if [ -n "$parent" ]; then supertree "$parent" "$pfx├── " "$pfx│   " ; fi
                parent=$p
            done
            if [ -n "$parent" ]; then supertree "$parent" "$pfx└── " "$pfx    " ; fi
        fi
    }
    if [[ '{{ dep }}' == *@* ]]; then
        # The dependency specifies an exact version, so print the packages that
        # depend on it.
        supertree '{{ dep }}'
    else
        # The dependency does not specify an exact version, find all versions of
        # the package and print the packages that depend on each of them.
        first=1
        for pkg in $(echo "$GRAPH" | awk '{ print $1 }' | sort | uniq) ; do
            if [[ "$pkg" == '{{ dep }}'@* ]]; then
                if (( $first )); then first=0; else echo; fi
                supertree "$pkg"
            fi
        done
    fi

# Print all versions of the specified go package.
go-mod-versions dep:
    #!/usr/bin/env bash
    set -eu
    if [[ '{{ dep }}' == *@* ]]; then
        echo 'The dependency must not specify an exact version.' >&2
        exit 1
    fi
    for pkg in $(go mod graph | awk '{ print $1; print $2 }' | sort | uniq) ; do
        if [[ "$pkg" == '{{ dep }}'@* ]]; then
            echo "$pkg"
        fi
    done

# vim: set ft=make :
