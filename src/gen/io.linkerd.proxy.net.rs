#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAddress {
    #[prost(oneof = "ip_address::Ip", tags = "1, 2")]
    pub ip: ::core::option::Option<ip_address::Ip>,
}
/// Nested message and enum types in `IPAddress`.
pub mod ip_address {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ip {
        #[prost(fixed32, tag = "1")]
        Ipv4(u32),
        #[prost(message, tag = "2")]
        Ipv6(super::IPv6),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpNetwork {
    #[prost(message, optional, tag = "1")]
    pub ip: ::core::option::Option<IpAddress>,
    #[prost(uint32, tag = "2")]
    pub prefix_len: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IPv6 {
    /// hextets 1-4
    #[prost(fixed64, tag = "1")]
    pub first: u64,
    /// hextets 5-8
    #[prost(fixed64, tag = "2")]
    pub last: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpAddress {
    #[prost(message, optional, tag = "1")]
    pub ip: ::core::option::Option<IpAddress>,
    #[prost(uint32, tag = "2")]
    pub port: u32,
}
