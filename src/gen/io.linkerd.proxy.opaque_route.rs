/// Used to indicate that the policy is invalid.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Invalid {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Used to indicate that traffic is forbidden.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Forbidden {}
