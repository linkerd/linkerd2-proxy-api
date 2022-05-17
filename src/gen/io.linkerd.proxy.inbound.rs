#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortSpec {
    /// Identifies a proxy workload (e.g., pod name).
    #[prost(string, tag="1")]
    pub workload: ::prost::alloc::string::String,
    /// An inbound port on _workload_.
    #[prost(uint32, tag="2")]
    pub port: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Server {
    /// If set, indicates how the proxy should proxy connections on the specified
    /// port.
    #[prost(message, optional, tag="1")]
    pub protocol: ::core::option::Option<ProxyProtocol>,
    /// Indicates the IP addresses on which the proxy may receive connections.
    /// Connections targetting other IP addresses will be dropped.
    #[prost(message, repeated, tag="2")]
    pub server_ips: ::prost::alloc::vec::Vec<super::net::IpAddress>,
    /// Configures a proxy to allow connections from the specified clients.
    ///
    /// If unset, no connections are permitted.
    #[prost(message, repeated, tag="3")]
    pub authorizations: ::prost::alloc::vec::Vec<Authz>,
    /// Descriptive labels to be added to metrics, etc.
    ///
    /// A control plane SHOULD return the same keys in all policies. That is, we do
    /// NOT want to return arbitrary pod labels in this field.
    #[prost(map="string, string", tag="4")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyProtocol {
    #[prost(oneof="proxy_protocol::Kind", tags="1, 2, 3, 4, 5, 6")]
    pub kind: ::core::option::Option<proxy_protocol::Kind>,
}
/// Nested message and enum types in `ProxyProtocol`.
pub mod proxy_protocol {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Detect {
        #[prost(message, optional, tag="1")]
        pub timeout: ::core::option::Option<::prost_types::Duration>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http1 {
        /// Disables the setting of informational headers on this server.
        #[prost(bool, tag="1")]
        pub disable_informational_headers: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http2 {
        /// Disables the setting of informational headers on this server.
        #[prost(bool, tag="1")]
        pub disable_informational_headers: bool,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Grpc {
        /// Disables the setting of informational headers on this server.
        #[prost(bool, tag="1")]
        pub disable_informational_headers: bool,
    }
    /// TODO: opaque TLS settings (versions, algorithms, SNI)
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Opaque {
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Tls {
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag="1")]
        Detect(Detect),
        #[prost(message, tag="2")]
        Opaque(Opaque),
        #[prost(message, tag="3")]
        Tls(Tls),
        #[prost(message, tag="4")]
        Http1(Http1),
        #[prost(message, tag="5")]
        Http2(Http2),
        #[prost(message, tag="6")]
        Grpc(Grpc),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authz {
    /// Limits this authorization to client addresses in the provided networks.
    ///
    /// Must have at least one network, otherwise the authorization must be
    /// ignored. An authorization matches all clients by including an explicit
    /// match on, i.e., `[0.0.0.0/0, 0::/0]``.
    #[prost(message, repeated, tag="1")]
    pub networks: ::prost::alloc::vec::Vec<Network>,
    /// Must be set.
    #[prost(message, optional, tag="2")]
    pub authentication: ::core::option::Option<Authn>,
    /// Descriptive labels to be added to metrics, etc.
    ///
    /// A control plane SHOULD return the same keys in all authorizations. That is,
    /// we do NOT want to return arbitrary pod labels in this field.
    #[prost(map="string, string", tag="3")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Describes a network of authorized clients.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    #[prost(message, optional, tag="1")]
    pub net: ::core::option::Option<super::net::IpNetwork>,
    #[prost(message, repeated, tag="2")]
    pub except: ::prost::alloc::vec::Vec<super::net::IpNetwork>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authn {
    #[prost(oneof="authn::Permit", tags="1, 2")]
    pub permit: ::core::option::Option<authn::Permit>,
}
/// Nested message and enum types in `Authn`.
pub mod authn {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PermitUnauthenticated {
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PermitMeshTls {
        #[prost(oneof="permit_mesh_tls::Clients", tags="1, 2")]
        pub clients: ::core::option::Option<permit_mesh_tls::Clients>,
    }
    /// Nested message and enum types in `PermitMeshTLS`.
    pub mod permit_mesh_tls {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PermitClientIdentities {
            /// A list of literal identities.
            #[prost(message, repeated, tag="1")]
            pub identities: ::prost::alloc::vec::Vec<super::super::Identity>,
            /// A list of identity suffixes.
            ///
            /// If this contains an empty suffix, all identities are matched.
            #[prost(message, repeated, tag="2")]
            pub suffixes: ::prost::alloc::vec::Vec<super::super::IdentitySuffix>,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Clients {
            /// Indicates that client identities are not required.
            #[prost(message, tag="1")]
            Unauthenticated(super::PermitUnauthenticated),
            /// Indicates that mutually-authenticated connections are permitted from
            /// clients with matching identities.
            #[prost(message, tag="2")]
            Identities(PermitClientIdentities),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Permit {
        #[prost(message, tag="1")]
        Unauthenticated(PermitUnauthenticated),
        /// If set, requires that the connection is transported over mesh TLS.
        #[prost(message, tag="2")]
        MeshTls(PermitMeshTls),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Identity {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Encodes a DNS-like name suffix as sequence of parts.
///
/// An empty list is equivalent to `.` (matching all names); the list `["foo",
/// "bar"]` is equivalent to "foo.bar." (matching `*.foo.bar`), etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentitySuffix {
    #[prost(string, repeated, tag="1")]
    pub parts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod inbound_server_policies_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    //// An API exposed to the linkerd2-proxy to configure the inbound proxy with per-port configuration
    ////
    //// Proxies are expected to watch policies for each known port. As policies change, proxies update
    //// their behavior for newly accepted connections.
    ////
    //// The unary `GetPort` endpoint is exposed as a convenience for clients to query policies for
    //// diagnostic purposes.
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
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        //// Gets the inbound server policy for a given workload port.
        pub async fn get_port(
            &mut self,
            request: impl tonic::IntoRequest<super::PortSpec>,
        ) -> Result<tonic::Response<super::Server>, tonic::Status> {
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
            self.inner.unary(request.into_request(), path, codec).await
        }
        //// Watches the inbound server policy for a given workload port.
        pub async fn watch_port(
            &mut self,
            request: impl tonic::IntoRequest<super::PortSpec>,
        ) -> Result<
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
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod inbound_server_policies_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with InboundServerPoliciesServer.
    #[async_trait]
    pub trait InboundServerPolicies: Send + Sync + 'static {
        //// Gets the inbound server policy for a given workload port.
        async fn get_port(
            &self,
            request: tonic::Request<super::PortSpec>,
        ) -> Result<tonic::Response<super::Server>, tonic::Status>;
        ///Server streaming response type for the WatchPort method.
        type WatchPortStream: futures_core::Stream<
                Item = Result<super::Server, tonic::Status>,
            >
            + Send
            + 'static;
        //// Watches the inbound server policy for a given workload port.
        async fn watch_port(
            &self,
            request: tonic::Request<super::PortSpec>,
        ) -> Result<tonic::Response<Self::WatchPortStream>, tonic::Status>;
    }
    //// An API exposed to the linkerd2-proxy to configure the inbound proxy with per-port configuration
    ////
    //// Proxies are expected to watch policies for each known port. As policies change, proxies update
    //// their behavior for newly accepted connections.
    ////
    //// The unary `GetPort` endpoint is exposed as a convenience for clients to query policies for
    //// diagnostic purposes.
    #[derive(Debug)]
    pub struct InboundServerPoliciesServer<T: InboundServerPolicies> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
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
        ) -> Poll<Result<(), Self::Error>> {
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
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_port(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPortSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
                            let inner = self.0.clone();
                            let fut = async move { (*inner).watch_port(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WatchPortSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
            }
        }
    }
    impl<T: InboundServerPolicies> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
