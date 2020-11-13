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
