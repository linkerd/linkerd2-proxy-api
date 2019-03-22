extern crate h2;
extern crate http;
extern crate prost;
extern crate prost_types;
#[cfg(feature = "arbitrary")]
extern crate quickcheck;
#[cfg(feature = "arbitrary")]
extern crate rand;
extern crate tower_grpc;

use std::fmt;
use std::error::Error;

#[cfg(feature = "arbitrary")]
pub mod arbitrary;

pub use self::gen::*;

// The generated code requires two tiers of outer modules so that references between
// modules resolve properly.
mod gen {
    pub mod net {
        include!(concat!(env!("OUT_DIR"), "/io.linkerd.proxy.net.rs"));
    }

    pub mod http_types {
        include!(concat!(env!("OUT_DIR"), "/io.linkerd.proxy.http_types.rs"));
    }

    pub mod destination {
        include!(concat!(env!("OUT_DIR"), "/io.linkerd.proxy.destination.rs"));
    }

    pub mod identity {
        include!(concat!(env!("OUT_DIR"), "/io.linkerd.proxy.identity.rs"));
    }

    pub mod tap {
        include!(concat!(env!("OUT_DIR"), "/io.linkerd.proxy.tap.rs"));
    }
}

pub fn pb_elapsed(t0: ::std::time::Instant, t1: ::std::time::Instant) -> ::prost_types::Duration {
    pb_duration(t1 - t0)
}

/// Converts a Rust Duration to a Protobuf Duration.
pub fn pb_duration(d: ::std::time::Duration) -> ::prost_types::Duration {
    let seconds = if d.as_secs() > ::std::i64::MAX as u64 {
        ::std::i64::MAX
    } else {
        d.as_secs() as i64
    };

    let nanos = if d.subsec_nanos() > ::std::i32::MAX as u32 {
        ::std::i32::MAX
    } else {
        d.subsec_nanos() as i32
    };

    ::prost_types::Duration {
        seconds,
        nanos,
    }
}

/// Indicates an HTTP Method could not be decoded.
#[derive(Debug, Clone)]
pub struct InvalidMethod;


/// Indicates a URI Scheme could not be decoded.
#[derive(Debug, Clone)]
pub struct InvalidScheme;


// ===== impl tap::Eos =====

impl From<h2::Reason> for tap::Eos {
    fn from(reason: h2::Reason) -> Self {
        let end = tap::eos::End::ResetErrorCode(reason.into());
        tap::Eos { end: Some(end) }
    }
}

impl tap::Eos {
    pub fn from_grpc_status(code: u32) -> Self {
        let end = tap::eos::End::GrpcStatusCode(code);
        tap::Eos { end: Some(end) }
    }
}

// ===== impl net::IpAddress =====

impl<T> From<T> for net::IpAddress
where
    net::ip_address::Ip: From<T>,
{
    #[inline]
    fn from(ip: T) -> Self {
        Self {
            ip: Some(ip.into()),
        }
    }
}

impl From<::std::net::IpAddr> for net::IpAddress {
    fn from(ip: ::std::net::IpAddr) -> Self {
        match ip {
            ::std::net::IpAddr::V4(v4) => Self {
                ip: Some(v4.into()),
            },
            ::std::net::IpAddr::V6(v6) => Self {
                ip: Some(v6.into()),
            },
        }
    }
}

impl From<[u8; 4]> for net::ip_address::Ip {
    fn from(octets: [u8; 4]) -> Self {
        net::ip_address::Ip::Ipv4(
            u32::from(octets[0]) << 24 | u32::from(octets[1]) << 16 | u32::from(octets[2]) << 8
                | u32::from(octets[3]),
        )
    }
}

// ===== impl net::ip_address:Ip =====

impl From<::std::net::Ipv4Addr> for net::ip_address::Ip {
    #[inline]
    fn from(v4: ::std::net::Ipv4Addr) -> Self {
        Self::from(v4.octets())
    }
}

impl<T> From<T> for net::ip_address::Ip
where
    net::IPv6: From<T>,
{
    #[inline]
    fn from(t: T) -> Self {
        net::ip_address::Ip::Ipv6(net::IPv6::from(t))
    }
}

// ===== impl net::IPv6 =====

impl From<[u8; 16]> for net::IPv6 {
    fn from(octets: [u8; 16]) -> Self {
        let first = (u64::from(octets[0]) << 56) + (u64::from(octets[1]) << 48)
            + (u64::from(octets[2]) << 40) + (u64::from(octets[3]) << 32)
            + (u64::from(octets[4]) << 24) + (u64::from(octets[5]) << 16)
            + (u64::from(octets[6]) << 8) + u64::from(octets[7]);
        let last = (u64::from(octets[8]) << 56) + (u64::from(octets[9]) << 48)
            + (u64::from(octets[10]) << 40) + (u64::from(octets[11]) << 32)
            + (u64::from(octets[12]) << 24) + (u64::from(octets[13]) << 16)
            + (u64::from(octets[14]) << 8) + u64::from(octets[15]);
        Self {
            first,
            last,
        }
    }
}

