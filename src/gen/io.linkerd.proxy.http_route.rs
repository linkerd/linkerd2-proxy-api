/// Describes how to match an `:authority` or `host` header.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostMatch {
    #[prost(oneof = "host_match::Match", tags = "1, 2")]
    pub r#match: ::core::option::Option<host_match::Match>,
}
/// Nested message and enum types in `HostMatch`.
pub mod host_match {
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
        /// Match an exact hostname, e.g. www.example.com.
        #[prost(string, tag = "1")]
        Exact(::prost::alloc::string::String),
        /// Match a hostname as a wildcard suffix, e.g. *.example.com.
        #[prost(message, tag = "2")]
        Suffix(Suffix),
    }
}
/// Describes a set of matches, ALL of which must apply.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRouteMatch {
    /// Matches requests by path.
    #[prost(message, optional, tag = "1")]
    pub path: ::core::option::Option<PathMatch>,
    /// A set of header value matches that must be satisified. This match is not
    /// comprehensive, so requests may include headers that are not covered by this
    /// match.
    #[prost(message, repeated, tag = "2")]
    pub headers: ::prost::alloc::vec::Vec<HeaderMatch>,
    /// A set of query parmaeter value matches that must be satisified. This match
    /// is not comprehensive, so requests may include query parameters that are not
    /// covered by this match.
    #[prost(message, repeated, tag = "3")]
    pub query_params: ::prost::alloc::vec::Vec<QueryParamMatch>,
    /// If specified, restricts the match to a single HTTP method.
    #[prost(message, optional, tag = "4")]
    pub method: ::core::option::Option<super::http_types::HttpMethod>,
}
/// Describes how to match a path.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PathMatch {
    #[prost(oneof = "path_match::Kind", tags = "1, 2, 3")]
    pub kind: ::core::option::Option<path_match::Kind>,
}
/// Nested message and enum types in `PathMatch`.
pub mod path_match {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(string, tag = "1")]
        Exact(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        Prefix(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        Regex(::prost::alloc::string::String),
    }
}
/// Describes how to match a header by name and value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMatch {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "header_match::Value", tags = "2, 3")]
    pub value: ::core::option::Option<header_match::Value>,
}
/// Nested message and enum types in `HeaderMatch`.
pub mod header_match {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(bytes, tag = "2")]
        Exact(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag = "3")]
        Regex(::prost::alloc::string::String),
    }
}
/// Describes how to match a query parameter by name and value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamMatch {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "query_param_match::Value", tags = "2, 3")]
    pub value: ::core::option::Option<query_param_match::Value>,
}
/// Nested message and enum types in `QueryParamMatch`.
pub mod query_param_match {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "2")]
        Exact(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        Regex(::prost::alloc::string::String),
    }
}
/// Configures a route to modify a request's headers.
///
/// Modifications are to be applied in the order they are described here:
/// additions apply first, then sets, and then removals.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestHeaderModifier {
    /// A list of headers name-value pairs to set on requests, augmenting any
    /// existing values for the header.
    #[prost(message, optional, tag = "1")]
    pub add: ::core::option::Option<super::http_types::Headers>,
    /// A list of headers name-value pairs to set on requests, replacing any
    /// existing values for the header.
    #[prost(message, optional, tag = "2")]
    pub set: ::core::option::Option<super::http_types::Headers>,
    /// A list of headers names to be removed from requests.
    #[prost(string, repeated, tag = "3")]
    pub remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Configures a route to modify a response's headers.
///
/// Modifications are to be applied in the order they are described here:
/// additions apply first, then sets, and then removals.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseHeaderModifier {
    /// A list of headers name-value pairs to set on responses, augmenting any
    /// existing values for the header.
    #[prost(message, optional, tag = "1")]
    pub add: ::core::option::Option<super::http_types::Headers>,
    /// A list of headers name-value pairs to set on responses, replacing any
    /// existing values for the header.
    #[prost(message, optional, tag = "2")]
    pub set: ::core::option::Option<super::http_types::Headers>,
    /// A list of headers names to be removed from responses.
    #[prost(string, repeated, tag = "3")]
    pub remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Configures a route to respond with a redirect response. The `location` header
/// is set with the given URL parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestRedirect {
    /// The scheme value to be used in the `location` header. If not specified,
    /// the original request's scheme is used.
    #[prost(message, optional, tag = "1")]
    pub scheme: ::core::option::Option<super::http_types::Scheme>,
    /// The host value to be used in the `location` header. If not specified, the
    /// original request's host is used.
    #[prost(string, tag = "2")]
    pub host: ::prost::alloc::string::String,
    /// If set, configures how the request's path should be modified for use in
    /// the `location` header. If not specified, the original request's path is
    /// used.
    #[prost(message, optional, tag = "3")]
    pub path: ::core::option::Option<PathModifier>,
    /// If set, specifies the port to use in the `location` header.
    #[prost(uint32, tag = "4")]
    pub port: u32,
    /// The status code to use in the HTTP response. If not specified, 301 is
    /// used.
    #[prost(uint32, tag = "5")]
    pub status: u32,
}
/// Describes how a path value may be rewritten in a route.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PathModifier {
    #[prost(oneof = "path_modifier::Replace", tags = "1, 2")]
    pub replace: ::core::option::Option<path_modifier::Replace>,
}
/// Nested message and enum types in `PathModifier`.
pub mod path_modifier {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Replace {
        /// Indicates that the path should be replaced with the given value.
        #[prost(string, tag = "1")]
        Full(::prost::alloc::string::String),
        /// Indicates that the path prefix should be replaced with the given
        /// value. When used, the route MUST match the request with PathMatch
        /// prefix match. Server implementations SHOULD prevent the useof prefix
        /// modifiers on routes that do use a PathMatch prefix match. Proxyies
        /// MUST not process requests for routes where this condition is not
        /// satisfied.
        #[prost(string, tag = "2")]
        Prefix(::prost::alloc::string::String),
    }
}
/// Configures a route to respond with a fixed response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpFailureInjector {
    /// The status code to use in the HTTP response. Must be specified.
    #[prost(uint32, tag = "1")]
    pub status: u32,
    /// An error message to log and include in the `l5d-proxy-err` header.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// If specified, the rate of requests that should be failed. If not specified,
    /// ALL requests are failed.
    #[prost(message, optional, tag = "3")]
    pub ratio: ::core::option::Option<Ratio>,
}
/// A ratio (i.e., of requests) to which an filter should be applied.
///
/// Represents fractional values on \[0, 1\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ratio {
    #[prost(uint32, tag = "1")]
    pub numerator: u32,
    /// MUST not be zero.
    #[prost(uint32, tag = "2")]
    pub denominator: u32,
}
