#[cfg(feature = "net")]
pub mod net;

#[cfg(feature = "http_types")]
pub mod http_types;

#[cfg(feature = "destination")]
pub mod destination {
    tonic::include_proto!("io.linkerd.proxy.destination");
}

#[cfg(feature = "identity")]
pub mod identity {
    tonic::include_proto!("io.linkerd.proxy.identity");
}

#[cfg(feature = "tap")]
pub mod tap;
