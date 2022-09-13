#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientPolicy {
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
    #[prost(string, tag="1")]
    pub fully_qualified_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub protocol: ::core::option::Option<ProxyProtocol>,
    /// A list of filters that should be considered when sending traffic to the
    /// destination.
    #[prost(message, repeated, tag="3")]
    pub filters: ::prost::alloc::vec::Vec<Filter>,
    /// If this field is set, it indicates that the target is a known endpoint (and
    /// not a service address). The values of `fully_qualified_name` and
    /// `dst_overrides` will be ignored for the purposes of service discovery--
    /// traffic split and load balancing will be skipped and the single endpoint
    /// are used.
    ///
    /// No endpoint should be set If the target is unknown.
    #[prost(message, optional, tag="4")]
    pub endpoint: ::core::option::Option<super::destination::WeightedAddr>,
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
        #[prost(message, repeated, tag="3")]
        pub http_routes: ::prost::alloc::vec::Vec<super::HttpRoute>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http1 {
        #[prost(message, repeated, tag="2")]
        pub routes: ::prost::alloc::vec::Vec<super::HttpRoute>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http2 {
        #[prost(message, repeated, tag="2")]
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
}
/// Nested message and enum types in `HttpRoute`.
pub mod http_route {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rule {
        #[prost(message, repeated, tag="1")]
        pub matches: ::prost::alloc::vec::Vec<super::super::http_route::HttpRouteMatch>,
        #[prost(message, repeated, tag="2")]
        pub filters: ::prost::alloc::vec::Vec<super::Filter>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(oneof="filter::Filter", tags="1")]
    pub filter: ::core::option::Option<filter::Filter>,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// The timeout that should be used for requests.
        #[prost(message, tag="1")]
        Timeout(::prost_types::Duration),
    }
}
/// Generated client implementations.
pub mod client_policies_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ClientPoliciesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ClientPoliciesClient<T>
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
        ) -> ClientPoliciesClient<InterceptedService<T, F>>
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
            ClientPoliciesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Given a destination, return the ClientPolicy that is attached to that
        /// destination and send an update whenever it changes.
        pub async fn get_client_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::destination::GetDestination>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ClientPolicy>>,
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
                "/io.linkerd.proxy.client_policy.ClientPolicies/GetClientPolicy",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod client_policies_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ClientPoliciesServer.
    #[async_trait]
    pub trait ClientPolicies: Send + Sync + 'static {
        ///Server streaming response type for the GetClientPolicy method.
        type GetClientPolicyStream: futures_core::Stream<
                Item = Result<super::ClientPolicy, tonic::Status>,
            >
            + Send
            + 'static;
        /// Given a destination, return the ClientPolicy that is attached to that
        /// destination and send an update whenever it changes.
        async fn get_client_policy(
            &self,
            request: tonic::Request<super::super::destination::GetDestination>,
        ) -> Result<tonic::Response<Self::GetClientPolicyStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ClientPoliciesServer<T: ClientPolicies> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ClientPolicies> ClientPoliciesServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ClientPoliciesServer<T>
    where
        T: ClientPolicies,
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
                "/io.linkerd.proxy.client_policy.ClientPolicies/GetClientPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetClientPolicySvc<T: ClientPolicies>(pub Arc<T>);
                    impl<
                        T: ClientPolicies,
                    > tonic::server::ServerStreamingService<
                        super::super::destination::GetDestination,
                    > for GetClientPolicySvc<T> {
                        type Response = super::ClientPolicy;
                        type ResponseStream = T::GetClientPolicyStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::destination::GetDestination,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_client_policy(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetClientPolicySvc(inner);
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
    impl<T: ClientPolicies> Clone for ClientPoliciesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ClientPolicies> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ClientPolicies> tonic::server::NamedService for ClientPoliciesServer<T> {
        const NAME: &'static str = "io.linkerd.proxy.client_policy.ClientPolicies";
    }
}
