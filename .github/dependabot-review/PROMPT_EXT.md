<!-- markdownlint-disable-file MD041 -->
<!-- Appended to PROMPT.md under "## Repository extension (supplied)". -->

This repository defines the gRPC API between the Linkerd proxy and the
control plane. It is a library, not a deployed service: the Rust crate
`linkerd2-proxy-api` is published to crates.io and used by `linkerd2-proxy`,
and the Go module is imported by the `linkerd2` control plane. Its bindings
are generated from `proto/` and checked in (`src/gen/`, `go/`). CI regenerates
them and fails if they differ (`rs-gen-check`, `go-gen-check`), so a bump
that changes generated output must include the regenerated files.

## Critical paths in this repo

- **Rust gRPC and protobuf codegen.** `prost`, `prost-types`, `tonic`,
  `tonic-prost`, and `tonic-prost-build` (dev) generate and back every type
  in `src/gen/`. Their types are part of this crate's public API, so a minor
  bump of the 0.x line (for example 0.14 to 0.15) is a breaking release of
  this crate and must move in step with `linkerd2-proxy`. Treat any change to
  generated code, message encoding, or `tonic` client/server traits as
  critical.
- **Rust public API types.** `http` (`src/http_types.rs` conversions to
  `http::Method` and `http::uri::Scheme`), `h2` (`From<h2::Reason>` in
  `src/tap.rs`), `ipnet` (`TryFrom<IpNetwork> for ipnet::IpNet` in
  `src/net.rs`), and `thiserror` appear in the public API under the
  `http-types`, `tap`, and `net` features. A major bump of any of them is a
  breaking change for this crate's users.
- **Rust MSRV.** `Cargo.toml` sets `rust-version = "1.90"`. A dependency
  that raises its minimum Rust version above that is breaking.
- **Go gRPC and protobuf.** `google.golang.org/grpc`,
  `google.golang.org/protobuf`, `github.com/golang/protobuf`, and the
  generators `google.golang.org/grpc/cmd/protoc-gen-go-grpc` and
  `github.com/golang/protobuf/protoc-gen-go` (pinned through `tools.go`)
  produce and back the code in `go/`. A generator bump changes generated
  files and the `versions:` header in `go/**/*_grpc.pb.go`; a runtime bump
  can raise the minimum runtime the generated code requires. Both reach the
  `linkerd2` control plane.
- **Go toolchain directive.** `go.mod` declares `go 1.26.0`. A dependency
  that forces a higher `go` directive raises the requirement for every
  module that imports this one.
- **Release workflow.** `.github/workflows/release.yml` publishes the crate
  with `CRATESIO_TOKEN` and creates GitHub releases with
  `softprops/action-gh-release` under `contents: write`. Bumps of actions in
  that workflow are on the release path.
- **Linkerd dev toolchain.** `linkerd/dev/actions/setup-tools` and the
  `ghcr.io/linkerd/dev` images (in workflows and
  `.devcontainer/devcontainer.json`) supply `protoc`, Rust, and Go. The
  `devcontainer-versions` check requires every `ghcr.io/linkerd/dev` image in
  the workflows to match the devcontainer image. Dependabot bumps only the
  `setup-tools` action, so such a bump leaves the action and images on
  different versions.
