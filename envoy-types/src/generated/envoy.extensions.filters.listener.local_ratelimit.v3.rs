#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalRateLimit {
    /// The prefix to use when emitting :ref:`statistics <config_listener_filters_local_rate_limit_stats>`.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// The token bucket configuration to use for rate limiting all incoming sockets. If the token is available,
    /// the socket will be allowed. If no tokens are available, the socket will be immediately closed.
    ///
    /// .. note::
    /// In the current implementation the token bucket's :ref:`fill_interval <envoy_v3_api_field_type.v3.TokenBucket.fill_interval>` must be >= 50ms to avoid too aggressive
    /// refills.
    #[prost(message, optional, tag = "2")]
    pub token_bucket: ::core::option::Option<
        super::super::super::super::super::r#type::v3::TokenBucket,
    >,
    /// Runtime flag that controls whether the filter is enabled or not. If not specified, defaults
    /// to enabled.
    #[prost(message, optional, tag = "3")]
    pub runtime_enabled: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFeatureFlag,
    >,
}
