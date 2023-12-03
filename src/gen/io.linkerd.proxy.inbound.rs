#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortSpec {
    /// Identifies a proxy workload (e.g., pod name).
    #[prost(string, tag = "1")]
    pub workload: ::prost::alloc::string::String,
    /// An inbound port on _workload_.
    #[prost(uint32, tag = "2")]
    pub port: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Server {
    /// If set, indicates how the proxy should proxy connections on the specified
    /// port.
    #[prost(message, optional, tag = "1")]
    pub protocol: ::core::option::Option<ProxyProtocol>,
    /// Indicates the IP addresses on which the proxy may receive connections.
    /// Connections targetting other IP addresses will be dropped.
    #[prost(message, repeated, tag = "2")]
    pub server_ips: ::prost::alloc::vec::Vec<super::net::IpAddress>,
    /// Configures a proxy to allow connections from the specified clients.
    ///
    /// If unset, no connections are permitted.
    #[prost(message, repeated, tag = "3")]
    pub authorizations: ::prost::alloc::vec::Vec<Authz>,
    /// Descriptive labels to be added to metrics, etc.
    ///
    /// A control plane SHOULD return the same keys in all policies. That is, we do
    /// NOT want to return arbitrary pod labels in this field.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyProtocol {
    #[prost(oneof = "proxy_protocol::Kind", tags = "1, 2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<proxy_protocol::Kind>,
}
/// Nested message and enum types in `ProxyProtocol`.
pub mod proxy_protocol {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Detect {
        #[prost(message, optional, tag = "1")]
        pub timeout: ::core::option::Option<::prost_types::Duration>,
        /// If the protocol detected as HTTP, a list of HTTP routes that should be
        /// matched.
        #[prost(message, repeated, tag = "3")]
        pub http_routes: ::prost::alloc::vec::Vec<super::HttpRoute>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http1 {
        #[prost(message, repeated, tag = "2")]
        pub routes: ::prost::alloc::vec::Vec<super::HttpRoute>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http2 {
        #[prost(message, repeated, tag = "2")]
        pub routes: ::prost::alloc::vec::Vec<super::HttpRoute>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Grpc {
        #[prost(message, repeated, tag = "2")]
        pub routes: ::prost::alloc::vec::Vec<super::GrpcRoute>,
    }
    /// TODO: opaque TLS settings (versions, algorithms, SNI)
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Opaque {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Tls {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Detect(Detect),
        #[prost(message, tag = "2")]
        Opaque(Opaque),
        #[prost(message, tag = "3")]
        Tls(Tls),
        #[prost(message, tag = "4")]
        Http1(Http1),
        #[prost(message, tag = "5")]
        Http2(Http2),
        #[prost(message, tag = "6")]
        Grpc(Grpc),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authz {
    /// Limits this authorization to client addresses in the provided networks.
    ///
    /// Must have at least one network, otherwise the authorization must be
    /// ignored. An authorization matches all clients by including an explicit
    /// match on, i.e., `\[0.0.0.0/0, 0::/0\]``.
    #[prost(message, repeated, tag = "1")]
    pub networks: ::prost::alloc::vec::Vec<Network>,
    /// Must be set.
    #[prost(message, optional, tag = "2")]
    pub authentication: ::core::option::Option<Authn>,
    /// Descriptive labels to be added to metrics, etc.
    ///
    /// A control plane SHOULD return the same keys in all authorizations. That is,
    /// we do NOT want to return arbitrary pod labels in this field.
    ///
    /// `labels` should be considered deprecated. `metadata` is preferred. However,
    /// controllers should continue to set `labels` for compatibility with older
    /// proxies.
    #[prost(map = "string, string", tag = "3")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// If set, describes an Authorization configuration. Replaces the free-from
    /// `labels` field.
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<super::meta::Metadata>,
}
/// Describes a network of authorized clients.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    #[prost(message, optional, tag = "1")]
    pub net: ::core::option::Option<super::net::IpNetwork>,
    #[prost(message, repeated, tag = "2")]
    pub except: ::prost::alloc::vec::Vec<super::net::IpNetwork>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authn {
    #[prost(oneof = "authn::Permit", tags = "1, 2")]
    pub permit: ::core::option::Option<authn::Permit>,
}
/// Nested message and enum types in `Authn`.
pub mod authn {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PermitUnauthenticated {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PermitMeshTls {
        #[prost(oneof = "permit_mesh_tls::Clients", tags = "1, 2")]
        pub clients: ::core::option::Option<permit_mesh_tls::Clients>,
    }
    /// Nested message and enum types in `PermitMeshTLS`.
    pub mod permit_mesh_tls {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PermitClientIdentities {
            /// A list of literal identities.
            #[prost(message, repeated, tag = "1")]
            pub identities: ::prost::alloc::vec::Vec<super::super::Identity>,
            /// A list of identity suffixes.
            ///
            /// If this contains an empty suffix, all identities are matched.
            #[prost(message, repeated, tag = "2")]
            pub suffixes: ::prost::alloc::vec::Vec<super::super::IdentitySuffix>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Clients {
            /// Indicates that client identities are not required.
            #[prost(message, tag = "1")]
            Unauthenticated(super::PermitUnauthenticated),
            /// Indicates that mutually-authenticated connections are permitted from
            /// clients with matching identities.
            #[prost(message, tag = "2")]
            Identities(PermitClientIdentities),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Permit {
        #[prost(message, tag = "1")]
        Unauthenticated(PermitUnauthenticated),
        /// If set, requires that the connection is transported over mesh TLS.
        #[prost(message, tag = "2")]
        MeshTls(PermitMeshTls),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Identity {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Encodes a DNS-like name suffix as sequence of parts.
///
/// An empty list is equivalent to `.` (matching all names); the list `["foo",
/// "bar"]` is equivalent to "foo.bar." (matching `*.foo.bar`), etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentitySuffix {
    #[prost(string, repeated, tag = "1")]
    pub parts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Inbound-specific HTTP route configuration (based on the
/// [Gateway API](<https://gateway-api.sigs.k8s.io/v1alpha2/references/spec/#gateway.networking.k8s.io/v1alpha2.HTTPRoute>)).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRoute {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::meta::Metadata>,
    /// If empty, the host value is ignored.
    #[prost(message, repeated, tag = "2")]
    pub hosts: ::prost::alloc::vec::Vec<super::http_route::HostMatch>,
    /// Extends the list of authorizations on the `Server` with authorizations
    /// specific to this route.
    #[prost(message, repeated, tag = "3")]
    pub authorizations: ::prost::alloc::vec::Vec<Authz>,
    /// Must have at least one rule.
    #[prost(message, repeated, tag = "4")]
    pub rules: ::prost::alloc::vec::Vec<http_route::Rule>,
}
/// Nested message and enum types in `HttpRoute`.
pub mod http_route {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rule {
        #[prost(message, repeated, tag = "1")]
        pub matches: ::prost::alloc::vec::Vec<super::super::http_route::HttpRouteMatch>,
        #[prost(message, repeated, tag = "2")]
        pub filters: ::prost::alloc::vec::Vec<Filter>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        #[prost(oneof = "filter::Kind", tags = "1, 2, 3")]
        pub kind: ::core::option::Option<filter::Kind>,
    }
    /// Nested message and enum types in `Filter`.
    pub mod filter {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            #[prost(message, tag = "1")]
            FailureInjector(super::super::super::http_route::HttpFailureInjector),
            #[prost(message, tag = "2")]
            RequestHeaderModifier(
                super::super::super::http_route::RequestHeaderModifier,
            ),
            #[prost(message, tag = "3")]
            Redirect(super::super::super::http_route::RequestRedirect),
        }
    }
}
/// Inbound-specific gRPC route configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcRoute {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::meta::Metadata>,
    /// If empty, the host value is ignored.
    #[prost(message, repeated, tag = "2")]
    pub hosts: ::prost::alloc::vec::Vec<super::http_route::HostMatch>,
    /// The server MUST return at least one authorization, otherwise all requests
    /// to this route will fail with an unauthorized response.
    #[prost(message, repeated, tag = "3")]
    pub authorizations: ::prost::alloc::vec::Vec<Authz>,
    /// Must have at least one rule.
    #[prost(message, repeated, tag = "4")]
    pub rules: ::prost::alloc::vec::Vec<grpc_route::Rule>,
}
/// Nested message and enum types in `GrpcRoute`.
pub mod grpc_route {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rule {
        #[prost(message, repeated, tag = "1")]
        pub matches: ::prost::alloc::vec::Vec<super::super::grpc_route::GrpcRouteMatch>,
        #[prost(message, repeated, tag = "2")]
        pub filters: ::prost::alloc::vec::Vec<Filter>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        #[prost(oneof = "filter::Kind", tags = "1, 2")]
        pub kind: ::core::option::Option<filter::Kind>,
    }
    /// Nested message and enum types in `Filter`.
    pub mod filter {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            #[prost(message, tag = "1")]
            FailureInjector(super::super::super::grpc_route::GrpcFailureInjector),
            #[prost(message, tag = "2")]
            RequestHeaderModifier(
                super::super::super::http_route::RequestHeaderModifier,
            ),
        }
    }
}
/// Generated client implementations.
pub mod inbound_server_policies_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// / An API exposed to the linkerd2-proxy to configure the inbound proxy with per-port configuration
    /// /
    /// / Proxies are expected to watch policies for each known port. As policies change, proxies update
    /// / their behavior for newly accepted connections.
    /// /
    /// / The unary `GetPort` endpoint is exposed as a convenience for clients to query policies for
    /// / diagnostic purposes.
    #[derive(Debug, Clone)]
    pub struct InboundServerPoliciesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> InboundServerPoliciesClient<T>
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
        ) -> InboundServerPoliciesClient<InterceptedService<T, F>>
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
            InboundServerPoliciesClient::new(InterceptedService::new(inner, interceptor))
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
        /// / Gets the inbound server policy for a given workload port.
        pub async fn get_port(
            &mut self,
            request: impl tonic::IntoRequest<super::PortSpec>,
        ) -> std::result::Result<tonic::Response<super::Server>, tonic::Status> {
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
                "/io.linkerd.proxy.inbound.InboundServerPolicies/GetPort",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "io.linkerd.proxy.inbound.InboundServerPolicies",
                        "GetPort",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// / Watches the inbound server policy for a given workload port.
        pub async fn watch_port(
            &mut self,
            request: impl tonic::IntoRequest<super::PortSpec>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::Server>>,
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
                "/io.linkerd.proxy.inbound.InboundServerPolicies/WatchPort",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "io.linkerd.proxy.inbound.InboundServerPolicies",
                        "WatchPort",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod inbound_server_policies_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InboundServerPoliciesServer.
    #[async_trait]
    pub trait InboundServerPolicies: Send + Sync + 'static {
        /// / Gets the inbound server policy for a given workload port.
        async fn get_port(
            &self,
            request: tonic::Request<super::PortSpec>,
        ) -> std::result::Result<tonic::Response<super::Server>, tonic::Status>;
        /// Server streaming response type for the WatchPort method.
        type WatchPortStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::Server, tonic::Status>,
            >
            + Send
            + 'static;
        /// / Watches the inbound server policy for a given workload port.
        async fn watch_port(
            &self,
            request: tonic::Request<super::PortSpec>,
        ) -> std::result::Result<tonic::Response<Self::WatchPortStream>, tonic::Status>;
    }
    /// / An API exposed to the linkerd2-proxy to configure the inbound proxy with per-port configuration
    /// /
    /// / Proxies are expected to watch policies for each known port. As policies change, proxies update
    /// / their behavior for newly accepted connections.
    /// /
    /// / The unary `GetPort` endpoint is exposed as a convenience for clients to query policies for
    /// / diagnostic purposes.
    #[derive(Debug)]
    pub struct InboundServerPoliciesServer<T: InboundServerPolicies> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InboundServerPolicies> InboundServerPoliciesServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for InboundServerPoliciesServer<T>
    where
        T: InboundServerPolicies,
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
                "/io.linkerd.proxy.inbound.InboundServerPolicies/GetPort" => {
                    #[allow(non_camel_case_types)]
                    struct GetPortSvc<T: InboundServerPolicies>(pub Arc<T>);
                    impl<
                        T: InboundServerPolicies,
                    > tonic::server::UnaryService<super::PortSpec> for GetPortSvc<T> {
                        type Response = super::Server;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PortSpec>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InboundServerPolicies>::get_port(&inner, request)
                                    .await
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
                        let method = GetPortSvc(inner);
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
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/io.linkerd.proxy.inbound.InboundServerPolicies/WatchPort" => {
                    #[allow(non_camel_case_types)]
                    struct WatchPortSvc<T: InboundServerPolicies>(pub Arc<T>);
                    impl<
                        T: InboundServerPolicies,
                    > tonic::server::ServerStreamingService<super::PortSpec>
                    for WatchPortSvc<T> {
                        type Response = super::Server;
                        type ResponseStream = T::WatchPortStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PortSpec>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as InboundServerPolicies>::watch_port(&inner, request)
                                    .await
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
                        let method = WatchPortSvc(inner);
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
    impl<T: InboundServerPolicies> Clone for InboundServerPoliciesServer<T> {
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
    impl<T: InboundServerPolicies> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InboundServerPolicies> tonic::server::NamedService
    for InboundServerPoliciesServer<T> {
        const NAME: &'static str = "io.linkerd.proxy.inbound.InboundServerPolicies";
    }
}
