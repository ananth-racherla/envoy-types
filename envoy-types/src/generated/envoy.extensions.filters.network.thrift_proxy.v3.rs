#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteConfiguration {
    /// The name of the route configuration. This name is used in asynchronous route discovery.
    /// For example, it might match
    /// :ref:`route_config_name <envoy_v3_api_field_extensions.filters.network.thrift_proxy.v3.Trds.route_config_name>` in
    /// :ref:`envoy_v3_api_msg_extensions.filters.network.thrift_proxy.v3.Trds`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The list of routes that will be matched, in order, against incoming requests. The first route
    /// that matches will be used.
    #[prost(message, repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<Route>,
    /// An optional boolean that specifies whether the clusters that the route
    /// table refers to will be validated by the cluster manager. If set to true
    /// and a route refers to a non-existent cluster, the route table will not
    /// load. If set to false and a route refers to a non-existent cluster, the
    /// route table will load and the router filter will return a INTERNAL_ERROR
    /// if the route is selected at runtime. This setting defaults to true if the route table
    /// is statically defined via the :ref:`route_config <envoy_v3_api_field_extensions.filters.network.thrift_proxy.v3.ThriftProxy.route_config>`
    /// option. This setting default to false if the route table is loaded dynamically via the
    /// :ref:`trds <envoy_v3_api_field_extensions.filters.network.thrift_proxy.v3.ThriftProxy.trds>`
    /// option. Users may wish to override the default behavior in certain cases (for example when
    /// using CDS with a static route table).
    #[prost(message, optional, tag = "3")]
    pub validate_clusters: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// Route matching parameters.
    #[prost(message, optional, tag = "1")]
    pub r#match: ::core::option::Option<RouteMatch>,
    /// Route request to some upstream cluster.
    #[prost(message, optional, tag = "2")]
    pub route: ::core::option::Option<RouteAction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteMatch {
    /// Inverts whatever matching is done in the :ref:`method_name <envoy_v3_api_field_extensions.filters.network.thrift_proxy.v3.RouteMatch.method_name>` or
    /// :ref:`service_name <envoy_v3_api_field_extensions.filters.network.thrift_proxy.v3.RouteMatch.service_name>` fields.
    /// Cannot be combined with wildcard matching as that would result in routes never being matched.
    ///
    /// .. note::
    ///
    /// This does not invert matching done as part of the :ref:`headers field <envoy_v3_api_field_extensions.filters.network.thrift_proxy.v3.RouteMatch.headers>` field. To
    /// invert header matching, see :ref:`invert_match <envoy_v3_api_field_config.route.v3.HeaderMatcher.invert_match>`.
    #[prost(bool, tag = "3")]
    pub invert: bool,
    /// Specifies a set of headers that the route should match on. The router will check the request’s
    /// headers against all the specified headers in the route config. A match will happen if all the
    /// headers in the route are present in the request with the same values (or based on presence if
    /// the value field is not in the config). Note that this only applies for Thrift transports and/or
    /// protocols that support headers.
    #[prost(message, repeated, tag = "4")]
    pub headers: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::route::v3::HeaderMatcher,
    >,
    #[prost(oneof = "route_match::MatchSpecifier", tags = "1, 2")]
    pub match_specifier: ::core::option::Option<route_match::MatchSpecifier>,
}
/// Nested message and enum types in `RouteMatch`.
pub mod route_match {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchSpecifier {
        /// If specified, the route must exactly match the request method name. As a special case, an
        /// empty string matches any request method name.
        #[prost(string, tag = "1")]
        MethodName(::prost::alloc::string::String),
        /// If specified, the route must have the service name as the request method name prefix. As a
        /// special case, an empty string matches any service name. Only relevant when service
        /// multiplexing.
        #[prost(string, tag = "2")]
        ServiceName(::prost::alloc::string::String),
    }
}
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteAction {
    /// Optional endpoint metadata match criteria used by the subset load balancer. Only endpoints in
    /// the upstream cluster with metadata matching what is set in this field will be considered.
    /// Note that this will be merged with what's provided in :ref:`WeightedCluster.metadata_match <envoy_v3_api_field_extensions.filters.network.thrift_proxy.v3.WeightedCluster.ClusterWeight.metadata_match>`,
    /// with values there taking precedence. Keys and values should be provided under the "envoy.lb"
    /// metadata key.
    #[prost(message, optional, tag = "3")]
    pub metadata_match: ::core::option::Option<
        super::super::super::super::super::config::core::v3::Metadata,
    >,
    /// Specifies a set of rate limit configurations that could be applied to the route.
    /// N.B. Thrift service or method name matching can be achieved by specifying a RequestHeaders
    /// action with the header name ":method-name".
    #[prost(message, repeated, tag = "4")]
    pub rate_limits: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::route::v3::RateLimit,
    >,
    /// Strip the service prefix from the method name, if there's a prefix. For
    /// example, the method call Service:method would end up being just method.
    #[prost(bool, tag = "5")]
    pub strip_service_name: bool,
    /// Indicates that the route has request mirroring policies.
    #[prost(message, repeated, tag = "7")]
    pub request_mirror_policies: ::prost::alloc::vec::Vec<
        route_action::RequestMirrorPolicy,
    >,
    #[prost(oneof = "route_action::ClusterSpecifier", tags = "1, 2, 6")]
    pub cluster_specifier: ::core::option::Option<route_action::ClusterSpecifier>,
}
/// Nested message and enum types in `RouteAction`.
pub mod route_action {
    /// The router is capable of shadowing traffic from one cluster to another. The current
    /// implementation is "fire and forget," meaning Envoy will not wait for the shadow cluster to
    /// respond before returning the response from the primary cluster. All normal statistics are
    /// collected for the shadow cluster making this feature useful for testing.
    ///
    /// .. note::
    ///
    /// Shadowing will not be triggered if the primary cluster does not exist.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RequestMirrorPolicy {
        /// Specifies the cluster that requests will be mirrored to. The cluster must
        /// exist in the cluster manager configuration when the route configuration is loaded.
        /// If it disappears at runtime, the shadow request will silently be ignored.
        #[prost(string, tag = "1")]
        pub cluster: ::prost::alloc::string::String,
        /// If not specified, all requests to the target cluster will be mirrored.
        ///
        /// For some fraction N/D, a random number in the range \[0,D) is selected. If the
        /// number is \<= the value of the numerator N, or if the key is not present, the default
        /// value, the request will be mirrored.
        #[prost(message, optional, tag = "2")]
        pub runtime_fraction: ::core::option::Option<
            super::super::super::super::super::super::config::core::v3::RuntimeFractionalPercent,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterSpecifier {
        /// Indicates a single upstream cluster to which the request should be routed
        /// to.
        #[prost(string, tag = "1")]
        Cluster(::prost::alloc::string::String),
        /// Multiple upstream clusters can be specified for a given route. The
        /// request is routed to one of the upstream clusters based on weights
        /// assigned to each cluster.
        #[prost(message, tag = "2")]
        WeightedClusters(super::WeightedCluster),
        /// Envoy will determine the cluster to route to by reading the value of the
        /// Thrift header named by cluster_header from the request headers. If the
        /// header is not found or the referenced cluster does not exist Envoy will
        /// respond with an unknown method exception or an internal error exception,
        /// respectively.
        #[prost(string, tag = "6")]
        ClusterHeader(::prost::alloc::string::String),
    }
}
/// Allows for specification of multiple upstream clusters along with weights that indicate the
/// percentage of traffic to be forwarded to each cluster. The router selects an upstream cluster
/// based on these weights.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightedCluster {
    /// Specifies one or more upstream clusters associated with the route.
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<weighted_cluster::ClusterWeight>,
}
/// Nested message and enum types in `WeightedCluster`.
pub mod weighted_cluster {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterWeight {
        /// Name of the upstream cluster.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// When a request matches the route, the choice of an upstream cluster is determined by its
        /// weight. The sum of weights across all entries in the clusters array determines the total
        /// weight.
        #[prost(message, optional, tag = "2")]
        pub weight: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        /// Optional endpoint metadata match criteria used by the subset load balancer. Only endpoints in
        /// the upstream cluster with metadata matching what is set in this field, combined with what's
        /// provided in :ref:`RouteAction's metadata_match <envoy_v3_api_field_extensions.filters.network.thrift_proxy.v3.RouteAction.metadata_match>`,
        /// will be considered. Values here will take precedence. Keys and values should be provided
        /// under the "envoy.lb" metadata key.
        #[prost(message, optional, tag = "3")]
        pub metadata_match: ::core::option::Option<
            super::super::super::super::super::super::config::core::v3::Metadata,
        >,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trds {
    /// Configuration source specifier.
    /// In case of `api_config_source` only aggregated `api_type` is supported.
    #[prost(message, optional, tag = "1")]
    pub config_source: ::core::option::Option<
        super::super::super::super::super::config::core::v3::ConfigSource,
    >,
    /// The name of the route configuration. This allows to use different route
    /// configurations. Tells which route configuration should be fetched from the configuration source.
    /// Leave unspecified is also valid and means the unnamed route configuration.
    #[prost(string, tag = "2")]
    pub route_config_name: ::prost::alloc::string::String,
}
/// \[\#next-free-field: 11\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThriftProxy {
    /// Supplies the type of transport that the Thrift proxy should use. Defaults to
    /// :ref:`AUTO_TRANSPORT<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.TransportType.AUTO_TRANSPORT>`.
    #[prost(enumeration = "TransportType", tag = "2")]
    pub transport: i32,
    /// Supplies the type of protocol that the Thrift proxy should use. Defaults to
    /// :ref:`AUTO_PROTOCOL<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.AUTO_PROTOCOL>`.
    #[prost(enumeration = "ProtocolType", tag = "3")]
    pub protocol: i32,
    /// The human readable prefix to use when emitting statistics.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// The route table for the connection manager is static and is specified in this property.
    /// It is invalid to define both `route_config` and `trds`.
    #[prost(message, optional, tag = "4")]
    pub route_config: ::core::option::Option<RouteConfiguration>,
    /// Use xDS to fetch the route configuration. It is invalid to define both `route_config` and `trds`.
    #[prost(message, optional, tag = "8")]
    pub trds: ::core::option::Option<Trds>,
    /// A list of individual Thrift filters that make up the filter chain for requests made to the
    /// Thrift proxy. Order matters as the filters are processed sequentially. For backwards
    /// compatibility, if no thrift_filters are specified, a default Thrift router filter
    /// (`envoy.filters.thrift.router`) is used.
    /// \[\#extension-category: envoy.thrift_proxy.filters\]
    #[prost(message, repeated, tag = "5")]
    pub thrift_filters: ::prost::alloc::vec::Vec<ThriftFilter>,
    /// If set to true, Envoy will try to skip decode data after metadata in the Thrift message.
    /// This mode will only work if the upstream and downstream protocols are the same and the transports
    /// are Framed or Header, and the protocol is not Twitter. Otherwise Envoy will
    /// fallback to decode the data.
    #[prost(bool, tag = "6")]
    pub payload_passthrough: bool,
    /// Optional maximum requests for a single downstream connection. If not specified, there is no limit.
    #[prost(message, optional, tag = "7")]
    pub max_requests_per_connection: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Configuration for :ref:`access logs <arch_overview_access_logs>`
    /// emitted by Thrift proxy.
    #[prost(message, repeated, tag = "9")]
    pub access_log: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::accesslog::v3::AccessLog,
    >,
    /// If set to true, Envoy will preserve the case of Thrift header keys instead of serializing them to
    /// lower case as per the default behavior. Note that NUL, CR and LF characters will also be preserved
    /// as mandated by the Thrift spec.
    ///
    /// More info: <https://github.com/apache/thrift/commit/e165fa3c85d00cb984f4d9635ed60909a1266ce1.>
    #[prost(bool, tag = "10")]
    pub header_keys_preserve_case: bool,
}
/// ThriftFilter configures a Thrift filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThriftFilter {
    /// The name of the filter to instantiate. The name must match a supported
    /// filter. The built-in filters are:
    ///
    /// \[\#comment:TODO(zuercher): Auto generate the following list\]
    ///
    /// * :ref:`envoy.filters.thrift.router <config_thrift_filters_router>`
    /// * :ref:`envoy.filters.thrift.rate_limit <config_thrift_filters_rate_limit>`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Filter specific configuration which depends on the filter being instantiated. See the supported
    /// filters for further documentation.
    #[prost(oneof = "thrift_filter::ConfigType", tags = "3")]
    pub config_type: ::core::option::Option<thrift_filter::ConfigType>,
}
/// Nested message and enum types in `ThriftFilter`.
pub mod thrift_filter {
    /// Filter specific configuration which depends on the filter being instantiated. See the supported
    /// filters for further documentation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag = "3")]
        TypedConfig(
            super::super::super::super::super::super::super::google::protobuf::Any,
        ),
    }
}
/// ThriftProtocolOptions specifies Thrift upstream protocol options. This object is used in
/// in
/// :ref:`typed_extension_protocol_options<envoy_v3_api_field_config.cluster.v3.Cluster.typed_extension_protocol_options>`,
/// keyed by the name `envoy.filters.network.thrift_proxy`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThriftProtocolOptions {
    /// Supplies the type of transport that the Thrift proxy should use for upstream connections.
    /// Selecting
    /// :ref:`AUTO_TRANSPORT<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.TransportType.AUTO_TRANSPORT>`,
    /// which is the default, causes the proxy to use the same transport as the downstream connection.
    #[prost(enumeration = "TransportType", tag = "1")]
    pub transport: i32,
    /// Supplies the type of protocol that the Thrift proxy should use for upstream connections.
    /// Selecting
    /// :ref:`AUTO_PROTOCOL<envoy_v3_api_enum_value_extensions.filters.network.thrift_proxy.v3.ProtocolType.AUTO_PROTOCOL>`,
    /// which is the default, causes the proxy to use the same protocol as the downstream connection.
    #[prost(enumeration = "ProtocolType", tag = "2")]
    pub protocol: i32,
}
/// Thrift transport types supported by Envoy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransportType {
    /// For downstream connections, the Thrift proxy will attempt to determine which transport to use.
    /// For upstream connections, the Thrift proxy will use same transport as the downstream
    /// connection.
    AutoTransport = 0,
    /// The Thrift proxy will use the Thrift framed transport.
    Framed = 1,
    /// The Thrift proxy will use the Thrift unframed transport.
    Unframed = 2,
    /// The Thrift proxy will assume the client is using the Thrift header transport.
    Header = 3,
}
impl TransportType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransportType::AutoTransport => "AUTO_TRANSPORT",
            TransportType::Framed => "FRAMED",
            TransportType::Unframed => "UNFRAMED",
            TransportType::Header => "HEADER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTO_TRANSPORT" => Some(Self::AutoTransport),
            "FRAMED" => Some(Self::Framed),
            "UNFRAMED" => Some(Self::Unframed),
            "HEADER" => Some(Self::Header),
            _ => None,
        }
    }
}
/// Thrift Protocol types supported by Envoy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtocolType {
    /// For downstream connections, the Thrift proxy will attempt to determine which protocol to use.
    /// Note that the older, non-strict (or lax) binary protocol is not included in automatic protocol
    /// detection. For upstream connections, the Thrift proxy will use the same protocol as the
    /// downstream connection.
    AutoProtocol = 0,
    /// The Thrift proxy will use the Thrift binary protocol.
    Binary = 1,
    /// The Thrift proxy will use Thrift non-strict binary protocol.
    LaxBinary = 2,
    /// The Thrift proxy will use the Thrift compact protocol.
    Compact = 3,
    /// The Thrift proxy will use the Thrift "Twitter" protocol implemented by the finagle library.
    Twitter = 4,
}
impl ProtocolType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtocolType::AutoProtocol => "AUTO_PROTOCOL",
            ProtocolType::Binary => "BINARY",
            ProtocolType::LaxBinary => "LAX_BINARY",
            ProtocolType::Compact => "COMPACT",
            ProtocolType::Twitter => "TWITTER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTO_PROTOCOL" => Some(Self::AutoProtocol),
            "BINARY" => Some(Self::Binary),
            "LAX_BINARY" => Some(Self::LaxBinary),
            "COMPACT" => Some(Self::Compact),
            "TWITTER" => Some(Self::Twitter),
            _ => None,
        }
    }
}
