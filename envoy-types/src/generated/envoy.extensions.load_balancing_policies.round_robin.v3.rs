/// This configuration allows the built-in ROUND_ROBIN LB policy to be configured via the LB policy
/// extension point. See the :ref:`load balancing architecture overview <arch_overview_load_balancing_types>` for more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoundRobin {
    /// Configuration for slow start mode.
    /// If this configuration is not set, slow start will not be not enabled.
    #[prost(message, optional, tag = "1")]
    pub slow_start_config: ::core::option::Option<
        super::super::common::v3::SlowStartConfig,
    >,
    /// Configuration for local zone aware load balancing or locality weighted load balancing.
    #[prost(message, optional, tag = "2")]
    pub locality_lb_config: ::core::option::Option<
        super::super::common::v3::LocalityLbConfig,
    >,
}
