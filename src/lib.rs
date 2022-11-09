#![deny(warnings, rust_2018_idioms)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![forbid(unsafe_code)]

#[cfg(feature = "destination")]
pub mod destination {
    include!("gen/io.linkerd.proxy.destination.rs");
}

#[cfg(feature = "grpc-route")]
pub mod grpc_route {
    include!("gen/io.linkerd.proxy.grpc_route.rs");
}

#[cfg(feature = "http-route")]
pub mod http_route {
    include!("gen/io.linkerd.proxy.http_route.rs");
}

#[cfg(feature = "http-types")]
pub mod http_types;

#[cfg(feature = "identity")]
pub mod identity {
    include!("gen/io.linkerd.proxy.identity.rs");
}

#[cfg(feature = "inbound")]
pub mod inbound {
    include!("gen/io.linkerd.proxy.inbound.rs");
}

#[cfg(feature = "outbound")]
pub mod outbound {
    include!("gen/io.linkerd.proxy.outbound.rs");
}

#[cfg(feature = "meta")]
pub mod meta {
    include!("gen/io.linkerd.proxy.meta.rs");
}

#[cfg(feature = "net")]
pub mod net;

#[cfg(feature = "tap")]
pub mod tap;
