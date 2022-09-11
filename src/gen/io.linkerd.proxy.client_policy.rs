#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientPolicy {
    /// The fully-qualifed service name if one exists.
    #[prost(string, tag="1")]
    pub fully_qualifed_name: ::prost::alloc::string::String,
    /// A list of routes, each with a RequestMatch. If a request matches more
    /// than one route, the first match wins.
    #[prost(message, repeated, tag="2")]
    pub routes: ::prost::alloc::vec::Vec<super::destination::Route>,
    /// A list of filters that should be considered when sending traffic to the
    /// destination.
    #[prost(message, repeated, tag="3")]
    pub filters: ::prost::alloc::vec::Vec<Filter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(oneof="filter::Filter", tags="1, 2, 3, 4")]
    pub filter: ::core::option::Option<filter::Filter>,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// The timeout that should be used for requests.
        #[prost(message, tag="1")]
        Timeout(::prost_types::Duration),
        /// The load a proxy can generate as retries. Failed requests on retryable
        /// routes will not be retried if there is no available budget
        #[prost(message, tag="2")]
        RetryBudget(super::super::destination::RetryBudget),
        /// The TrafficSplit that should be used for requests.
        #[prost(message, tag="3")]
        TrafficSplit(super::TrafficSplit),
        /// The ExtensionReference that should be used for requests.
        #[prost(message, tag="4")]
        ExtensionReference(super::ExtensionReference),
    }
}
/// TrafficSplit allows clients to dynamically shift arbitrary portions of
/// traffic for one destination to a list of other destinations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficSplit {
    /// The list of destinations that each receive a portion of requests
    /// proportional to their weight.
    #[prost(message, repeated, tag="1")]
    pub destination_overrides: ::prost::alloc::vec::Vec<super::destination::WeightedDst>,
}
/// ExtensionReference allows clients to use an arbitrary extension for
/// specifying certain traffic behaviors that should be used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionReference {
    #[prost(string, tag="1")]
    pub group: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
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
        /// Given a destination, return all Endpoints for that destination as a long-
        /// running stream of updates.
        pub async fn get_endpoints(
            &mut self,
            request: impl tonic::IntoRequest<super::super::destination::GetDestination>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::destination::Update>>,
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
                "/io.linkerd.proxy.client_policy.Destination/GetEndpoints",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
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
                "/io.linkerd.proxy.client_policy.Destination/GetClientPolicy",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod destination_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with DestinationServer.
    #[async_trait]
    pub trait Destination: Send + Sync + 'static {
        ///Server streaming response type for the GetEndpoints method.
        type GetEndpointsStream: futures_core::Stream<
                Item = Result<super::super::destination::Update, tonic::Status>,
            >
            + Send
            + 'static;
        /// Given a destination, return all Endpoints for that destination as a long-
        /// running stream of updates.
        async fn get_endpoints(
            &self,
            request: tonic::Request<super::super::destination::GetDestination>,
        ) -> Result<tonic::Response<Self::GetEndpointsStream>, tonic::Status>;
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
    pub struct DestinationServer<T: Destination> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
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
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/io.linkerd.proxy.client_policy.Destination/GetEndpoints" => {
                    #[allow(non_camel_case_types)]
                    struct GetEndpointsSvc<T: Destination>(pub Arc<T>);
                    impl<
                        T: Destination,
                    > tonic::server::ServerStreamingService<
                        super::super::destination::GetDestination,
                    > for GetEndpointsSvc<T> {
                        type Response = super::super::destination::Update;
                        type ResponseStream = T::GetEndpointsStream;
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
                                (*inner).get_endpoints(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEndpointsSvc(inner);
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
                "/io.linkerd.proxy.client_policy.Destination/GetClientPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetClientPolicySvc<T: Destination>(pub Arc<T>);
                    impl<
                        T: Destination,
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
    impl<T: Destination> Clone for DestinationServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Destination> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Destination> tonic::server::NamedService for DestinationServer<T> {
        const NAME: &'static str = "io.linkerd.proxy.client_policy.Destination";
    }
}
