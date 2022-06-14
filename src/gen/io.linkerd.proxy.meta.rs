/// General metadata about a configuration object. Typically references either an
/// implicit default configuration or a Kubernetes resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(oneof="metadata::Kind", tags="1, 2")]
    pub kind: ::core::option::Option<metadata::Kind>,
}
/// Nested message and enum types in `Metadata`.
pub mod metadata {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        #[prost(string, tag="1")]
        pub group: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub kind: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub name: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(string, tag="1")]
        Default(::prost::alloc::string::String),
        #[prost(message, tag="2")]
        Resource(Resource),
    }
}
