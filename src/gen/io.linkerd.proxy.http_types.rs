#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpMethod {
    #[prost(oneof="http_method::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<http_method::Type>,
}
/// Nested message and enum types in `HttpMethod`.
pub mod http_method {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Registered {
        Get = 0,
        Post = 1,
        Put = 2,
        Delete = 3,
        Patch = 4,
        Options = 5,
        Connect = 6,
        Head = 7,
        Trace = 8,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(enumeration="Registered", tag="1")]
        Registered(i32),
        #[prost(string, tag="2")]
        Unregistered(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scheme {
    #[prost(oneof="scheme::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<scheme::Type>,
}
/// Nested message and enum types in `Scheme`.
pub mod scheme {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Registered {
        Http = 0,
        Https = 1,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(enumeration="Registered", tag="1")]
        Registered(i32),
        #[prost(string, tag="2")]
        Unregistered(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Headers {
    #[prost(message, repeated, tag="1")]
    pub headers: ::prost::alloc::vec::Vec<headers::Header>,
}
/// Nested message and enum types in `Headers`.
pub mod headers {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Header {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(bytes="vec", tag="2")]
        pub value: ::prost::alloc::vec::Vec<u8>,
    }
}
