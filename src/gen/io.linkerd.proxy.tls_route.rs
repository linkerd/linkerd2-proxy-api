/// Describes how to match an `SNI` ClientHello extension.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SniMatch {
    #[prost(oneof = "sni_match::Match", tags = "1, 2")]
    pub r#match: ::core::option::Option<sni_match::Match>,
}
/// Nested message and enum types in `SniMatch`.
pub mod sni_match {
    /// A match like `*.example.com` is encoded as \[com, example\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Suffix {
        #[prost(string, repeated, tag = "1")]
        pub reverse_labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Match {
        /// Match an exact SNI, e.g. www.example.com.
        #[prost(string, tag = "1")]
        Exact(::prost::alloc::string::String),
        /// Match a SNI as a wildcard suffix, e.g. *.example.com.
        #[prost(message, tag = "2")]
        Suffix(Suffix),
    }
}
/// Error type that is used to indicate that this backend
/// is invalid and traffic targeted to it should be failed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidBackendError {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Error type that is used to indicate that any traffic
/// that is delivered through a route should be failed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteError {
    #[prost(enumeration = "route_error::Kind", tag = "1")]
    pub kind: i32,
}
/// Nested message and enum types in `RouteError`.
pub mod route_error {
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
    pub enum Kind {
        Forbidden = 0,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Forbidden => "FORBIDDEN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FORBIDDEN" => Some(Self::Forbidden),
                _ => None,
            }
        }
    }
}
