#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcRouteMatch {
    #[prost(message, optional, tag = "1")]
    pub rpc: ::core::option::Option<GrpcRpcMatch>,
    /// A set of header value matches that must be satisified. This match is not
    /// comprehensive, so requests may include headers that are not covered by this
    /// match.
    #[prost(message, repeated, tag = "2")]
    pub headers: ::prost::alloc::vec::Vec<super::http_route::HeaderMatch>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcRpcMatch {
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub method: ::prost::alloc::string::String,
}
/// Configures a route to respond with a fixed response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcFailureInjector {
    /// The status code to use in the `grpc-status` response. Must be specified.
    #[prost(uint32, tag = "1")]
    pub code: u32,
    /// An error message to log and include in the `grpc-message` header.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// If specified, the rate of requests that should be failed. If not specified,
    /// ALL requests are failed.
    #[prost(message, optional, tag = "3")]
    pub ratio: ::core::option::Option<super::http_route::Ratio>,
}
