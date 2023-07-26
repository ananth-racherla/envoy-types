#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionLimit {
    /// The prefix to use when emitting :ref:`statistics <config_network_filters_connection_limit_stats>`.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// The max connections configuration to use for new incoming connections that are processed
    /// by the filter's filter chain. When max_connection is reached, the incoming connection
    /// will be closed after delay duration.
    #[prost(message, optional, tag = "2")]
    pub max_connections: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt64Value,
    >,
    /// The delay configuration to use for rejecting the connection after some specified time duration
    /// instead of immediately rejecting the connection. That way, a malicious user is not able to
    /// retry as fast as possible which provides a better DoS protection for Envoy. If this is not present,
    /// the connection will be closed immediately.
    #[prost(message, optional, tag = "3")]
    pub delay: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Runtime flag that controls whether the filter is enabled or not. If not specified, defaults
    /// to enabled.
    #[prost(message, optional, tag = "4")]
    pub runtime_enabled: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFeatureFlag,
    >,
}
