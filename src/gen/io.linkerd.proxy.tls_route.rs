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
