/// RBAC filter config.
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rbac {
    /// Specify the RBAC rules to be applied globally.
    /// If absent, no enforcing RBAC policy will be applied.
    /// If present and empty, DENY.
    /// If both rules and matcher are configured, rules will be ignored.
    #[prost(message, optional, tag = "1")]
    pub rules: ::core::option::Option<
        super::super::super::super::super::config::rbac::v3::Rbac,
    >,
    /// The match tree to use when resolving RBAC action for incoming requests. Requests do not
    /// match any matcher will be denied.
    /// If absent, no enforcing RBAC matcher will be applied.
    /// If present and empty, deny all requests.
    #[prost(message, optional, tag = "4")]
    pub matcher: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    /// Shadow rules are not enforced by the filter (i.e., returning a 403)
    /// but will emit stats and logs and can be used for rule testing.
    /// If absent, no shadow RBAC policy will be applied.
    /// If both shadow rules and shadow matcher are configured, shadow rules will be ignored.
    #[prost(message, optional, tag = "2")]
    pub shadow_rules: ::core::option::Option<
        super::super::super::super::super::config::rbac::v3::Rbac,
    >,
    /// The match tree to use for emitting stats and logs which can be used for rule testing for
    /// incoming requests.
    /// If absent, no shadow matcher will be applied.
    #[prost(message, optional, tag = "5")]
    pub shadow_matcher: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    /// If specified, shadow rules will emit stats with the given prefix.
    /// This is useful to distinguish the stat when there are more than 1 RBAC filter configured with
    /// shadow rules.
    #[prost(string, tag = "3")]
    pub shadow_rules_stat_prefix: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RbacPerRoute {
    /// Override the global configuration of the filter with this new config.
    /// If absent, the global RBAC policy will be disabled for this route.
    #[prost(message, optional, tag = "2")]
    pub rbac: ::core::option::Option<Rbac>,
}
