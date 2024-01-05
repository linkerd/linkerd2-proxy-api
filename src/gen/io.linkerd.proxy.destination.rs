#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDestination {
    #[prost(string, tag = "1")]
    pub scheme: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// An opaque value that is set at injection-time and sent with destintion
    /// lookups.
    ///
    /// If, for instance, the token encodes a namespace or some locality
    /// information, the service may alter its results to take this locality into
    /// account.
    #[prost(string, tag = "3")]
    pub context_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Update {
    #[prost(oneof = "update::Update", tags = "1, 2, 3")]
    pub update: ::core::option::Option<update::Update>,
}
/// Nested message and enum types in `Update`.
pub mod update {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Update {
        /// A new set of endpoints are available for the service. The set might be
        /// empty.
        #[prost(message, tag = "1")]
        Add(super::WeightedAddrSet),
        /// Some endpoints have been removed from the service.
        #[prost(message, tag = "2")]
        Remove(super::AddrSet),
        /// `no_endpoints{exists: false}` indicates that the service does not exist
        /// and the client MAY try an alternate service discovery method (e.g. DNS).
        ///
        /// `no_endpoints(exists: true)` indicates that the service does exist and
        /// the client MUST NOT fall back to an alternate service discovery method.
        #[prost(message, tag = "3")]
        NoEndpoints(super::NoEndpoints),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddrSet {
    #[prost(message, repeated, tag = "1")]
    pub addrs: ::prost::alloc::vec::Vec<super::net::TcpAddress>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightedAddrSet {
    #[prost(message, repeated, tag = "1")]
    pub addrs: ::prost::alloc::vec::Vec<WeightedAddr>,
    #[prost(map = "string, string", tag = "2")]
    pub metric_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightedAddr {
    #[prost(message, optional, tag = "1")]
    pub addr: ::core::option::Option<super::net::TcpAddress>,
    #[prost(uint32, tag = "3")]
    pub weight: u32,
    #[prost(map = "string, string", tag = "4")]
    pub metric_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "5")]
    pub tls_identity: ::core::option::Option<TlsIdentity>,
    #[prost(message, optional, tag = "6")]
    pub protocol_hint: ::core::option::Option<ProtocolHint>,
    #[prost(message, optional, tag = "7")]
    pub authority_override: ::core::option::Option<AuthorityOverride>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsIdentity {
    /// The server name of the endpoint. This is the value that needs to be included
    /// by clients in the ClientHello SNI extension of the TLS handshake when they
    /// initiate TLS connections to servers.
    #[prost(message, optional, tag = "4")]
    pub server_name: ::core::option::Option<tls_identity::DnsLikeIdentity>,
    #[prost(oneof = "tls_identity::Strategy", tags = "1, 3")]
    pub strategy: ::core::option::Option<tls_identity::Strategy>,
}
/// Nested message and enum types in `TlsIdentity`.
pub mod tls_identity {
    /// Verify the certificate based on the Kubernetes pod identity.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DnsLikeIdentity {
        /// A DNS-like name that encodes workload coordinates.
        ///
        /// For example:
        ///     {name}.{namespace}.{type}.identity.{control-namespace}.{trust-domain...}
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
    }
    /// Verify the certificate based on an URI identity.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UriLikeIdentity {
        /// A URI name that encodes workload identity.
        ///
        /// For example:
        ///     spiffe://trust-domain/workload-dentifier
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Strategy {
        #[prost(message, tag = "1")]
        DnsLikeIdentity(DnsLikeIdentity),
        #[prost(message, tag = "3")]
        UriLikeIdentity(UriLikeIdentity),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorityOverride {
    #[prost(string, tag = "1")]
    pub authority_override: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoEndpoints {
    #[prost(bool, tag = "1")]
    pub exists: bool,
}
/// A hint of what protocol the service knows. The default value is
/// for the `hint` field to be not be set, essentially meaning "unknown".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolHint {
    /// When set, indicates that the target supports receiving opaque traffic
    /// wrapped with the Linkerd connection header on the specified port.
    #[prost(message, optional, tag = "2")]
    pub opaque_transport: ::core::option::Option<protocol_hint::OpaqueTransport>,
    #[prost(oneof = "protocol_hint::Protocol", tags = "1, 3")]
    pub protocol: ::core::option::Option<protocol_hint::Protocol>,
}
/// Nested message and enum types in `ProtocolHint`.
pub mod protocol_hint {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct H2 {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Opaque {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OpaqueTransport {
        /// The target proxy's inbound port.
        #[prost(uint32, tag = "1")]
        pub inbound_port: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Protocol {
        /// Hints that the service understands HTTP2 and the proxy's internal
        /// http2-upgrade mechanism.
        #[prost(message, tag = "1")]
        H2(H2),
        /// Hints that the destination will handle this connection as an opaque TCP
        /// stream, and (if `opaque_transport` is set) that the proxy should not send
        /// a `SessionProtocol` as part of its transport header.
        #[prost(message, tag = "3")]
        Opaque(Opaque),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationProfile {
    /// The fully-qualified service name, if one exists.
    ///
    /// When resolving (especially by IP), this field provides the fully-qualified
    /// name of the resolved service, if one exists. This field does NOT include
    /// any port information. E.g. a lookup for 10.2.3.4:8080 might have a name
    /// like `foo.bar.svc.cluster.local`.
    ///
    /// Implementations MAY provide names for non-service IP-lookups (e.g., pod or
    /// node dns names), but this is not required.
    ///
    /// If the lookup does not refer to a known named entity, this field MUST be
    /// left empty.
    #[prost(string, tag = "5")]
    pub fully_qualified_name: ::prost::alloc::string::String,
    /// Indicates that connections on this service address should be handled as
    /// opaque TCP streams. HTTP routes returned on for such services will be
    /// ignored.
    #[prost(bool, tag = "4")]
    pub opaque_protocol: bool,
    /// A list of routes, each with a RequestMatch.  If a request matches
    /// more than one route, the first match wins.
    #[prost(message, repeated, tag = "1")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
    /// The retry budget controls how much additional load the proxy can generate
    /// as retries. Failured requests on retryable routes will not be retried if
    /// there is no available budget.
    #[prost(message, optional, tag = "2")]
    pub retry_budget: ::core::option::Option<RetryBudget>,
    /// If this list is non-empty, requests to this destination should instead be
    /// split between the destinations in this list.  Each destination should
    /// receive a portion of the requests proportional to its weight.  If this
    /// list is empty, requests should be sent to this destination as normal.
    #[prost(message, repeated, tag = "3")]
    pub dst_overrides: ::prost::alloc::vec::Vec<WeightedDst>,
    /// If this field is set, it indicates that the target is a known endpoint (and
    /// not a service address). The values of `fully_qualified_name` and
    /// `dst_overrides` will be ignored for the purposes of service discovery--
    /// traffic split and load balancing will be skipped and the single endpoint
    /// are used.
    ///
    /// No endpoint should be set If the target is unknown.
    #[prost(message, optional, tag = "6")]
    pub endpoint: ::core::option::Option<WeightedAddr>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// This route contains requests which match this condition.
    #[prost(message, optional, tag = "1")]
    pub condition: ::core::option::Option<RequestMatch>,
    /// A list of response classes for this route.  If a response matches
    /// more than one ResponseClass, the first match wins.  If a response does not
    /// match any ResponseClasses, it is considered to be a successful response.
    #[prost(message, repeated, tag = "2")]
    pub response_classes: ::prost::alloc::vec::Vec<ResponseClass>,
    /// Metric labels to attach to requests and responses that match this route.
    #[prost(map = "string, string", tag = "3")]
    pub metrics_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// If a route is retryable, any failed requests on that route may be retried
    /// by the proxy.
    #[prost(bool, tag = "4")]
    pub is_retryable: bool,
    /// After this time has elapsed since receiving the initial request, any
    /// outstanding request will be cancelled, a timeout error response will be
    /// returned, and no more retries will be attempted.
    #[prost(message, optional, tag = "5")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryBudget {
    /// The ratio of additional traffic that may be added by retries.  A
    /// retry_ratio of 0.1 means that 1 retry may be attempted for every 10 regular
    /// requests.  A retry_ratio of 1.0 means that 1 retry may be attempted for
    /// every 1 regular request (in other words, total request load may be doubled
    /// as a result of retries).
    #[prost(float, tag = "1")]
    pub retry_ratio: f32,
    /// The proxy may always attempt this number of retries per second, even if it
    /// would violate the retry_ratio.  This is to allow retries to happen even
    /// when the request rate is very low.
    #[prost(uint32, tag = "2")]
    pub min_retries_per_second: u32,
    /// This duration indicates for how long requests should be considered for the
    /// purposes of enforcing the retry_ratio.  A higher value considers a larger
    /// window and therefore allows burstier retries.
    #[prost(message, optional, tag = "3")]
    pub ttl: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseClass {
    /// This class contains responses which match this condition.
    #[prost(message, optional, tag = "1")]
    pub condition: ::core::option::Option<ResponseMatch>,
    /// If responses in this class should be considered failures.  This defaults
    /// to false (success).
    #[prost(bool, tag = "2")]
    pub is_failure: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMatch {
    #[prost(oneof = "request_match::Match", tags = "1, 2, 3, 4, 5")]
    pub r#match: ::core::option::Option<request_match::Match>,
}
/// Nested message and enum types in `RequestMatch`.
pub mod request_match {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Seq {
        #[prost(message, repeated, tag = "1")]
        pub matches: ::prost::alloc::vec::Vec<super::RequestMatch>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Match {
        #[prost(message, tag = "1")]
        All(Seq),
        #[prost(message, tag = "2")]
        Any(Seq),
        #[prost(message, tag = "3")]
        Not(::prost::alloc::boxed::Box<super::RequestMatch>),
        #[prost(message, tag = "4")]
        Path(super::PathMatch),
        /// TODO: match on arbitrary header
        #[prost(message, tag = "5")]
        Method(super::super::http_types::HttpMethod),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PathMatch {
    /// Match if the request path matches this regex.
    #[prost(string, tag = "1")]
    pub regex: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMatch {
    #[prost(oneof = "response_match::Match", tags = "1, 2, 3, 4")]
    pub r#match: ::core::option::Option<response_match::Match>,
}
/// Nested message and enum types in `ResponseMatch`.
pub mod response_match {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Seq {
        #[prost(message, repeated, tag = "1")]
        pub matches: ::prost::alloc::vec::Vec<super::ResponseMatch>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Match {
        #[prost(message, tag = "1")]
        All(Seq),
        #[prost(message, tag = "2")]
        Any(Seq),
        #[prost(message, tag = "3")]
        Not(::prost::alloc::boxed::Box<super::ResponseMatch>),
        /// TODO: match on arbitrary header or trailer
        #[prost(message, tag = "4")]
        Status(super::HttpStatusRange),
    }
}
/// If either a minimum or maximum is not specified, the range is considered to
/// be over a discrete value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpStatusRange {
    /// Minimum matching http status code (inclusive), if specified.
    #[prost(uint32, tag = "1")]
    pub min: u32,
    /// Maximum matching http status code (inclusive), if specified.
    #[prost(uint32, tag = "2")]
    pub max: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightedDst {
    /// This authority will be used as the `path` in a call to the Destination.Get
    /// rpc.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// The proportion of requests to send to this destination.  This value is
    /// relative to other weights in the same dst_overrides list.
    #[prost(uint32, tag = "2")]
    pub weight: u32,
}
/// Generated client implementations.
pub mod destination_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DestinationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DestinationClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DestinationClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DestinationClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Given a destination, return all addresses in that destination as a long-
        /// running stream of updates.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDestination>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Update>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.linkerd.proxy.destination.Destination/Get",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("io.linkerd.proxy.destination.Destination", "Get"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Given a destination, return that destination's profile and send an update
        /// whenever it changes.
        pub async fn get_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDestination>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::DestinationProfile>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/io.linkerd.proxy.destination.Destination/GetProfile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "io.linkerd.proxy.destination.Destination",
                        "GetProfile",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod destination_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DestinationServer.
    #[async_trait]
    pub trait Destination: Send + Sync + 'static {
        /// Server streaming response type for the Get method.
        type GetStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::Update, tonic::Status>,
            >
            + Send
            + 'static;
        /// Given a destination, return all addresses in that destination as a long-
        /// running stream of updates.
        async fn get(
            &self,
            request: tonic::Request<super::GetDestination>,
        ) -> std::result::Result<tonic::Response<Self::GetStream>, tonic::Status>;
        /// Server streaming response type for the GetProfile method.
        type GetProfileStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::DestinationProfile, tonic::Status>,
            >
            + Send
            + 'static;
        /// Given a destination, return that destination's profile and send an update
        /// whenever it changes.
        async fn get_profile(
            &self,
            request: tonic::Request<super::GetDestination>,
        ) -> std::result::Result<tonic::Response<Self::GetProfileStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DestinationServer<T: Destination> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Destination> DestinationServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DestinationServer<T>
    where
        T: Destination,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/io.linkerd.proxy.destination.Destination/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: Destination>(pub Arc<T>);
                    impl<
                        T: Destination,
                    > tonic::server::ServerStreamingService<super::GetDestination>
                    for GetSvc<T> {
                        type Response = super::Update;
                        type ResponseStream = T::GetStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDestination>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Destination>::get(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.linkerd.proxy.destination.Destination/GetProfile" => {
                    #[allow(non_camel_case_types)]
                    struct GetProfileSvc<T: Destination>(pub Arc<T>);
                    impl<
                        T: Destination,
                    > tonic::server::ServerStreamingService<super::GetDestination>
                    for GetProfileSvc<T> {
                        type Response = super::DestinationProfile;
                        type ResponseStream = T::GetProfileStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDestination>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Destination>::get_profile(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProfileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Destination> Clone for DestinationServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Destination> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Destination> tonic::server::NamedService for DestinationServer<T> {
        const NAME: &'static str = "io.linkerd.proxy.destination.Destination";
    }
}
