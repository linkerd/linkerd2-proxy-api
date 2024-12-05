use std::convert::{TryFrom, TryInto};
use thiserror::Error;

include!("gen/io.linkerd.proxy.net.rs");

/// Indicates an IP address could not be decoded.
#[derive(Clone, Debug, Error)]
#[error("invalid IP address")]
pub struct InvalidIpAddress;

/// Indicates an IP address could not be decoded.
#[derive(Clone, Debug, Error)]
pub enum InvalidIpNetwork {
    #[error("invalid IP address")]
    Ip(#[from] InvalidIpAddress),
    #[error("invalid network prefix length")]
    PrefixLen(#[from] ipnet::PrefixLenError),
}

// === impl IpAddress ===

#[cfg(feature = "net")]
impl TryFrom<IpAddress> for std::net::IpAddr {
    type Error = InvalidIpAddress;

    fn try_from(ip: IpAddress) -> Result<Self, Self::Error> {
        use ip_address::Ip;
        match ip.ip {
            Some(Ip::Ipv4(v4)) => Ok(std::net::IpAddr::V4(v4.into())),
            Some(Ip::Ipv6(v6)) => Ok(std::net::IpAddr::V6(v6.into())),
            None => Err(InvalidIpAddress),
        }
    }
}

#[cfg(feature = "net")]
impl<T> From<T> for IpAddress
where
    ip_address::Ip: From<T>,
{
    #[inline]
    fn from(ip: T) -> Self {
        Self {
            ip: Some(ip.into()),
        }
    }
}

impl From<std::net::IpAddr> for IpAddress {
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

impl From<[u8; 4]> for ip_address::Ip {
    fn from(octets: [u8; 4]) -> Self {
        ip_address::Ip::Ipv4(
            u32::from(octets[0]) << 24
                | u32::from(octets[1]) << 16
                | u32::from(octets[2]) << 8
                | u32::from(octets[3]),
        )
    }
}

// === impl IpNetwork ===

impl TryFrom<IpNetwork> for ipnet::IpNet {
    type Error = InvalidIpNetwork;

    fn try_from(net: IpNetwork) -> Result<Self, Self::Error> {
        let ip = net
            .ip
            .ok_or(InvalidIpNetwork::Ip(InvalidIpAddress))?
            .try_into()
            .map_err(InvalidIpNetwork::Ip)?;
        let prefix_len = if (0..=u8::MAX as u32).contains(&net.prefix_len) {
            net.prefix_len as u8
        } else {
            return Err(InvalidIpNetwork::PrefixLen(ipnet::PrefixLenError));
        };
        match ip {
            std::net::IpAddr::V4(addr) => ipnet::Ipv4Net::new(addr, prefix_len)
                .map(Into::into)
                .map_err(InvalidIpNetwork::PrefixLen),
            std::net::IpAddr::V6(addr) => ipnet::Ipv6Net::new(addr, prefix_len)
                .map(Into::into)
                .map_err(InvalidIpNetwork::PrefixLen),
        }
    }
}

impl<T> From<(T, u8)> for IpNetwork
where
    IpAddress: From<T>,
{
    #[inline]
    fn from((ip, prefix_len): (T, u8)) -> Self {
        Self {
            ip: Some(ip.into()),
            prefix_len: prefix_len.into(),
        }
    }
}

impl From<ipnet::IpNet> for IpNetwork {
    fn from(net: ipnet::IpNet) -> Self {
        IpNetwork {
            ip: Some(net.addr().into()),
            prefix_len: net.prefix_len().into(),
        }
    }
}

// === impl ip_address:Ip ===

impl From<std::net::Ipv4Addr> for ip_address::Ip {
    #[inline]
    fn from(v4: std::net::Ipv4Addr) -> Self {
        Self::from(v4.octets())
    }
}

impl<T> From<T> for ip_address::Ip
where
    IPv6: From<T>,
{
    #[inline]
    fn from(t: T) -> Self {
        ip_address::Ip::Ipv6(IPv6::from(t))
    }
}

// === impl IPv6 ===

impl From<[u8; 16]> for IPv6 {
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

impl From<std::net::Ipv6Addr> for IPv6 {
    #[inline]
    fn from(v6: std::net::Ipv6Addr) -> Self {
        Self::from(v6.octets())
    }
}

impl From<IPv6> for std::net::Ipv6Addr {
    fn from(ip: IPv6) -> std::net::Ipv6Addr {
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

// === impl TcpAddress ===

impl From<std::net::SocketAddr> for TcpAddress {
    fn from(sa: std::net::SocketAddr) -> TcpAddress {
        TcpAddress {
            ip: Some(sa.ip().into()),
            port: u32::from(sa.port()),
        }
    }
}

impl TryFrom<TcpAddress> for std::net::SocketAddr {
    type Error = InvalidIpAddress;

    fn try_from(tcp: TcpAddress) -> Result<std::net::SocketAddr, Self::Error> {
        if let Some(ip) = tcp.ip {
            let port = tcp.port as u16;
            let ip = std::net::IpAddr::try_from(ip)?;
            return Ok(std::net::SocketAddr::from((ip, port)));
        }

        Err(InvalidIpAddress)
    }
}

#[cfg(feature = "arbitrary")]
mod arbitary {
    use super::*;
    use quickcheck::*;

    impl Arbitrary for IpAddress {
        fn arbitrary(g: &mut Gen) -> Self {
            IpAddress {
                ip: Arbitrary::arbitrary(g),
            }
        }
    }

    impl Arbitrary for ip_address::Ip {
        fn arbitrary(g: &mut Gen) -> Self {
            if bool::arbitrary(g) {
                ip_address::Ip::Ipv4(Arbitrary::arbitrary(g))
            } else {
                ip_address::Ip::Ipv6(IPv6::arbitrary(g))
            }
        }
    }

    impl Arbitrary for IPv6 {
        fn arbitrary(g: &mut Gen) -> Self {
            IPv6 {
                first: Arbitrary::arbitrary(g),
                last: Arbitrary::arbitrary(g),
            }
        }
    }
}
