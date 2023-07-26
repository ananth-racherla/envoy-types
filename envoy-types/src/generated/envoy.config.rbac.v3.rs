/// Role Based Access Control (RBAC) provides service-level and method-level access control for a
/// service. Requests are allowed or denied based on the `action` and whether a matching policy is
/// found. For instance, if the action is ALLOW and a matching policy is found the request should be
/// allowed.
///
/// RBAC can also be used to make access logging decisions by communicating with access loggers
/// through dynamic metadata. When the action is LOG and at least one policy matches, the
/// `access_log_hint` value in the shared key namespace 'envoy.common' is set to `true` indicating
/// the request should be logged.
///
/// Here is an example of RBAC configuration. It has two policies:
///
/// * Service account `cluster.local/ns/default/sa/admin` has full access to the service, and so
///   does "cluster.local/ns/default/sa/superuser".
///
/// * Any user can read (`GET`) the service at paths with prefix `/products`, so long as the
///   destination port is either 80 or 443.
///
/// .. code-block:: yaml
///
/// action: ALLOW
/// policies:
/// "service-admin":
/// permissions:
/// - any: true
/// principals:
/// - authenticated:
/// principal_name:
/// exact: "cluster.local/ns/default/sa/admin"
/// - authenticated:
/// principal_name:
/// exact: "cluster.local/ns/default/sa/superuser"
/// "product-viewer":
/// permissions:
/// - and_rules:
/// rules:
/// - header:
/// name: ":method"
/// string_match:
/// exact: "GET"
/// - url_path:
/// path: { prefix: "/products" }
/// - or_rules:
/// rules:
/// - destination_port: 80
/// - destination_port: 443
/// principals:
/// - any: true
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rbac {
    /// The action to take if a policy matches. Every action either allows or denies a request,
    /// and can also carry out action-specific operations.
    ///
    /// Actions:
    ///
    /// * `ALLOW`: Allows the request if and only if there is a policy that matches
    ///   the request.
    /// * `DENY`: Allows the request if and only if there are no policies that
    ///   match the request.
    /// * `LOG`: Allows all requests. If at least one policy matches, the dynamic
    ///   metadata key `access_log_hint` is set to the value `true` under the shared
    ///   key namespace `envoy.common`. If no policies match, it is set to `false`.
    ///   Other actions do not modify this key.
    #[prost(enumeration = "rbac::Action", tag = "1")]
    pub action: i32,
    /// Maps from policy name to policy. A match occurs when at least one policy matches the request.
    /// The policies are evaluated in lexicographic order of the policy name.
    #[prost(map = "string, message", tag = "2")]
    pub policies: ::std::collections::HashMap<::prost::alloc::string::String, Policy>,
    /// Audit logging options that include the condition for audit logging to happen
    /// and audit logger configurations.
    ///
    /// \\[\#not-implemented-hide:\\]
    #[prost(message, optional, tag = "3")]
    pub audit_logging_options: ::core::option::Option<rbac::AuditLoggingOptions>,
}
/// Nested message and enum types in `RBAC`.
pub mod rbac {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AuditLoggingOptions {
        /// Condition for the audit logging to happen.
        /// If this condition is met, all the audit loggers configured here will be invoked.
        ///
        /// \\[\#not-implemented-hide:\\]
        #[prost(enumeration = "audit_logging_options::AuditCondition", tag = "1")]
        pub audit_condition: i32,
        /// Configurations for RBAC-based authorization audit loggers.
        ///
        /// \\[\#not-implemented-hide:\\]
        #[prost(message, repeated, tag = "2")]
        pub logger_configs: ::prost::alloc::vec::Vec<
            audit_logging_options::AuditLoggerConfig,
        >,
    }
    /// Nested message and enum types in `AuditLoggingOptions`.
    pub mod audit_logging_options {
        /// \\[\#not-implemented-hide:\\]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AuditLoggerConfig {
            /// Typed logger configuration.
            ///
            /// \[\#extension-category: envoy.rbac.audit_loggers\]
            #[prost(message, optional, tag = "1")]
            pub audit_logger: ::core::option::Option<
                super::super::super::super::core::v3::TypedExtensionConfig,
            >,
            /// If true, when the logger is not supported, the data plane will not NACK but simply ignore it.
            #[prost(bool, tag = "2")]
            pub is_optional: bool,
        }
        /// Deny and allow here refer to RBAC decisions, not actions.
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
        pub enum AuditCondition {
            /// Never audit.
            None = 0,
            /// Audit when RBAC denies the request.
            OnDeny = 1,
            /// Audit when RBAC allows the request.
            OnAllow = 2,
            /// Audit whether RBAC allows or denies the request.
            OnDenyAndAllow = 3,
        }
        impl AuditCondition {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    AuditCondition::None => "NONE",
                    AuditCondition::OnDeny => "ON_DENY",
                    AuditCondition::OnAllow => "ON_ALLOW",
                    AuditCondition::OnDenyAndAllow => "ON_DENY_AND_ALLOW",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "NONE" => Some(Self::None),
                    "ON_DENY" => Some(Self::OnDeny),
                    "ON_ALLOW" => Some(Self::OnAllow),
                    "ON_DENY_AND_ALLOW" => Some(Self::OnDenyAndAllow),
                    _ => None,
                }
            }
        }
    }
    /// Should we do safe-list or block-list style access control?
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
    pub enum Action {
        /// The policies grant access to principals. The rest are denied. This is safe-list style
        /// access control. This is the default type.
        Allow = 0,
        /// The policies deny access to principals. The rest are allowed. This is block-list style
        /// access control.
        Deny = 1,
        /// The policies set the `access_log_hint` dynamic metadata key based on if requests match.
        /// All requests are allowed.
        Log = 2,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Allow => "ALLOW",
                Action::Deny => "DENY",
                Action::Log => "LOG",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ALLOW" => Some(Self::Allow),
                "DENY" => Some(Self::Deny),
                "LOG" => Some(Self::Log),
                _ => None,
            }
        }
    }
}
/// Policy specifies a role and the principals that are assigned/denied the role.
/// A policy matches if and only if at least one of its permissions match the
/// action taking place AND at least one of its principals match the downstream
/// AND the condition is true if specified.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    /// Required. The set of permissions that define a role. Each permission is
    /// matched with OR semantics. To match all actions for this policy, a single
    /// Permission with the `any` field set to true should be used.
    #[prost(message, repeated, tag = "1")]
    pub permissions: ::prost::alloc::vec::Vec<Permission>,
    /// Required. The set of principals that are assigned/denied the role based on
    /// “action”. Each principal is matched with OR semantics. To match all
    /// downstreams for this policy, a single Principal with the `any` field set to
    /// true should be used.
    #[prost(message, repeated, tag = "2")]
    pub principals: ::prost::alloc::vec::Vec<Principal>,
    /// An optional symbolic expression specifying an access control
    /// :ref:`condition <arch_overview_condition>`. The condition is combined
    /// with the permissions and the principals as a clause with AND semantics.
    /// Only be used when checked_condition is not used.
    #[prost(message, optional, tag = "3")]
    pub condition: ::core::option::Option<
        super::super::super::super::google::api::expr::v1alpha1::Expr,
    >,
    /// \\[\#not-implemented-hide:\\]
    /// An optional symbolic expression that has been successfully type checked.
    /// Only be used when condition is not used.
    #[prost(message, optional, tag = "4")]
    pub checked_condition: ::core::option::Option<
        super::super::super::super::google::api::expr::v1alpha1::CheckedExpr,
    >,
}
/// Permission defines an action (or actions) that a principal can take.
/// \[\#next-free-field: 13\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    #[prost(oneof = "permission::Rule", tags = "1, 2, 3, 4, 10, 5, 6, 11, 7, 8, 9, 12")]
    pub rule: ::core::option::Option<permission::Rule>,
}
/// Nested message and enum types in `Permission`.
pub mod permission {
    /// Used in the `and_rules` and `or_rules` fields in the `rule` oneof. Depending on the context,
    /// each are applied with the associated behavior.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Set {
        #[prost(message, repeated, tag = "1")]
        pub rules: ::prost::alloc::vec::Vec<super::Permission>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rule {
        /// A set of rules that all must match in order to define the action.
        #[prost(message, tag = "1")]
        AndRules(Set),
        /// A set of rules where at least one must match in order to define the action.
        #[prost(message, tag = "2")]
        OrRules(Set),
        /// When any is set, it matches any action.
        #[prost(bool, tag = "3")]
        Any(bool),
        /// A header (or pseudo-header such as :path or :method) on the incoming HTTP request. Only
        /// available for HTTP request.
        /// Note: the pseudo-header :path includes the query and fragment string. Use the `url_path`
        /// field if you want to match the URL path without the query and fragment string.
        #[prost(message, tag = "4")]
        Header(super::super::super::route::v3::HeaderMatcher),
        /// A URL path on the incoming HTTP request. Only available for HTTP.
        #[prost(message, tag = "10")]
        UrlPath(super::super::super::super::r#type::matcher::v3::PathMatcher),
        /// A CIDR block that describes the destination IP.
        #[prost(message, tag = "5")]
        DestinationIp(super::super::super::core::v3::CidrRange),
        /// A port number that describes the destination port connecting to.
        #[prost(uint32, tag = "6")]
        DestinationPort(u32),
        /// A port number range that describes a range of destination ports connecting to.
        #[prost(message, tag = "11")]
        DestinationPortRange(super::super::super::super::r#type::v3::Int32Range),
        /// Metadata that describes additional information about the action.
        #[prost(message, tag = "7")]
        Metadata(super::super::super::super::r#type::matcher::v3::MetadataMatcher),
        /// Negates matching the provided permission. For instance, if the value of
        /// `not_rule` would match, this permission would not match. Conversely, if
        /// the value of `not_rule` would not match, this permission would match.
        #[prost(message, tag = "8")]
        NotRule(::prost::alloc::boxed::Box<super::Permission>),
        /// The request server from the client's connection request. This is
        /// typically TLS SNI.
        ///
        /// .. attention::
        ///
        /// The behavior of this field may be affected by how Envoy is configured
        /// as explained below.
        ///
        /// * If the :ref:`TLS Inspector <config_listener_filters_tls_inspector>`
        ///   filter is not added, and if a `FilterChainMatch` is not defined for
        ///   the :ref:`server name <envoy_v3_api_field_config.listener.v3.FilterChainMatch.server_names>`,
        ///   a TLS connection's requested SNI server name will be treated as if it
        ///   wasn't present.
        ///
        /// * A :ref:`listener filter <arch_overview_listener_filters>` may
        ///   overwrite a connection's requested server name within Envoy.
        ///
        /// Please refer to :ref:`this FAQ entry <faq_how_to_setup_sni>` to learn to
        /// setup SNI.
        #[prost(message, tag = "9")]
        RequestedServerName(
            super::super::super::super::r#type::matcher::v3::StringMatcher,
        ),
        /// Extension for configuring custom matchers for RBAC.
        /// \[\#extension-category: envoy.rbac.matchers\]
        #[prost(message, tag = "12")]
        Matcher(super::super::super::core::v3::TypedExtensionConfig),
    }
}
/// Principal defines an identity or a group of identities for a downstream
/// subject.
/// \[\#next-free-field: 13\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Principal {
    #[prost(
        oneof = "principal::Identifier",
        tags = "1, 2, 3, 4, 5, 10, 11, 6, 9, 7, 12, 8"
    )]
    pub identifier: ::core::option::Option<principal::Identifier>,
}
/// Nested message and enum types in `Principal`.
pub mod principal {
    /// Used in the `and_ids` and `or_ids` fields in the `identifier` oneof.
    /// Depending on the context, each are applied with the associated behavior.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Set {
        #[prost(message, repeated, tag = "1")]
        pub ids: ::prost::alloc::vec::Vec<super::Principal>,
    }
    /// Authentication attributes for a downstream.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Authenticated {
        /// The name of the principal. If set, The URI SAN or DNS SAN in that order
        /// is used from the certificate, otherwise the subject field is used. If
        /// unset, it applies to any user that is authenticated.
        #[prost(message, optional, tag = "2")]
        pub principal_name: ::core::option::Option<
            super::super::super::super::r#type::matcher::v3::StringMatcher,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Identifier {
        /// A set of identifiers that all must match in order to define the
        /// downstream.
        #[prost(message, tag = "1")]
        AndIds(Set),
        /// A set of identifiers at least one must match in order to define the
        /// downstream.
        #[prost(message, tag = "2")]
        OrIds(Set),
        /// When any is set, it matches any downstream.
        #[prost(bool, tag = "3")]
        Any(bool),
        /// Authenticated attributes that identify the downstream.
        #[prost(message, tag = "4")]
        Authenticated(Authenticated),
        /// A CIDR block that describes the downstream IP.
        /// This address will honor proxy protocol, but will not honor XFF.
        ///
        /// This field is deprecated; either use :ref:`remote_ip <envoy_v3_api_field_config.rbac.v3.Principal.remote_ip>` for the same
        /// behavior, or use
        /// :ref:`direct_remote_ip <envoy_v3_api_field_config.rbac.v3.Principal.direct_remote_ip>`.
        #[prost(message, tag = "5")]
        SourceIp(super::super::super::core::v3::CidrRange),
        /// A CIDR block that describes the downstream remote/origin address.
        /// Note: This is always the physical peer even if the
        /// :ref:`remote_ip <envoy_v3_api_field_config.rbac.v3.Principal.remote_ip>` is
        /// inferred from for example the x-forwarder-for header, proxy protocol,
        /// etc.
        #[prost(message, tag = "10")]
        DirectRemoteIp(super::super::super::core::v3::CidrRange),
        /// A CIDR block that describes the downstream remote/origin address.
        /// Note: This may not be the physical peer and could be different from the
        /// :ref:`direct_remote_ip <envoy_v3_api_field_config.rbac.v3.Principal.direct_remote_ip>`. E.g, if the
        /// remote ip is inferred from for example the x-forwarder-for header, proxy
        /// protocol, etc.
        #[prost(message, tag = "11")]
        RemoteIp(super::super::super::core::v3::CidrRange),
        /// A header (or pseudo-header such as :path or :method) on the incoming HTTP
        /// request. Only available for HTTP request. Note: the pseudo-header :path
        /// includes the query and fragment string. Use the `url_path` field if you
        /// want to match the URL path without the query and fragment string.
        #[prost(message, tag = "6")]
        Header(super::super::super::route::v3::HeaderMatcher),
        /// A URL path on the incoming HTTP request. Only available for HTTP.
        #[prost(message, tag = "9")]
        UrlPath(super::super::super::super::r#type::matcher::v3::PathMatcher),
        /// Metadata that describes additional information about the principal.
        #[prost(message, tag = "7")]
        Metadata(super::super::super::super::r#type::matcher::v3::MetadataMatcher),
        /// Identifies the principal using a filter state object.
        #[prost(message, tag = "12")]
        FilterState(super::super::super::super::r#type::matcher::v3::FilterStateMatcher),
        /// Negates matching the provided principal. For instance, if the value of
        /// `not_id` would match, this principal would not match. Conversely, if the
        /// value of `not_id` would not match, this principal would match.
        #[prost(message, tag = "8")]
        NotId(::prost::alloc::boxed::Box<super::Principal>),
    }
}
/// Action defines the result of allowance or denial when a request matches the matcher.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    /// The name indicates the policy name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The action to take if the matcher matches. Every action either allows or denies a request,
    /// and can also carry out action-specific operations.
    ///
    /// Actions:
    ///
    /// * `ALLOW`: If the request gets matched on ALLOW, it is permitted.
    /// * `DENY`: If the request gets matched on DENY, it is not permitted.
    /// * `LOG`: If the request gets matched on LOG, it is permitted. Besides, the
    ///   dynamic metadata key `access_log_hint` under the shared key namespace
    ///   `envoy.common` will be set to the value `true`.
    /// * If the request cannot get matched, it will fallback to `DENY`.
    ///
    /// Log behavior:
    ///
    /// If the RBAC matcher contains at least one LOG action, the dynamic
    /// metadata key `access_log_hint` will be set based on if the request
    /// get matched on the LOG action.
    #[prost(enumeration = "rbac::Action", tag = "2")]
    pub action: i32,
}
