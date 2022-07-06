#![deny(warnings, rust_2018_idioms)]
#![forbid(unsafe_code)]

#[cfg(feature = "http-types")]
pub mod http_types;

#[cfg(feature = "destination")]
pub mod destination {
    include!("gen/io.linkerd.proxy.destination.rs");
}

#[cfg(feature = "identity")]
pub mod identity {
    include!("gen/io.linkerd.proxy.identity.rs");
}

#[cfg(feature = "inbound")]
pub mod inbound {
    include!("gen/io.linkerd.proxy.inbound.rs");
}

#[cfg(feature = "meta")]
pub mod meta {
    include!("gen/io.linkerd.proxy.meta.rs");
}

#[cfg(feature = "net")]
pub mod net;

#[cfg(feature = "tap")]
pub mod tap;
