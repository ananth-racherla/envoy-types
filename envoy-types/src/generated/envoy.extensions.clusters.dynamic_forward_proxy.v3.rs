/// Configuration for the dynamic forward proxy cluster. See the :ref:`architecture overview <arch_overview_http_dynamic_forward_proxy>` for more information.
/// \[\#extension: envoy.clusters.dynamic_forward_proxy\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterConfig {
    /// If true allow the cluster configuration to disable the auto_sni and auto_san_validation options
    /// in the :ref:`cluster's upstream_http_protocol_options <envoy_v3_api_field_config.cluster.v3.Cluster.upstream_http_protocol_options>`
    #[prost(bool, tag = "2")]
    pub allow_insecure_cluster_options: bool,
    /// If true allow HTTP/2 and HTTP/3 connections to be reused for requests to different
    /// origins than the connection was initially created for. This will only happen when the
    /// resolved address for the new connection matches the peer address of the connection and
    /// the TLS certificate is also valid for the new hostname. For example, if a connection
    /// has previously been established to foo.example.com at IP 1.2.3.4 with a certificate
    /// that is valid for `*.example.com`, then this connection could be used for requests to
    /// bar.example.com if that also resolved to 1.2.3.4.
    ///
    /// .. note::
    /// By design, this feature will maximize reuse of connections. This means that instead
    /// opening a new connection when an existing connection reaches the maximum number of
    /// concurrent streams, requests will instead be sent to the existing connection.
    ///
    /// .. note::
    /// The coalesced connections might be to upstreams that would not be otherwise
    /// selected by Envoy. See the section `Connection Reuse in RFC 7540 <<https://datatracker.ietf.org/doc/html/rfc7540#section-9.1.1>`\_>
    #[prost(bool, tag = "3")]
    pub allow_coalesced_connections: bool,
    #[prost(oneof = "cluster_config::ClusterImplementationSpecifier", tags = "1, 4")]
    pub cluster_implementation_specifier: ::core::option::Option<
        cluster_config::ClusterImplementationSpecifier,
    >,
}
/// Nested message and enum types in `ClusterConfig`.
pub mod cluster_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterImplementationSpecifier {
        /// The DNS cache configuration that the cluster will attach to. Note this configuration must
        /// match that of associated :ref:`dynamic forward proxy HTTP filter configuration <envoy_v3_api_field_extensions.filters.http.dynamic_forward_proxy.v3.FilterConfig.dns_cache_config>`.
        #[prost(message, tag = "1")]
        DnsCacheConfig(
            super::super::super::super::common::dynamic_forward_proxy::v3::DnsCacheConfig,
        ),
        /// Configuration for sub clusters, when this configuration is enabled,
        /// Envoy will create an independent sub cluster dynamically for each host:port.
        /// Most of the configuration of a sub cluster is inherited from the current cluster,
        /// i.e. health_checks, dns_resolvers and etc.
        /// And the load_assignment will be set to the only one endpoint, host:port.
        ///
        /// Compared to the dns_cache_config, it has the following advantages:
        ///
        /// 1. sub clusters will be created with the STRICT_DNS DiscoveryType,
        ///    so that Envoy will use all of the IPs resolved from the host.
        ///
        /// 1. each sub cluster is full featured cluster, with lb_policy and health check and etc enabled.
        #[prost(message, tag = "4")]
        SubClustersConfig(super::SubClustersConfig),
    }
}
/// Configuration for sub clusters. Hard code STRICT_DNS cluster type now.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubClustersConfig {
    /// The :ref:`load balancer type <arch_overview_load_balancing_types>` to use
    /// when picking a host in a sub cluster. Note that CLUSTER_PROVIDED is not allowed here.
    #[prost(
        enumeration = "super::super::super::super::config::cluster::v3::cluster::LbPolicy",
        tag = "1"
    )]
    pub lb_policy: i32,
    /// The maximum number of sub clusters that the DFP cluster will hold. If not specified defaults to 1024.
    #[prost(message, optional, tag = "2")]
    pub max_sub_clusters: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// The TTL for sub clusters that are unused. Sub clusters that have not been used in the configured time
    /// interval will be purged. If not specified defaults to 5m.
    #[prost(message, optional, tag = "3")]
    pub sub_cluster_ttl: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Sub clusters that should be created & warmed upon creation. This might provide a
    /// performance improvement, in the form of cache hits, for sub clusters that are going to be
    /// warmed during steady state and are known at config load time.
    #[prost(message, repeated, tag = "4")]
    pub preresolve_clusters: ::prost::alloc::vec::Vec<
        super::super::super::super::config::core::v3::SocketAddress,
    >,
}