impl From<::std::net::Ipv6Addr> for net::IPv6 {
    #[inline]
    fn from(v6: ::std::net::Ipv6Addr) -> Self {
        Self::from(v6.octets())
    }
}

impl<'a> From<&'a net::IPv6> for ::std::net::Ipv6Addr {
    fn from(ip: &'a net::IPv6) -> ::std::net::Ipv6Addr {
        ::std::net::Ipv6Addr::new(
            (ip.first >> 48) as u16,
            (ip.first >> 32) as u16,
            (ip.first >> 16) as u16,
            (ip.first) as u16,
            (ip.last >> 48) as u16,
            (ip.last >> 32) as u16,
            (ip.last >> 16) as u16,
            (ip.last) as u16,
        )
    }
}

// ===== impl net::TcpAddress =====

impl<'a> From<&'a ::std::net::SocketAddr> for net::TcpAddress {
    fn from(sa: &::std::net::SocketAddr) -> net::TcpAddress {
        net::TcpAddress {
            ip: Some(sa.ip().into()),
            port: u32::from(sa.port()),
        }
    }
}

// ===== impl http_types::scheme::Type =====

impl http_types::scheme::Type {
    pub fn try_to_string(&self) -> Result<String, InvalidScheme> {
        use self::http_types::scheme::*;

        match *self {
            Type::Registered(reg) => if reg == Registered::Http.into() {
                Ok("http".into())
            } else if reg == Registered::Https.into() {
                Ok("https".into())
            } else {
                Err(InvalidScheme)
            },
            Type::Unregistered(ref s) => Ok(s.clone()),
        }
    }
}

// ===== impl http::HttpMethod =====

impl http_types::http_method::Type {
    pub fn try_as_http(&self) -> Result<http::Method, InvalidMethod> {
        use self::http_types::http_method::*;
        use http::HttpTryFrom;

        match *self {
            Type::Registered(reg) => if reg == Registered::Get.into() {
                Ok(http::Method::GET)
            } else if reg == Registered::Post.into() {
                Ok(http::Method::POST)
            } else if reg == Registered::Put.into() {
                Ok(http::Method::PUT)
            } else if reg == Registered::Delete.into() {
                Ok(http::Method::DELETE)
            } else if reg == Registered::Patch.into() {
                Ok(http::Method::PATCH)
            } else if reg == Registered::Options.into() {
                Ok(http::Method::OPTIONS)
            } else if reg == Registered::Connect.into() {
                Ok(http::Method::CONNECT)
            } else if reg == Registered::Head.into() {
                Ok(http::Method::HEAD)
            } else if reg == Registered::Trace.into() {
                Ok(http::Method::TRACE)
            } else {
                Err(InvalidMethod)
            },
            Type::Unregistered(ref m) => {
                HttpTryFrom::try_from(m.as_str()).map_err(|_| InvalidMethod)
            }
        }
    }
}

impl<'a> From<&'a http::Method> for http_types::http_method::Type {
    fn from(m: &'a http::Method) -> Self {
        use self::http_types::http_method::*;

        match *m {
            http::Method::GET => Type::Registered(Registered::Get.into()),
            http::Method::POST => Type::Registered(Registered::Post.into()),
            http::Method::PUT => Type::Registered(Registered::Put.into()),
            http::Method::DELETE => Type::Registered(Registered::Delete.into()),
            http::Method::HEAD => Type::Registered(Registered::Head.into()),
            http::Method::OPTIONS => Type::Registered(Registered::Options.into()),
            http::Method::CONNECT => Type::Registered(Registered::Connect.into()),
            http::Method::TRACE => Type::Registered(Registered::Trace.into()),
            ref method => Type::Unregistered(method.as_str().into()),
        }
    }
}

impl<'a> From<&'a http::Method> for http_types::HttpMethod {
    fn from(m: &'a http::Method) -> Self {
        http_types::HttpMethod {
            r#type: Some(m.into()),
        }
    }
}

// ===== impl InvalidMethod =====

impl fmt::Display for InvalidMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid http method")
    }
}

impl Error for InvalidMethod {
    #[inline]
    fn description(&self) -> &str {
        "invalid http method"
    }
}

// ===== impl http_types::Scheme =====

impl<'a> From<&'a http::uri::Scheme> for http_types::Scheme {
    fn from(scheme: &'a http::uri::Scheme) -> Self {
        scheme.as_ref().into()
    }
}

impl<'a> From<&'a str> for http_types::scheme::Type {
    fn from(s: &'a str) -> Self {
        use self::http_types::scheme::*;

        match s {
            "http" => Type::Registered(Registered::Http.into()),
            "https" => Type::Registered(Registered::Https.into()),
            s => Type::Unregistered(s.into()),
        }
    }
}

impl<'a> From<&'a str> for http_types::Scheme {
    fn from(s: &'a str) -> Self {
        http_types::Scheme {
            r#type: Some(s.into()),
        }
    }
}

// ===== impl InvalidScheme =====

impl fmt::Display for InvalidScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid http scheme")
    }
}

impl Error for InvalidScheme {
    #[inline]
    fn description(&self) -> &str {
        "invalid http scheme"
    }
}
