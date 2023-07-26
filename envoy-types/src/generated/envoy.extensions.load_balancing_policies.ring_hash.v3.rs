/// This configuration allows the built-in RING_HASH LB policy to be configured via the LB policy
/// extension point. See the :ref:`load balancing architecture overview <arch_overview_load_balancing_types>` for more information.
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RingHash {
    /// The hash function used to hash hosts onto the ketama ring. The value defaults to
    /// :ref:`XX_HASH<envoy_v3_api_enum_value_config.cluster.v3.Cluster.RingHashLbConfig.HashFunction.XX_HASH>`.
    #[prost(enumeration = "ring_hash::HashFunction", tag = "1")]
    pub hash_function: i32,
    /// Minimum hash ring size. The larger the ring is (that is, the more hashes there are for each
    /// provided host) the better the request distribution will reflect the desired weights. Defaults
    /// to 1024 entries, and limited to 8M entries. See also
    /// :ref:`maximum_ring_size<envoy_v3_api_field_config.cluster.v3.Cluster.RingHashLbConfig.maximum_ring_size>`.
    #[prost(message, optional, tag = "2")]
    pub minimum_ring_size: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt64Value,
    >,
    /// Maximum hash ring size. Defaults to 8M entries, and limited to 8M entries, but can be lowered
    /// to further constrain resource use. See also
    /// :ref:`minimum_ring_size<envoy_v3_api_field_config.cluster.v3.Cluster.RingHashLbConfig.minimum_ring_size>`.
    #[prost(message, optional, tag = "3")]
    pub maximum_ring_size: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt64Value,
    >,
    /// If set to `true`, the cluster will use hostname instead of the resolved
    /// address as the key to consistently hash to an upstream host. Only valid for StrictDNS clusters with hostnames which resolve to a single IP address.
    ///
    /// ..note::
    /// This is deprecated and please use :ref:`consistent_hashing_lb_config <envoy_v3_api_field_extensions.load_balancing_policies.ring_hash.v3.RingHash.consistent_hashing_lb_config>` instead.
    #[deprecated]
    #[prost(bool, tag = "4")]
    pub use_hostname_for_hashing: bool,
    /// Configures percentage of average cluster load to bound per upstream host. For example, with a value of 150
    /// no upstream host will get a load more than 1.5 times the average load of all the hosts in the cluster.
    /// If not specified, the load is not bounded for any upstream host. Typical value for this parameter is between 120 and 200.
    /// Minimum is 100.
    ///
    /// This is implemented based on the method described in the paper <https://arxiv.org/abs/1608.01350.> For the specified
    /// `hash_balance_factor`, requests to any upstream host are capped at `hash_balance_factor/100` times the average number of requests
    /// across the cluster. When a request arrives for an upstream host that is currently serving at its max capacity, linear probing
    /// is used to identify an eligible host. Further, the linear probe is implemented using a random jump in hosts ring/table to identify
    /// the eligible host (this technique is as described in the paper <https://arxiv.org/abs/1908.08762> - the random jump avoids the
    /// cascading overflow effect when choosing the next host in the ring/table).
    ///
    /// If weights are specified on the hosts, they are respected.
    ///
    /// This is an O(N) algorithm, unlike other load balancers. Using a lower `hash_balance_factor` results in more hosts
    /// being probed, so use a higher value if you require better performance.
    ///
    /// ..note::
    /// This is deprecated and please use :ref:`consistent_hashing_lb_config <envoy_v3_api_field_extensions.load_balancing_policies.ring_hash.v3.RingHash.consistent_hashing_lb_config>` instead.
    #[deprecated]
    #[prost(message, optional, tag = "5")]
    pub hash_balance_factor: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Common configuration for hashing-based load balancing policies.
    #[prost(message, optional, tag = "6")]
    pub consistent_hashing_lb_config: ::core::option::Option<
        super::super::common::v3::ConsistentHashingLbConfig,
    >,
    /// Enable locality weighted load balancing for ring hash lb explicitly.
    #[prost(message, optional, tag = "7")]
    pub locality_weighted_lb_config: ::core::option::Option<
        super::super::common::v3::locality_lb_config::LocalityWeightedLbConfig,
    >,
}
/// Nested message and enum types in `RingHash`.
pub mod ring_hash {
    /// The hash function used to hash hosts onto the ketama ring.
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
    pub enum HashFunction {
        /// Currently defaults to XX_HASH.
        DefaultHash = 0,
        /// Use `xxHash <<https://github.com/Cyan4973/xxHash>`\_.>
        XxHash = 1,
        /// Use `MurmurHash2 <<https://sites.google.com/site/murmurhash/>`\_,> this is compatible with
        /// std:hash<string> in GNU libstdc++ 3.4.20 or above. This is typically the case when compiled
        /// on Linux and not macOS.
        MurmurHash2 = 2,
    }
    impl HashFunction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HashFunction::DefaultHash => "DEFAULT_HASH",
                HashFunction::XxHash => "XX_HASH",
                HashFunction::MurmurHash2 => "MURMUR_HASH_2",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT_HASH" => Some(Self::DefaultHash),
                "XX_HASH" => Some(Self::XxHash),
                "MURMUR_HASH_2" => Some(Self::MurmurHash2),
                _ => None,
            }
        }
    }
}
