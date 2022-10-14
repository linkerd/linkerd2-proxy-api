#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetSpec {
    /// Identifies the source proxy workload (e.g., pod name).
    #[prost(string, tag="1")]
    pub workload: ::prost::alloc::string::String,
    /// Target address
    #[prost(message, optional, tag="2")]
    pub address: ::core::option::Option<super::net::IpAddress>,
    /// Target port
    #[prost(uint32, tag="3")]
    pub port: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    #[prost(message, optional, tag="1")]
    pub protocol: ::core::option::Option<ProxyProtocol>,
    #[prost(message, optional, tag="2")]
    pub retry_budget: ::core::option::Option<super::destination::RetryBudget>,
    /// Backends for this service. Can be overridden by backends specified
    /// in a matching route.
    #[prost(message, repeated, tag="3")]
    pub backends: ::prost::alloc::vec::Vec<Backend>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyProtocol {
    #[prost(oneof="proxy_protocol::Kind", tags="1, 2, 3, 4, 5")]
    pub kind: ::core::option::Option<proxy_protocol::Kind>,
}
/// Nested message and enum types in `ProxyProtocol`.
pub mod proxy_protocol {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Detect {
        #[prost(message, optional, tag="1")]
        pub timeout: ::core::option::Option<::prost_types::Duration>,
        /// If the protocol detected as HTTP, a list of HTTP routes that should be
        /// matched.
        #[prost(message, repeated, tag="2")]
        pub http_routes: ::prost::alloc::vec::Vec<super::HttpRoute>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http1 {
        #[prost(message, repeated, tag="1")]
        pub routes: ::prost::alloc::vec::Vec<super::HttpRoute>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http2 {
        #[prost(message, repeated, tag="1")]
        pub routes: ::prost::alloc::vec::Vec<super::HttpRoute>,
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
    }
}
/// Outbound-specific HTTP route configuration (based on the [Gateway API]\[api\]).
///
/// \[api\]: <https://gateway-api.sigs.k8s.io/v1alpha2/references/spec/#gateway.networking.k8s.io/v1alpha2.HTTPRoute>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRoute {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::meta::Metadata>,
    /// If empty, the host value is ignored.
    #[prost(message, repeated, tag="2")]
    pub hosts: ::prost::alloc::vec::Vec<super::http_route::HostMatch>,
    /// Must have at least one rule.
    #[prost(message, repeated, tag="3")]
    pub rules: ::prost::alloc::vec::Vec<http_route::Rule>,
    #[prost(message, repeated, tag="4")]
    pub response_classes: ::prost::alloc::vec::Vec<super::destination::ResponseClass>,
    /// If a route is retryable, any failed requests on that route may be retried
    /// by the proxy.
    #[prost(bool, tag="5")]
    pub is_retryable: bool,
    /// After this time has elapsed since receiving the initial request, any
    /// outstanding request will be cancelled, a timeout error response will be
    /// returned, and no more retries will be attempted.
    #[prost(message, optional, tag="6")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `HttpRoute`.
pub mod http_route {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rule {
        #[prost(message, repeated, tag="1")]
        pub matches: ::prost::alloc::vec::Vec<super::super::http_route::HttpRouteMatch>,
        #[prost(message, repeated, tag="3")]
        pub backends: ::prost::alloc::vec::Vec<super::Backend>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backend {
    #[prost(oneof="backend::Backend", tags="1, 2")]
    pub backend: ::core::option::Option<backend::Backend>,
}
/// Nested message and enum types in `Backend`.
pub mod backend {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Backend {
        #[prost(message, tag="1")]
        Dst(super::super::destination::WeightedDst),
        #[prost(message, tag="2")]
        Endpoint(super::super::destination::WeightedAddr),
    }
}
/// Generated client implementations.
pub mod outbound_policies_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OutboundPoliciesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OutboundPoliciesClient<T>
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
        ) -> OutboundPoliciesClient<InterceptedService<T, F>>
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
            OutboundPoliciesClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::TargetSpec>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
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
                "/io.linkerd.proxy.outbound.OutboundPolicies/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn watch(
            &mut self,
            request: impl tonic::IntoRequest<super::TargetSpec>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::Service>>,
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
                "/io.linkerd.proxy.outbound.OutboundPolicies/Watch",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod outbound_policies_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with OutboundPoliciesServer.
    #[async_trait]
    pub trait OutboundPolicies: Send + Sync + 'static {
        async fn get(
            &self,
            request: tonic::Request<super::TargetSpec>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status>;
        ///Server streaming response type for the Watch method.
        type WatchStream: futures_core::Stream<
                Item = Result<super::Service, tonic::Status>,
            >
            + Send
            + 'static;
        async fn watch(
            &self,
            request: tonic::Request<super::TargetSpec>,
        ) -> Result<tonic::Response<Self::WatchStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct OutboundPoliciesServer<T: OutboundPolicies> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: OutboundPolicies> OutboundPoliciesServer<T> {
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OutboundPoliciesServer<T>
    where
        T: OutboundPolicies,
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
                "/io.linkerd.proxy.outbound.OutboundPolicies/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: OutboundPolicies>(pub Arc<T>);
                    impl<
                        T: OutboundPolicies,
                    > tonic::server::UnaryService<super::TargetSpec> for GetSvc<T> {
                        type Response = super::Service;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TargetSpec>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
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
                "/io.linkerd.proxy.outbound.OutboundPolicies/Watch" => {
                    #[allow(non_camel_case_types)]
                    struct WatchSvc<T: OutboundPolicies>(pub Arc<T>);
                    impl<
                        T: OutboundPolicies,
                    > tonic::server::ServerStreamingService<super::TargetSpec>
                    for WatchSvc<T> {
                        type Response = super::Service;
                        type ResponseStream = T::WatchStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TargetSpec>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).watch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WatchSvc(inner);
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
    impl<T: OutboundPolicies> Clone for OutboundPoliciesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: OutboundPolicies> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: OutboundPolicies> tonic::server::NamedService for OutboundPoliciesServer<T> {
        const NAME: &'static str = "io.linkerd.proxy.outbound.OutboundPolicies";
    }
}
