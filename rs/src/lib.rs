#![deny(warnings, rust_2018_idioms)]

pub use self::gen::*;
use std::borrow::Cow;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt;

// The generated code requires two tiers of outer modules so that references between
// modules resolve properly.
mod gen {
    pub mod net {
        tonic::include_proto!("io.linkerd.proxy.net");
    }

    pub mod http_types {
        tonic::include_proto!("io.linkerd.proxy.http_types");
    }

    pub mod destination {
        tonic::include_proto!("io.linkerd.proxy.destination");
    }

    pub mod identity {
        tonic::include_proto!("io.linkerd.proxy.identity");
    }

    pub mod tap {
        tonic::include_proto!("io.linkerd.proxy.tap");
    }
}

pub fn pb_elapsed(t0: std::time::Instant, t1: std::time::Instant) -> prost_types::Duration {
    pb_duration(t1 - t0)
}

/// Converts a Rust Duration to a Protobuf Duration.
pub fn pb_duration(d: std::time::Duration) -> prost_types::Duration {
    let seconds = if d.as_secs() > std::i64::MAX as u64 {
        std::i64::MAX
    } else {
        d.as_secs() as i64
    };

    let nanos = if d.subsec_nanos() > std::i32::MAX as u32 {
        std::i32::MAX
    } else {
        d.subsec_nanos() as i32
    };

    ::prost_types::Duration { seconds, nanos }
}

/// Indicates an HTTP Method could not be decoded.
#[derive(Debug, Clone)]
pub struct InvalidMethod;

/// Indicates a URI Scheme could not be decoded.
#[derive(Debug, Clone)]
pub struct InvalidScheme;

/// Indicates an IP address could not be decoded.
#[derive(Debug, Clone)]
pub struct InvalidIpAddress;

/// Indicates an IP address could not be decoded.
#[derive(Debug, Clone)]
pub enum InvalidIpNetwork {
    Ip(InvalidIpAddress),
    PrefixLen(ipnet::PrefixLenError),
}

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

impl TryFrom<net::IpAddress> for std::net::IpAddr {
    type Error = InvalidIpAddress;
    fn try_from(ip: net::IpAddress) -> Result<Self, Self::Error> {
        use net::ip_address::Ip;
        match ip.ip {
            Some(Ip::Ipv4(octets)) => Ok(std::net::Ipv4Addr::from(octets).into()),
            Some(Ip::Ipv6(v6)) => std::net::Ipv6Addr::try_from(v6)
                .map(Into::into)
                .map_err(|_| InvalidIpAddress),
            None => Err(InvalidIpAddress),
        }
    }
}

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

impl From<std::net::IpAddr> for net::IpAddress {
    fn from(ip: std::net::IpAddr) -> Self {
        match ip {
            std::net::IpAddr::V4(v4) => Self {
                ip: Some(v4.into()),
            },
            std::net::IpAddr::V6(v6) => Self {
                ip: Some(v6.into()),
            },
        }
    }
}

// ===== impl net::IpAddress =====

impl TryFrom<net::IpNetwork> for ipnet::IpNet {
    type Error = InvalidIpNetwork;
    fn try_from(net: net::IpNetwork) -> Result<Self, Self::Error> {
        let ip = net
            .ip
            .ok_or(InvalidIpNetwork::Ip(InvalidIpAddress))?
            .try_into()
            .map_err(InvalidIpNetwork::Ip)?;
        let prefix_len = if (0..=std::u8::MAX as u32).contains(&net.prefix_len) {
            net.prefix_len as u8
        } else {
            return Err(InvalidIpNetwork::PrefixLen(ipnet::PrefixLenError));
        };
        match ip {
            std::net::IpAddr::V4(addr) => ipnet::Ipv4Net::new(addr, prefix_len).map(Into::into),
            std::net::IpAddr::V6(addr) => ipnet::Ipv6Net::new(addr, prefix_len).map(Into::into),
        }
        .map_err(InvalidIpNetwork::PrefixLen)
    }
}

impl<T> From<(T, u8)> for net::IpNetwork
where
    net::IpAddress: From<T>,
{
    #[inline]
    fn from((ip, prefix_len): (T, u8)) -> Self {
        Self {
            ip: Some(ip.into()),
            prefix_len: prefix_len.into(),
        }
    }
}

impl From<ipnet::IpNet> for net::IpNetwork {
    fn from(net: ipnet::IpNet) -> Self {
        net::IpNetwork {
            ip: Some(net.addr().into()),
            prefix_len: net.prefix_len().into(),
        }
    }
}

