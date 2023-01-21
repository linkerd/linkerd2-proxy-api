#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpMethod {
    #[prost(oneof = "http_method::Type", tags = "1, 2")]
    pub r#type: ::core::option::Option<http_method::Type>,
}
/// Nested message and enum types in `HttpMethod`.
pub mod http_method {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Registered {
        Get = 0,
        Post = 1,
        Put = 2,
        Delete = 3,
        Patch = 4,
        Options = 5,
        Connect = 6,
        Head = 7,
        Trace = 8,
    }
    impl Registered {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Registered::Get => "GET",
                Registered::Post => "POST",
                Registered::Put => "PUT",
                Registered::Delete => "DELETE",
                Registered::Patch => "PATCH",
                Registered::Options => "OPTIONS",
                Registered::Connect => "CONNECT",
                Registered::Head => "HEAD",
                Registered::Trace => "TRACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GET" => Some(Self::Get),
                "POST" => Some(Self::Post),
                "PUT" => Some(Self::Put),
                "DELETE" => Some(Self::Delete),
                "PATCH" => Some(Self::Patch),
                "OPTIONS" => Some(Self::Options),
                "CONNECT" => Some(Self::Connect),
                "HEAD" => Some(Self::Head),
                "TRACE" => Some(Self::Trace),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(enumeration = "Registered", tag = "1")]
        Registered(i32),
        #[prost(string, tag = "2")]
        Unregistered(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scheme {
    #[prost(oneof = "scheme::Type", tags = "1, 2")]
    pub r#type: ::core::option::Option<scheme::Type>,
}
/// Nested message and enum types in `Scheme`.
pub mod scheme {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Registered {
        Http = 0,
        Https = 1,
    }
    impl Registered {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Registered::Http => "HTTP",
                Registered::Https => "HTTPS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HTTP" => Some(Self::Http),
                "HTTPS" => Some(Self::Https),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(enumeration = "Registered", tag = "1")]
        Registered(i32),
        #[prost(string, tag = "2")]
        Unregistered(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Headers {
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<headers::Header>,
}
/// Nested message and enum types in `Headers`.
pub mod headers {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Header {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(bytes = "vec", tag = "2")]
        pub value: ::prost::alloc::vec::Vec<u8>,
    }
}
