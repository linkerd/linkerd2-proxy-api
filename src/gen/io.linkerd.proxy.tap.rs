#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObserveRequest {
    /// Limits the number of event keys that will be returned by this tap.
    #[prost(uint32, tag = "1")]
    pub limit: u32,
    /// Encodes request-matching logic.
    #[prost(message, optional, tag = "2")]
    pub r#match: ::core::option::Option<observe_request::Match>,
    /// Conditionally extracts components from requests and responses to include
    /// in tap events
    #[prost(message, optional, tag = "3")]
    pub extract: ::core::option::Option<observe_request::Extract>,
}
/// Nested message and enum types in `ObserveRequest`.
pub mod observe_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Match {
        #[prost(oneof = "r#match::Match", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
        pub r#match: ::core::option::Option<r#match::Match>,
    }
    /// Nested message and enum types in `Match`.
    pub mod r#match {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Seq {
            #[prost(message, repeated, tag = "1")]
            pub matches: ::prost::alloc::vec::Vec<super::Match>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Label {
            #[prost(string, tag = "1")]
            pub key: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub value: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Tcp {
            #[prost(oneof = "tcp::Match", tags = "1, 3")]
            pub r#match: ::core::option::Option<tcp::Match>,
        }
        /// Nested message and enum types in `Tcp`.
        pub mod tcp {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Netmask {
                #[prost(message, optional, tag = "1")]
                pub ip: ::core::option::Option<
                    super::super::super::super::net::IpAddress,
                >,
                #[prost(uint32, tag = "2")]
                pub mask: u32,
            }
            /// If either a minimum or maximum is not specified, the range is
            /// considered to be over a discrete value.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct PortRange {
                /// Minimum matching port value (inclusive), if specified.
                #[prost(uint32, tag = "1")]
                pub min: u32,
                /// Maximum matching port value (inclusive), if specified.
                #[prost(uint32, tag = "2")]
                pub max: u32,
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Match {
                #[prost(message, tag = "1")]
                Netmask(Netmask),
                #[prost(message, tag = "3")]
                Ports(PortRange),
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Http {
            #[prost(oneof = "http::Match", tags = "1, 3, 2, 4")]
            pub r#match: ::core::option::Option<http::Match>,
        }
        /// Nested message and enum types in `Http`.
        pub mod http {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct StringMatch {
                #[prost(oneof = "string_match::Match", tags = "1, 2")]
                pub r#match: ::core::option::Option<string_match::Match>,
            }
            /// Nested message and enum types in `StringMatch`.
            pub mod string_match {
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Match {
                    #[prost(string, tag = "1")]
                    Exact(::prost::alloc::string::String),
                    #[prost(string, tag = "2")]
                    Prefix(::prost::alloc::string::String),
                }
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Match {
                #[prost(message, tag = "1")]
                Scheme(super::super::super::super::http_types::Scheme),
                #[prost(message, tag = "3")]
                Method(super::super::super::super::http_types::HttpMethod),
                #[prost(message, tag = "2")]
                Authority(StringMatch),
                /// TODO Header        header    = 4;
                #[prost(message, tag = "4")]
                Path(StringMatch),
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Match {
            #[prost(message, tag = "1")]
            All(Seq),
            #[prost(message, tag = "2")]
            Any(Seq),
            #[prost(message, tag = "3")]
            Not(::prost::alloc::boxed::Box<super::Match>),
            #[prost(message, tag = "4")]
            Source(Tcp),
            #[prost(message, tag = "5")]
            Destination(Tcp),
            #[prost(message, tag = "6")]
            Http(Http),
            #[prost(message, tag = "7")]
            DestinationLabel(Label),
            #[prost(message, tag = "8")]
            RouteLabel(Label),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Extract {
        #[prost(oneof = "extract::Extract", tags = "1")]
        pub extract: ::core::option::Option<extract::Extract>,
    }
    /// Nested message and enum types in `Extract`.
    pub mod extract {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Http {
            #[prost(oneof = "http::Extract", tags = "1")]
            pub extract: ::core::option::Option<http::Extract>,
        }
        /// Nested message and enum types in `Http`.
        pub mod http {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Headers {}
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Extract {
                #[prost(message, tag = "1")]
                Headers(Headers),
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Extract {
            #[prost(message, tag = "1")]
            Http(Http),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Eos {
    #[prost(oneof = "eos::End", tags = "1, 2")]
    pub end: ::core::option::Option<eos::End>,
}
/// Nested message and enum types in `Eos`.
pub mod eos {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum End {
        #[prost(uint32, tag = "1")]
        GrpcStatusCode(u32),
        #[prost(uint32, tag = "2")]
        ResetErrorCode(u32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TapEvent {
    #[prost(message, optional, tag = "1")]
    pub source: ::core::option::Option<super::net::TcpAddress>,
    #[prost(message, optional, tag = "5")]
    pub source_meta: ::core::option::Option<tap_event::EndpointMeta>,
    #[prost(message, optional, tag = "7")]
    pub route_meta: ::core::option::Option<tap_event::RouteMeta>,
    #[prost(message, optional, tag = "2")]
    pub destination: ::core::option::Option<super::net::TcpAddress>,
    #[prost(message, optional, tag = "4")]
    pub destination_meta: ::core::option::Option<tap_event::EndpointMeta>,
    #[prost(enumeration = "tap_event::ProxyDirection", tag = "6")]
    pub proxy_direction: i32,
    #[prost(oneof = "tap_event::Event", tags = "3")]
    pub event: ::core::option::Option<tap_event::Event>,
}
/// Nested message and enum types in `TapEvent`.
pub mod tap_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EndpointMeta {
        #[prost(map = "string, string", tag = "1")]
        pub labels: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteMeta {
        #[prost(map = "string, string", tag = "1")]
        pub labels: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http {
        #[prost(oneof = "http::Event", tags = "1, 2, 3")]
        pub event: ::core::option::Option<http::Event>,
    }
    /// Nested message and enum types in `Http`.
    pub mod http {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct StreamId {
            /// A randomized base (stable across a process's runtime)
            #[prost(uint32, tag = "1")]
            pub base: u32,
            /// A stream id unique within the lifetime of `base`.
            #[prost(uint64, tag = "2")]
            pub stream: u64,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RequestInit {
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<StreamId>,
            #[prost(message, optional, tag = "2")]
            pub method: ::core::option::Option<
                super::super::super::http_types::HttpMethod,
            >,
            #[prost(message, optional, tag = "3")]
            pub scheme: ::core::option::Option<super::super::super::http_types::Scheme>,
            #[prost(string, tag = "4")]
            pub authority: ::prost::alloc::string::String,
            #[prost(string, tag = "5")]
            pub path: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "6")]
            pub headers: ::core::option::Option<
                super::super::super::http_types::Headers,
            >,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResponseInit {
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<StreamId>,
            #[prost(message, optional, tag = "2")]
            pub since_request_init: ::core::option::Option<::prost_types::Duration>,
            #[prost(uint32, tag = "3")]
            pub http_status: u32,
            #[prost(message, optional, tag = "4")]
            pub headers: ::core::option::Option<
                super::super::super::http_types::Headers,
            >,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ResponseEnd {
            #[prost(message, optional, tag = "1")]
            pub id: ::core::option::Option<StreamId>,
            #[prost(message, optional, tag = "2")]
            pub since_request_init: ::core::option::Option<::prost_types::Duration>,
            #[prost(message, optional, tag = "3")]
            pub since_response_init: ::core::option::Option<::prost_types::Duration>,
            #[prost(uint64, tag = "4")]
            pub response_bytes: u64,
            #[prost(message, optional, tag = "5")]
            pub eos: ::core::option::Option<super::super::Eos>,
            #[prost(message, optional, tag = "6")]
            pub trailers: ::core::option::Option<
                super::super::super::http_types::Headers,
            >,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Event {
            #[prost(message, tag = "1")]
            RequestInit(RequestInit),
            #[prost(message, tag = "2")]
            ResponseInit(ResponseInit),
            #[prost(message, tag = "3")]
            ResponseEnd(ResponseEnd),
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ProxyDirection {
        Unknown = 0,
        Inbound = 1,
        Outbound = 2,
    }
    impl ProxyDirection {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProxyDirection::Unknown => "UNKNOWN",
                ProxyDirection::Inbound => "INBOUND",
                ProxyDirection::Outbound => "OUTBOUND",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "INBOUND" => Some(Self::Inbound),
                "OUTBOUND" => Some(Self::Outbound),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "3")]
        Http(Http),
    }
}
/// Generated client implementations.
pub mod tap_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A service exposed by proxy instances to setup
    #[derive(Debug, Clone)]
    pub struct TapClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TapClient<T>
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
        ) -> TapClient<InterceptedService<T, F>>
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
            TapClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn observe(
            &mut self,
            request: impl tonic::IntoRequest<super::ObserveRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TapEvent>>,
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
                "/io.linkerd.proxy.tap.Tap/Observe",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("io.linkerd.proxy.tap.Tap", "Observe"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod tap_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TapServer.
    #[async_trait]
    pub trait Tap: Send + Sync + 'static {
        /// Server streaming response type for the Observe method.
        type ObserveStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::TapEvent, tonic::Status>,
            >
            + Send
            + 'static;
        async fn observe(
            &self,
            request: tonic::Request<super::ObserveRequest>,
        ) -> std::result::Result<tonic::Response<Self::ObserveStream>, tonic::Status>;
    }
    /// A service exposed by proxy instances to setup
    #[derive(Debug)]
    pub struct TapServer<T: Tap> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Tap> TapServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TapServer<T>
    where
        T: Tap,
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
                "/io.linkerd.proxy.tap.Tap/Observe" => {
                    #[allow(non_camel_case_types)]
                    struct ObserveSvc<T: Tap>(pub Arc<T>);
                    impl<
                        T: Tap,
                    > tonic::server::ServerStreamingService<super::ObserveRequest>
                    for ObserveSvc<T> {
                        type Response = super::TapEvent;
                        type ResponseStream = T::ObserveStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ObserveRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Tap>::observe(&inner, request).await
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
                        let method = ObserveSvc(inner);
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
    impl<T: Tap> Clone for TapServer<T> {
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
    impl<T: Tap> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Tap> tonic::server::NamedService for TapServer<T> {
        const NAME: &'static str = "io.linkerd.proxy.tap.Tap";
    }
}
