#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficSpec {
    /// Uniquely identifies the source proxy workload (e.g., pod name) to the
    /// control plane.
    #[prost(string, tag = "1")]
    pub source_workload: ::prost::alloc::string::String,
    /// Describes a target address, as observed by the proxy.
    #[prost(oneof = "traffic_spec::Target", tags = "2, 3")]
    pub target: ::core::option::Option<traffic_spec::Target>,
}
/// Nested message and enum types in `TrafficSpec`.
pub mod traffic_spec {
    /// Describes a target address, as observed by the proxy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// Indicates the proxy is connecting to a specific IP:port.
        #[prost(message, tag = "2")]
        Addr(super::super::net::TcpAddress),
        /// Indicates the proxy is connecting to a named address (like an HTTP
        /// authority).
        #[prost(string, tag = "3")]
        Authority(::prost::alloc::string::String),
    }
}
/// Outbound policy for a given traffic spec.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutboundPolicy {
    /// Indicates the protocol to use for this target. This will be set to Opaque
    /// if the target has been marked as opaque and will be Discover otherwise.
    #[prost(message, optional, tag = "1")]
    pub protocol: ::core::option::Option<ProxyProtocol>,
    /// Describes the resource for which outbound policy has been discovered.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<super::meta::Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyProtocol {
    #[prost(oneof = "proxy_protocol::Kind", tags = "1, 2, 3, 4, 5")]
    pub kind: ::core::option::Option<proxy_protocol::Kind>,
}
/// Nested message and enum types in `ProxyProtocol`.
pub mod proxy_protocol {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Detect {
        /// Protocol detection timeout.
        #[prost(message, optional, tag = "1")]
        pub timeout: ::core::option::Option<::prost_types::Duration>,
        #[prost(message, optional, tag = "2")]
        pub opaque: ::core::option::Option<Opaque>,
        /// HTTP/1 policy configuration.
        #[prost(message, optional, tag = "3")]
        pub http1: ::core::option::Option<Http1>,
        /// HTTP/2 policy configuration.
        #[prost(message, optional, tag = "4")]
        pub http2: ::core::option::Option<Http2>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Opaque {
        #[prost(message, repeated, tag = "1")]
        pub routes: ::prost::alloc::vec::Vec<super::OpaqueRoute>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http1 {
        #[prost(message, repeated, tag = "1")]
        pub routes: ::prost::alloc::vec::Vec<super::HttpRoute>,
        /// If empty, circuit breaking is not performed.
        #[prost(message, optional, tag = "2")]
        pub failure_accrual: ::core::option::Option<super::FailureAccrual>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Http2 {
        #[prost(message, repeated, tag = "1")]
        pub routes: ::prost::alloc::vec::Vec<super::HttpRoute>,
        /// If empty, circuit breaking is not performed.
        #[prost(message, optional, tag = "2")]
        pub failure_accrual: ::core::option::Option<super::FailureAccrual>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Grpc {
        #[prost(message, repeated, tag = "1")]
        pub routes: ::prost::alloc::vec::Vec<super::GrpcRoute>,
        /// If empty, circuit breaking is not performed.
        #[prost(message, optional, tag = "2")]
        pub failure_accrual: ::core::option::Option<super::FailureAccrual>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        Detect(Detect),
        #[prost(message, tag = "2")]
        Opaque(Opaque),
        /// HTTP/1 policy configuration.
        #[prost(message, tag = "3")]
        Http1(Http1),
        /// HTTP/2 policy configuration.
        #[prost(message, tag = "4")]
        Http2(Http2),
        /// gRPC policy configuration.
        #[prost(message, tag = "5")]
        Grpc(Grpc),
    }
}
/// Outbound-specific HTTP route configuration (based on the
/// [Gateway API](<https://gateway-api.sigs.k8s.io/v1alpha2/references/spec/#gateway.networking.k8s.io/v1alpha2.HTTPRoute>)).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRoute {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::meta::Metadata>,
    /// If empty, the host value is ignored.
    #[prost(message, repeated, tag = "2")]
    pub hosts: ::prost::alloc::vec::Vec<super::http_route::HostMatch>,
    /// Must have at least one rule.
    #[prost(message, repeated, tag = "3")]
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
        #[prost(message, optional, tag = "3")]
        pub backends: ::core::option::Option<Distribution>,
        /// After this time has elapsed since receiving the initial request, any
        /// outstanding request will be cancelled if no response has been received.
        /// If the request is cancelled, a timeout error response will be returned,
        /// and no more retries will be attempted
        ///
        /// If this field is empty, no request timeout is applied.
        #[prost(message, optional, tag = "4")]
        pub request_timeout: ::core::option::Option<::prost_types::Duration>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        #[prost(oneof = "filter::Kind", tags = "1, 2, 3, 4")]
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
            #[prost(message, tag = "4")]
            ResponseHeaderModifier(
                super::super::super::http_route::ResponseHeaderModifier,
            ),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Distribution {
        #[prost(oneof = "distribution::Kind", tags = "1, 2, 3")]
        pub kind: ::core::option::Option<distribution::Kind>,
    }
    /// Nested message and enum types in `Distribution`.
    pub mod distribution {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Empty {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FirstAvailable {
            #[prost(message, repeated, tag = "1")]
            pub backends: ::prost::alloc::vec::Vec<super::RouteBackend>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RandomAvailable {
            #[prost(message, repeated, tag = "1")]
            pub backends: ::prost::alloc::vec::Vec<super::WeightedRouteBackend>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            #[prost(message, tag = "1")]
            Empty(Empty),
            /// Use the first available backend in the list.
            #[prost(message, tag = "2")]
            FirstAvailable(FirstAvailable),
            #[prost(message, tag = "3")]
            RandomAvailable(RandomAvailable),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteBackend {
        #[prost(message, optional, tag = "1")]
        pub backend: ::core::option::Option<super::Backend>,
        #[prost(message, repeated, tag = "3")]
        pub filters: ::prost::alloc::vec::Vec<Filter>,
        /// After this time has elapsed since a request is dispatched to this
        /// backend, any request will be cancelled if no response has been received.
        /// If the request is not retried, a timeout error response is returned.
        ///
        /// If this field is empty, no request timeout is applied.
        #[prost(message, optional, tag = "4")]
        pub request_timeout: ::core::option::Option<::prost_types::Duration>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WeightedRouteBackend {
        #[prost(message, optional, tag = "1")]
        pub backend: ::core::option::Option<RouteBackend>,
        #[prost(uint32, tag = "2")]
        pub weight: u32,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrpcRoute {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::meta::Metadata>,
    /// If empty, the host value is ignored.
    #[prost(message, repeated, tag = "2")]
    pub hosts: ::prost::alloc::vec::Vec<super::http_route::HostMatch>,
    /// Must have at least one rule.
    #[prost(message, repeated, tag = "3")]
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
        #[prost(message, optional, tag = "3")]
        pub backends: ::core::option::Option<Distribution>,
        /// After this time has elapsed since receiving the initial request, any
        /// outstanding request will be cancelled if no response has been received.
        /// If the request is cancelled, a timeout error response will be returned,
        /// and no more retries will be attempted
        ///
        /// If this field is empty, no request timeout is applied.
        #[prost(message, optional, tag = "4")]
        pub request_timeout: ::core::option::Option<::prost_types::Duration>,
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
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Distribution {
        #[prost(oneof = "distribution::Kind", tags = "1, 2, 3")]
        pub kind: ::core::option::Option<distribution::Kind>,
    }
    /// Nested message and enum types in `Distribution`.
    pub mod distribution {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Empty {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FirstAvailable {
            #[prost(message, repeated, tag = "1")]
            pub backends: ::prost::alloc::vec::Vec<super::RouteBackend>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RandomAvailable {
            #[prost(message, repeated, tag = "1")]
            pub backends: ::prost::alloc::vec::Vec<super::WeightedRouteBackend>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            #[prost(message, tag = "1")]
            Empty(Empty),
            /// Use the first available backend in the list.
            #[prost(message, tag = "2")]
            FirstAvailable(FirstAvailable),
            #[prost(message, tag = "3")]
            RandomAvailable(RandomAvailable),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteBackend {
        #[prost(message, optional, tag = "1")]
        pub backend: ::core::option::Option<super::Backend>,
        #[prost(message, repeated, tag = "3")]
        pub filters: ::prost::alloc::vec::Vec<Filter>,
        /// After this time has elapsed since a request is dispatched to this
        /// backend, any request will be cancelled if no response has been received.
        /// If the request is not retried, a timeout error response is returned.
        ///
        /// If this field is empty, no request timeout is applied.
        #[prost(message, optional, tag = "4")]
        pub request_timeout: ::core::option::Option<::prost_types::Duration>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WeightedRouteBackend {
        #[prost(message, optional, tag = "1")]
        pub backend: ::core::option::Option<RouteBackend>,
        #[prost(uint32, tag = "2")]
        pub weight: u32,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpaqueRoute {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::meta::Metadata>,
    /// Must have at least one rule.
    #[prost(message, repeated, tag = "3")]
    pub rules: ::prost::alloc::vec::Vec<opaque_route::Rule>,
}
/// Nested message and enum types in `OpaqueRoute`.
pub mod opaque_route {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rule {
        #[prost(message, optional, tag = "1")]
        pub backends: ::core::option::Option<Distribution>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Distribution {
        #[prost(oneof = "distribution::Kind", tags = "1, 2, 3")]
        pub kind: ::core::option::Option<distribution::Kind>,
    }
    /// Nested message and enum types in `Distribution`.
    pub mod distribution {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Empty {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FirstAvailable {
            #[prost(message, repeated, tag = "1")]
            pub backends: ::prost::alloc::vec::Vec<super::RouteBackend>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RandomAvailable {
            #[prost(message, repeated, tag = "1")]
            pub backends: ::prost::alloc::vec::Vec<super::WeightedRouteBackend>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            #[prost(message, tag = "1")]
            Empty(Empty),
            /// Use the first available backend in the list.
            #[prost(message, tag = "2")]
            FirstAvailable(FirstAvailable),
            #[prost(message, tag = "3")]
            RandomAvailable(RandomAvailable),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteBackend {
        #[prost(message, optional, tag = "1")]
        pub backend: ::core::option::Option<super::Backend>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WeightedRouteBackend {
        #[prost(message, optional, tag = "1")]
        pub backend: ::core::option::Option<RouteBackend>,
        #[prost(uint32, tag = "2")]
        pub weight: u32,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backend {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::meta::Metadata>,
    /// Describes queue configuration for a backend.
    #[prost(message, optional, tag = "4")]
    pub queue: ::core::option::Option<Queue>,
    #[prost(oneof = "backend::Kind", tags = "2, 3")]
    pub kind: ::core::option::Option<backend::Kind>,
}
/// Nested message and enum types in `Backend`.
pub mod backend {
    /// A strategy for discovering endpoints for a service.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EndpointDiscovery {
        #[prost(oneof = "endpoint_discovery::Kind", tags = "1")]
        pub kind: ::core::option::Option<endpoint_discovery::Kind>,
    }
    /// Nested message and enum types in `EndpointDiscovery`.
    pub mod endpoint_discovery {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DestinationGet {
            #[prost(string, tag = "1")]
            pub path: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            /// Use the `Destination` service to discover endpoints for this service.
            #[prost(message, tag = "1")]
            Dst(DestinationGet),
        }
    }
    /// Describes a power-of-two-choices (P2C) load balancer configuration for a
    /// backend.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BalanceP2c {
        #[prost(message, optional, tag = "1")]
        pub discovery: ::core::option::Option<EndpointDiscovery>,
        /// The load estimation strategy used by this load balancer.
        #[prost(oneof = "balance_p2c::Load", tags = "2")]
        pub load: ::core::option::Option<balance_p2c::Load>,
    }
    /// Nested message and enum types in `BalanceP2c`.
    pub mod balance_p2c {
        /// Parameters configuring peak EWMA load estimation.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PeakEwma {
            /// Initial latency value used when no latencies have been
            /// recorded for an endpoint.
            #[prost(message, optional, tag = "1")]
            pub default_rtt: ::core::option::Option<::prost_types::Duration>,
            /// The duration of the moving window over which latency is observed.
            #[prost(message, optional, tag = "2")]
            pub decay: ::core::option::Option<::prost_types::Duration>,
        }
        /// The load estimation strategy used by this load balancer.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Load {
            /// This load balancer uses peak EWMA (exponentially weighted moving
            /// average) load estimates.
            #[prost(message, tag = "2")]
            PeakEwma(PeakEwma),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// A backend that consists of a single endpoint.
        #[prost(message, tag = "2")]
        Forward(super::super::destination::WeightedAddr),
        /// A backend that comprises a load balanced service.
        #[prost(message, tag = "3")]
        Balancer(BalanceP2c),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Queue {
    /// The number of requests that may be held in a queue before backpressure is
    /// exerted.
    #[prost(uint32, tag = "1")]
    pub capacity: u32,
    /// A timeout that limits how long a backend may remain unready before any
    /// requests in its queue are failed.
    #[prost(message, optional, tag = "2")]
    pub failfast_timeout: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailureAccrual {
    #[prost(oneof = "failure_accrual::Kind", tags = "1")]
    pub kind: ::core::option::Option<failure_accrual::Kind>,
}
/// Nested message and enum types in `FailureAccrual`.
pub mod failure_accrual {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConsecutiveFailures {
        #[prost(uint32, tag = "1")]
        pub max_failures: u32,
        #[prost(message, optional, tag = "2")]
        pub backoff: ::core::option::Option<super::ExponentialBackoff>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        ConsecutiveFailures(ConsecutiveFailures),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExponentialBackoff {
    /// The minimum amount of time to wait before resuming an operation.
    #[prost(message, optional, tag = "1")]
    pub min_backoff: ::core::option::Option<::prost_types::Duration>,
    /// The maximum amount of time to wait before resuming an operation.
    /// Must be greater than or equal to min_backoff.
    #[prost(message, optional, tag = "2")]
    pub max_backoff: ::core::option::Option<::prost_types::Duration>,
    /// The ratio of the base timeout that may be randomly added to a backoff.
    /// Must be greater than or equal to 0.0.
    #[prost(float, tag = "3")]
    pub jitter_ratio: f32,
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
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::TrafficSpec>,
        ) -> std::result::Result<tonic::Response<super::OutboundPolicy>, tonic::Status> {
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("io.linkerd.proxy.outbound.OutboundPolicies", "Get"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn watch(
            &mut self,
            request: impl tonic::IntoRequest<super::TrafficSpec>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::OutboundPolicy>>,
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "io.linkerd.proxy.outbound.OutboundPolicies",
                        "Watch",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod outbound_policies_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OutboundPoliciesServer.
    #[async_trait]
    pub trait OutboundPolicies: Send + Sync + 'static {
        async fn get(
            &self,
            request: tonic::Request<super::TrafficSpec>,
        ) -> std::result::Result<tonic::Response<super::OutboundPolicy>, tonic::Status>;
        /// Server streaming response type for the Watch method.
        type WatchStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::OutboundPolicy, tonic::Status>,
            >
            + Send
            + 'static;
        async fn watch(
            &self,
            request: tonic::Request<super::TrafficSpec>,
        ) -> std::result::Result<tonic::Response<Self::WatchStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct OutboundPoliciesServer<T: OutboundPolicies> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
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
        ) -> Poll<std::result::Result<(), Self::Error>> {
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
                    > tonic::server::UnaryService<super::TrafficSpec> for GetSvc<T> {
                        type Response = super::OutboundPolicy;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TrafficSpec>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OutboundPolicies>::get(&inner, request).await
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
                    > tonic::server::ServerStreamingService<super::TrafficSpec>
                    for WatchSvc<T> {
                        type Response = super::OutboundPolicy;
                        type ResponseStream = T::WatchStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TrafficSpec>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OutboundPolicies>::watch(&inner, request).await
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
                        let method = WatchSvc(inner);
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
    impl<T: OutboundPolicies> Clone for OutboundPoliciesServer<T> {
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
    impl<T: OutboundPolicies> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
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
