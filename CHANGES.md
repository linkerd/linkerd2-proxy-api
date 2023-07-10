# `linkerd2-proxy-api` changes

## v0.11.0

* Add `response_header_modifier` outbound HTTP filter

## v0.10.0

* Add `requestTimeout` fields to routes and route backends in the
  `OutboundPolicies` API
* Rust: update `h2` to v0.3.17

## v0.9.0

* Add new `OutboundPolicies` API
* Go: update `google.golang.org/grpc` to v1.53

## v0.8.0

* Add `Opaque` variant to `destination.ProtocolHint`

## v0.7.0

* Rust: Update `tonic` to v0.8 and `prost` to v0.11

## v0.6.0

* Add HTTP and gRPC Route types
* Add a resource Metadata type
* Go: update `google.golang.org/grpc` to v1.48
* Rust: Make error types cloneable
* Rust: Update `http_types` converters and enum matching
* Rust: Rename the `http_types` feature to `http-types`

## v0.5.0

* Rust: Remove the build-time dependency on `protoc`
* Rust: Remove the `client`, `server`, and `transport` features

## v0.4.0

* Go: Update `google.golang.org/protobuf` to v1.28
* Go: Update `google.golang.org/grpc` to v1.45
* Go: Update `protoc-gen-go` to v1.28
* Go: Update `protoc-gen-go-grpc` to v1.2
* Rust: Update `tonic` to v0.7 and `prost` to v0.10
* Rust: Update the MSRV to 1.59

## v0.3.1

* Rust: Fix serialization of `http::Method::PATCH` to use a registered method

## v0.3.0

* Rust: Update `tonic` to v0.6 and `prost` to v0.9
* Go: Update `google.golang.org/grpc` to v1.43.0

## v0.2.0

* Add the `io.linkerd.proxy.inbound.InboundServerPolicies` API to support
  server-side configuration and policy.
* Go: Update dependencies, including protoc and grpc
* Rust: Update dependencies, including tonic v0.5
* Rust: Add features for all APIs
* Rust: Add `client` and `server` features

## v0.1.18

* Update tonic to v0.4

## v0.1.17

* Update Rust dependencies

## v0.1.16

* Add an optional `OpaqueTransport` to the endpoint `ProtocolHint`

## v0.1.15

* Add `endpoint` to profile responses
* Update Rust dependencies

## v0.1.14

* Add `opaque_protocol` to profile responses
* Add `fully_qualified_name` to profile responses

## v0.1.13

* Update Rust crate to use `tonic` instead of `tower-grpc`
* Update `grpc-go` dependencies

## v0.1.12

* Add `AuthorityOverride` to  `WeightedAddr`

## v0.1.11

* Update Rust dependencies

## v0.1.10

* Add headers and trailers to Tap events
* Update tower-grpc to 0.1 from crates.io

## v0.1.9

* Add traffic split to profiles API

## v0.1.8

* Update Rust dependencies

## v0.1.7

* Introduce a new Identity service.
* destination: Deprecate `K8SPodIdentity` in favor of `DnsLikeIdentity`.
* destination: Rename `proxy_id` field to `context_token`.

## v0.1.6

* Add `proxy_id` field to Get and GetProfile methods.

## v0.1.5

* Add retryability and timeouts to destination profiles.

## v0.1.4

* Add Route metadata to Tap queries and events.

## v0.1.3

* Add metrics labels to routes
* Fix rust compilation for 'arbitrary' feature

## v0.1.2

* `io.linkerd.proxy.destination`
  * Add destination profile API to destination service
* Go
  * Upgrade protobuf to v1.2.0, dep to v0.5.0

## v0.1.1

* `io.linkerd.proxy.destination`
  * Support a per-endpoint `ProtocolHint` to allow for transparent
    proxy-to-proxy HTTP/2 upgrading
