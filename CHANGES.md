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
