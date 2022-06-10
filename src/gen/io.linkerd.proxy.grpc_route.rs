#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcRouteMatch {
    #[prost(message, optional, tag="1")]
    pub method: ::core::option::Option<GrpcMethodMatch>,
    /// A set of header value matches that must be satisified. This match is not
    /// comprehensive, so requests may include headers that are not covered by this
    /// match.
    #[prost(message, repeated, tag="2")]
    pub header: ::prost::alloc::vec::Vec<super::http_route::HeaderMatch>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcMethodMatch {
    #[prost(string, tag="1")]
    pub service: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub method: ::prost::alloc::string::String,
}
/// Configures a route to respond with a fixed response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcErrorResponder {
    /// The status code to use in the `grpc-status` response. Must be specified.
    #[prost(uint32, tag="1")]
    pub code: u32,
    /// An error message to log and include in the `grpc-message` header.
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
