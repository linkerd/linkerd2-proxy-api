/// General metadata about a configuration object. Typically references either an
/// implicit default configuration or a Kubernetes resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(oneof = "metadata::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<metadata::Kind>,
}
/// Nested message and enum types in `Metadata`.
pub mod metadata {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// A name describing a default/implicit configuration.
        ///
        /// For example, a policy default name like `all-authenticated` describes an
        /// implicit controller-implementedc policy that does not exist as a cluster
        /// resource.
        #[prost(string, tag = "1")]
        Default(::prost::alloc::string::String),
        #[prost(message, tag = "2")]
        Resource(super::Resource),
    }
}
/// References a (e.g., Kubernetes) resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(string, tag = "1")]
    pub group: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub section: ::prost::alloc::string::String,
    #[prost(uint32, tag = "6")]
    pub port: u32,
}
