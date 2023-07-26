/// Cors filter config. Set this in
/// ref:`http_filters <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.http_filters>`
/// to enable the CORS filter.
///
/// Please note that the :ref:`CorsPolicy <envoy_v3_api_msg_extensions.filters.http.cors.v3.CorsPolicy>`
/// must be configured in the `RouteConfiguration` as `typed_per_filter_config` at some level to make the filter work.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cors {}
/// Per route configuration for the CORS filter. This configuration should be configured in the `RouteConfiguration` as `typed_per_filter_config` at some level to
/// make the filter work.
/// \[\#next-free-field: 10\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CorsPolicy {
    /// Specifies string patterns that match allowed origins. An origin is allowed if any of the
    /// string matchers match.
    #[prost(message, repeated, tag = "1")]
    pub allow_origin_string_match: ::prost::alloc::vec::Vec<
        super::super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    /// Specifies the content for the `access-control-allow-methods` header.
    #[prost(string, tag = "2")]
    pub allow_methods: ::prost::alloc::string::String,
    /// Specifies the content for the `access-control-allow-headers` header.
    #[prost(string, tag = "3")]
    pub allow_headers: ::prost::alloc::string::String,
    /// Specifies the content for the `access-control-expose-headers` header.
    #[prost(string, tag = "4")]
    pub expose_headers: ::prost::alloc::string::String,
    /// Specifies the content for the `access-control-max-age` header.
    #[prost(string, tag = "5")]
    pub max_age: ::prost::alloc::string::String,
    /// Specifies whether the resource allows credentials.
    #[prost(message, optional, tag = "6")]
    pub allow_credentials: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Specifies the % of requests for which the CORS filter is enabled.
    ///
    /// If neither `filter_enabled`, nor `shadow_enabled` are specified, the CORS
    /// filter will be enabled for 100% of the requests.
    ///
    /// If :ref:`runtime_key <envoy_v3_api_field_config.core.v3.RuntimeFractionalPercent.runtime_key>` is
    /// specified, Envoy will lookup the runtime key to get the percentage of requests to filter.
    #[prost(message, optional, tag = "7")]
    pub filter_enabled: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFractionalPercent,
    >,
    /// Specifies the % of requests for which the CORS policies will be evaluated and tracked, but not
    /// enforced.
    ///
    /// This field is intended to be used when `filter_enabled` is off. That field have to explicitly disable
    /// the filter in order for this setting to take effect.
    ///
    /// If :ref:`runtime_key <envoy_v3_api_field_config.core.v3.RuntimeFractionalPercent.runtime_key>` is specified,
    /// Envoy will lookup the runtime key to get the percentage of requests for which it will evaluate
    /// and track the request's `Origin` to determine if it's valid but will not enforce any policies.
    #[prost(message, optional, tag = "8")]
    pub shadow_enabled: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFractionalPercent,
    >,
    /// Specify whether allow requests whose target server's IP address is more private than that from
    /// which the request initiator was fetched.
    ///
    /// More details refer to <https://developer.chrome.com/blog/private-network-access-preflight.>
    #[prost(message, optional, tag = "9")]
    pub allow_private_network_access: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
}