// ===== impl net::ip_address:Ip =====

impl From<[u8; 4]> for net::ip_address::Ip {
    fn from(octets: [u8; 4]) -> Self {
        net::ip_address::Ip::Ipv4(
            u32::from(octets[0]) << 24
                | u32::from(octets[1]) << 16
                | u32::from(octets[2]) << 8
                | u32::from(octets[3]),
        )
    }
}

impl From<std::net::Ipv4Addr> for net::ip_address::Ip {
    #[inline]
    fn from(v4: std::net::Ipv4Addr) -> Self {
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
        let first = (u64::from(octets[0]) << 56)
            + (u64::from(octets[1]) << 48)
            + (u64::from(octets[2]) << 40)
            + (u64::from(octets[3]) << 32)
            + (u64::from(octets[4]) << 24)
            + (u64::from(octets[5]) << 16)
            + (u64::from(octets[6]) << 8)
            + u64::from(octets[7]);
        let last = (u64::from(octets[8]) << 56)
            + (u64::from(octets[9]) << 48)
            + (u64::from(octets[10]) << 40)
            + (u64::from(octets[11]) << 32)
            + (u64::from(octets[12]) << 24)
            + (u64::from(octets[13]) << 16)
            + (u64::from(octets[14]) << 8)
            + u64::from(octets[15]);
        Self { first, last }
    }
}

impl From<std::net::Ipv6Addr> for net::IPv6 {
    #[inline]
    fn from(v6: std::net::Ipv6Addr) -> Self {
        Self::from(v6.octets())
    }
}

impl From<net::IPv6> for std::net::Ipv6Addr {
    fn from(ip: net::IPv6) -> std::net::Ipv6Addr {
        std::net::Ipv6Addr::new(
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

impl From<std::net::SocketAddr> for net::TcpAddress {
    fn from(sa: std::net::SocketAddr) -> net::TcpAddress {
        net::TcpAddress {
            ip: Some(sa.ip().into()),
            port: u32::from(sa.port()),
        }
    }
}

impl TryFrom<net::TcpAddress> for std::net::SocketAddr {
    type Error = InvalidIpAddress;

    fn try_from(tcp: net::TcpAddress) -> Result<std::net::SocketAddr, Self::Error> {
        if let Some(ip) = tcp.ip {
            let port = tcp.port as u16;
            let ip = std::net::IpAddr::try_from(ip)?;
            return Ok(std::net::SocketAddr::from((ip, port)));
        }

        Err(InvalidIpAddress)
    }
}

// ===== impl http_types::scheme::Type =====

impl TryInto<Cow<'static, str>> for &'_ http_types::scheme::Type {
    type Error = InvalidScheme;
    fn try_into(self) -> Result<Cow<'static, str>, Self::Error> {
        use self::http_types::scheme::*;

        match *self {
            Type::Registered(reg) => {
                if reg == Registered::Http.into() {
                    Ok(Cow::Borrowed("http"))
                } else if reg == Registered::Https.into() {
                    Ok(Cow::Borrowed("https"))
                } else {
                    Err(InvalidScheme)
                }
            }
            Type::Unregistered(ref s) => Ok(Cow::Owned(s.clone())),
        }
    }
}

// ===== impl http::HttpMethod =====

impl TryInto<http::Method> for &'_ http_types::http_method::Type {
    type Error = InvalidMethod;
    fn try_into(self) -> Result<http::Method, Self::Error> {
        use self::http_types::http_method::*;

        match *self {
            Type::Registered(reg) => {
                if reg == Registered::Get.into() {
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
                }
            }
            Type::Unregistered(ref m) => TryFrom::try_from(m.as_str()).map_err(|_| InvalidMethod),
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid http method")
    }
}

impl Error for InvalidMethod {}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid http scheme")
    }
}

impl Error for InvalidScheme {}

// ===== impl InvalidIpAddress =====

impl fmt::Display for InvalidIpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid ip address")
    }
}

impl Error for InvalidIpAddress {}

// ===== impl InvalidIpNetwork =====

impl fmt::Display for InvalidIpNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid network: ")?;
        match self {
            Self::Ip(e) => e.fmt(f),
            Self::PrefixLen(e) => e.fmt(f),
        }
    }
}

impl Error for InvalidIpNetwork {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Ip(e) => Some(&*e),
            Self::PrefixLen(e) => Some(&*e),
        }
    }
}
