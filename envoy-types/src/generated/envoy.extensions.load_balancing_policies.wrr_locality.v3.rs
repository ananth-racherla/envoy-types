/// Configuration for the wrr_locality LB policy. See the :ref:`load balancing architecture overview <arch_overview_load_balancing_types>` for more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WrrLocality {
    /// The child LB policy to create for endpoint-picking within the chosen locality.
    #[prost(message, optional, tag = "1")]
    pub endpoint_picking_policy: ::core::option::Option<
        super::super::super::super::config::cluster::v3::LoadBalancingPolicy,
    >,
}
