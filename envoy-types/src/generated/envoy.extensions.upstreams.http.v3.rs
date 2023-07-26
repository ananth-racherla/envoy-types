/// HttpProtocolOptions specifies Http upstream protocol options. This object
/// is used in
/// :ref:`typed_extension_protocol_options<envoy_v3_api_field_config.cluster.v3.Cluster.typed_extension_protocol_options>`,
/// keyed by the name `envoy.extensions.upstreams.http.v3.HttpProtocolOptions`.
///
/// This controls what protocol(s) should be used for upstream and how said protocol(s) are configured.
///
/// This replaces the prior pattern of explicit protocol configuration directly
/// in the cluster. So a configuration like this, explicitly configuring the use of HTTP/2 upstream:
///
/// .. code::
///
/// clusters:
/// - name: some_service
/// connect_timeout: 5s
/// upstream_http_protocol_options:
/// auto_sni: true
/// common_http_protocol_options:
/// idle_timeout: 1s
/// http2_protocol_options:
/// max_concurrent_streams: 100
/// .... \[further cluster config\]
///
/// Would now look like this:
///
/// .. code::
///
/// clusters:
/// - name: some_service
/// connect_timeout: 5s
/// typed_extension_protocol_options:
/// envoy.extensions.upstreams.http.v3.HttpProtocolOptions:
/// "@type": type.googleapis.com/envoy.extensions.upstreams.http.v3.HttpProtocolOptions
/// upstream_http_protocol_options:
/// auto_sni: true
/// common_http_protocol_options:
/// idle_timeout: 1s
/// explicit_http_config:
/// http2_protocol_options:
/// max_concurrent_streams: 100
/// .... \[further cluster config\]
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpProtocolOptions {
    /// This contains options common across HTTP/1 and HTTP/2
    #[prost(message, optional, tag = "1")]
    pub common_http_protocol_options: ::core::option::Option<
        super::super::super::super::config::core::v3::HttpProtocolOptions,
    >,
    /// This contains common protocol options which are only applied upstream.
    #[prost(message, optional, tag = "2")]
    pub upstream_http_protocol_options: ::core::option::Option<
        super::super::super::super::config::core::v3::UpstreamHttpProtocolOptions,
    >,
    /// .. note::
    /// Upstream HTTP filters are currently in alpha.
    ///
    /// Optional HTTP filters for the upstream filter chain.
    ///
    /// These filters will be applied for all HTTP streams which flow through this
    /// cluster. Unlike downstream filters, they will *not* be applied to terminated CONNECT requests.
    ///
    /// If using upstream filters, please be aware that local errors sent by
    /// upstream filters will not trigger retries, and local errors sent by
    /// upstream filters will count as a final response if hedging is configured.
    /// \[\#extension-category: envoy.filters.http.upstream\]
    #[prost(message, repeated, tag = "6")]
    pub http_filters: ::prost::alloc::vec::Vec<
        super::super::super::filters::network::http_connection_manager::v3::HttpFilter,
    >,
    /// Configuration options for Unified Header Validation (UHV).
    /// UHV is an extensible mechanism for checking validity of HTTP responses.
    ///
    /// \[\#comment:TODO(yanavlasov): Make it a link to the default header validator doc when it becomes visible.\]
    /// Leaving this field unspecified, selects the default header validator `envoy.http.header_validators.envoy_default`.
    ///
    /// \\[\#not-implemented-hide:\\]
    /// \[\#extension-category: envoy.http.header_validators\]
    #[prost(message, optional, tag = "7")]
    pub header_validation_config: ::core::option::Option<
        super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    /// This controls the actual protocol to be used upstream.
    #[prost(oneof = "http_protocol_options::UpstreamProtocolOptions", tags = "3, 4, 5")]
    pub upstream_protocol_options: ::core::option::Option<
        http_protocol_options::UpstreamProtocolOptions,
    >,
}
/// Nested message and enum types in `HttpProtocolOptions`.
pub mod http_protocol_options {
    /// If this is used, the cluster will only operate on one of the possible upstream protocols.
    /// Note that HTTP/2 or above should generally be used for upstream gRPC clusters.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExplicitHttpConfig {
        #[prost(oneof = "explicit_http_config::ProtocolConfig", tags = "1, 2, 3")]
        pub protocol_config: ::core::option::Option<
            explicit_http_config::ProtocolConfig,
        >,
    }
    /// Nested message and enum types in `ExplicitHttpConfig`.
    pub mod explicit_http_config {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ProtocolConfig {
            #[prost(message, tag = "1")]
            HttpProtocolOptions(
                super::super::super::super::super::super::config::core::v3::Http1ProtocolOptions,
            ),
            #[prost(message, tag = "2")]
            Http2ProtocolOptions(
                super::super::super::super::super::super::config::core::v3::Http2ProtocolOptions,
            ),
            /// .. warning::
            /// QUIC upstream support is currently not ready for internet use.
            /// Please see :ref:`here <arch_overview_http3>` for details.
            #[prost(message, tag = "3")]
            Http3ProtocolOptions(
                super::super::super::super::super::super::config::core::v3::Http3ProtocolOptions,
            ),
        }
    }
    /// If this is used, the cluster can use either of the configured protocols, and
    /// will use whichever protocol was used by the downstream connection.
    ///
    /// If HTTP/3 is configured for downstream and not configured for upstream,
    /// HTTP/3 requests will fail over to HTTP/2.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UseDownstreamHttpConfig {
        #[prost(message, optional, tag = "1")]
        pub http_protocol_options: ::core::option::Option<
            super::super::super::super::super::config::core::v3::Http1ProtocolOptions,
        >,
        #[prost(message, optional, tag = "2")]
        pub http2_protocol_options: ::core::option::Option<
            super::super::super::super::super::config::core::v3::Http2ProtocolOptions,
        >,
        /// .. warning::
        /// QUIC upstream support is currently not ready for internet use.
        /// Please see :ref:`here <arch_overview_http3>` for details.
        #[prost(message, optional, tag = "3")]
        pub http3_protocol_options: ::core::option::Option<
            super::super::super::super::super::config::core::v3::Http3ProtocolOptions,
        >,
    }
    /// If this is used, the cluster can use either HTTP/1 or HTTP/2, and will use whichever
    /// protocol is negotiated by ALPN with the upstream.
    /// Clusters configured with `AutoHttpConfig` will use the highest available
    /// protocol; HTTP/2 if supported, otherwise HTTP/1.
    /// If the upstream does not support ALPN, `AutoHttpConfig` will fail over to HTTP/1.
    /// This can only be used with transport sockets which support ALPN. Using a
    /// transport socket which does not support ALPN will result in configuration
    /// failure. The transport layer may be configured with custom ALPN, but the default ALPN
    /// for the cluster (or if custom ALPN fails) will be "h2,http/1.1".
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AutoHttpConfig {
        #[prost(message, optional, tag = "1")]
        pub http_protocol_options: ::core::option::Option<
            super::super::super::super::super::config::core::v3::Http1ProtocolOptions,
        >,
        #[prost(message, optional, tag = "2")]
        pub http2_protocol_options: ::core::option::Option<
            super::super::super::super::super::config::core::v3::Http2ProtocolOptions,
        >,
        /// Unlike HTTP/1 and HTTP/2, HTTP/3 will not be configured unless it is
        /// present, and (soon) only if there is an indication of server side
        /// support.
        /// See :ref:`here <arch_overview_http3_upstream>` for more information on
        /// when HTTP/3 will be used, and when Envoy will fail over to TCP.
        ///
        /// .. warning::
        /// QUIC upstream support is currently not ready for internet use.
        /// Please see :ref:`here <arch_overview_http3>` for details.
        #[prost(message, optional, tag = "3")]
        pub http3_protocol_options: ::core::option::Option<
            super::super::super::super::super::config::core::v3::Http3ProtocolOptions,
        >,
        /// The presence of alternate protocols cache options causes the use of the
        /// alternate protocols cache, which is responsible for parsing and caching
        /// HTTP Alt-Svc headers. This enables the use of HTTP/3 for origins that
        /// advertise supporting it.
        ///
        /// .. note::
        /// This is required when HTTP/3 is enabled.
        #[prost(message, optional, tag = "4")]
        pub alternate_protocols_cache_options: ::core::option::Option<
            super::super::super::super::super::config::core::v3::AlternateProtocolsCacheOptions,
        >,
    }
    /// This controls the actual protocol to be used upstream.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UpstreamProtocolOptions {
        /// To explicitly configure either HTTP/1 or HTTP/2 (but not both!) use `explicit_http_config`.
        /// If the `explicit_http_config` is empty, HTTP/1.1 is used.
        #[prost(message, tag = "3")]
        ExplicitHttpConfig(ExplicitHttpConfig),
        /// This allows switching on protocol based on what protocol the downstream
        /// connection used.
        #[prost(message, tag = "4")]
        UseDownstreamProtocolConfig(UseDownstreamHttpConfig),
        /// This allows switching on protocol based on ALPN
        #[prost(message, tag = "5")]
        AutoConfig(AutoHttpConfig),
    }
}
