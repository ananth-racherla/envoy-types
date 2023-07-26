/// Configuration for KillRequest filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KillRequest {
    /// The probability that a Kill request will be triggered.
    #[prost(message, optional, tag = "1")]
    pub probability: ::core::option::Option<
        super::super::super::super::super::r#type::v3::FractionalPercent,
    >,
    /// The name of the kill request header. If this field is not empty, it will override the :ref:`default header <config_http_filters_kill_request_http_header>` name. Otherwise the default header name will be used.
    #[prost(string, tag = "2")]
    pub kill_request_header: ::prost::alloc::string::String,
    #[prost(enumeration = "kill_request::Direction", tag = "3")]
    pub direction: i32,
}
/// Nested message and enum types in `KillRequest`.
pub mod kill_request {
    /// On which direction should the filter check for the `kill_request_header`.
    /// Default to `REQUEST` if unspecified.
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
    pub enum Direction {
        Request = 0,
        Response = 1,
    }
    impl Direction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Direction::Request => "REQUEST",
                Direction::Response => "RESPONSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REQUEST" => Some(Self::Request),
                "RESPONSE" => Some(Self::Response),
                _ => None,
            }
        }
    }
}
