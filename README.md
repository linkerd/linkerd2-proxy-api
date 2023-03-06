# Linkerd Proxy API

This repo contains the gRPC bindings that the [Linkerd Proxy][proxy-gh] uses to
communicate with the [Linkerd control plane][cp-gh].

## APIs

Generally, the proxy's APIs are Kubernetes-agnostic and expose abstractions that
allow proxies to discover runtime configuration.

### `destination`

The destination API is used by proxies to discover information about outbound
traffic. This configuration includes:

* the protocol of the destination, if known
* whether the destination is a load balanced service or individual endpoint
* labels to describe the destination in telemetry
* the mTLS identity of destination pods

### `identity`

The identity API is used by proxies to obtain TLS certificates used for
authenticed pod-to-pod communication.

### `inbound`

The inbound API is used by the proxy to discover inbound serving
policies--especially per-port authorization requirements.

### `tap`

The proxy can be configured to expose a gRPC server that allows the control
plane to query metadata about live requests transiting the proxy.

## Languages

### Protobuf

The [`./proto`](./proto) directory includes protobuf definitions.

### Go

The [`./go`](./go) directory contains statically generated Go bindings, which are
generally used by [controller implementations][cp-gh].

### Rust

[![Crates.io][rs-crate-badge]][rs-crate-url]
[![Documentation][rs-docs-badge]][rs-docs-url]
[![License][lic-badge]](LICENSE)

This repository publishes the [**linkerd2-proxy-api** crate][rs-crate-url],
which uses [`tonic`][tonic] to expose client and server implementations for each
API. Each API may be enabled independently with cargo feature flags.

The [proxy][proxy-gh] generally uses API clients. Some server implementations
are also used by the [control plane][cp-gh].

## Issues

Issues may be opened in the [**linkerd2** repository][new-issue].

## License

    Copyright 2022 The Linkerd Authors

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

<!-- refs -->
[cp-gh]: https://github.com/linkerd/linkerd2
[new-issue]: https://github.com/linkerd/linkerd2/issues/new/choose
[proxy-gh]: https://github.com/linkerd/linkerd2-proxy
[tonic]: https://github.com/hyperium/tonic
[rs-crate-badge]: https://img.shields.io/crates/v/linkerd2-proxy-api.svg
[rs-crate-url]: https://crates.io/crates/linkerd2-proxy-api
[rs-docs-badge]: https://docs.rs/linkerd2-proxy-api/badge.svg
[rs-docs-url]: https://docs.rs/linkerd2-proxy-api
[lic-badge]: https://img.shields.io/github/license/linkerd/linkerd2-proxy-api.svg
