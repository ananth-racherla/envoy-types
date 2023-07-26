/// \[\#next-free-field: 12\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutlierDetectionEvent {
    /// In case of eject represents type of ejection that took place.
    #[prost(enumeration = "OutlierEjectionType", tag = "1")]
    pub r#type: i32,
    /// Timestamp for event.
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<
        super::super::super::super::google::protobuf::Timestamp,
    >,
    /// The time in seconds since the last action (either an ejection or unejection) took place.
    #[prost(message, optional, tag = "3")]
    pub secs_since_last_action: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt64Value,
    >,
    /// The :ref:`cluster <envoy_v3_api_msg_config.cluster.v3.Cluster>` that owns the ejected host.
    #[prost(string, tag = "4")]
    pub cluster_name: ::prost::alloc::string::String,
    /// The URL of the ejected host. E.g., `tcp://1.2.3.4:80`.
    #[prost(string, tag = "5")]
    pub upstream_url: ::prost::alloc::string::String,
    /// The action that took place.
    #[prost(enumeration = "Action", tag = "6")]
    pub action: i32,
    /// If `action` is `eject`, specifies the number of times the host has been ejected (local to
    /// that Envoy and gets reset if the host gets removed from the upstream cluster for any reason and
    /// then re-added).
    #[prost(uint32, tag = "7")]
    pub num_ejections: u32,
    /// If `action` is `eject`, specifies if the ejection was enforced. `true` means the host was
    /// ejected. `false` means the event was logged but the host was not actually ejected.
    #[prost(bool, tag = "8")]
    pub enforced: bool,
    #[prost(oneof = "outlier_detection_event::Event", tags = "9, 10, 11")]
    pub event: ::core::option::Option<outlier_detection_event::Event>,
}
/// Nested message and enum types in `OutlierDetectionEvent`.
pub mod outlier_detection_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag = "9")]
        EjectSuccessRateEvent(super::OutlierEjectSuccessRate),
        #[prost(message, tag = "10")]
        EjectConsecutiveEvent(super::OutlierEjectConsecutive),
        #[prost(message, tag = "11")]
        EjectFailurePercentageEvent(super::OutlierEjectFailurePercentage),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutlierEjectSuccessRate {
    /// Host’s success rate at the time of the ejection event on a 0-100 range.
    #[prost(uint32, tag = "1")]
    pub host_success_rate: u32,
    /// Average success rate of the hosts in the cluster at the time of the ejection event on a 0-100
    /// range.
    #[prost(uint32, tag = "2")]
    pub cluster_average_success_rate: u32,
    /// Success rate ejection threshold at the time of the ejection event.
    #[prost(uint32, tag = "3")]
    pub cluster_success_rate_ejection_threshold: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutlierEjectConsecutive {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutlierEjectFailurePercentage {
    /// Host's success rate at the time of the ejection event on a 0-100 range.
    #[prost(uint32, tag = "1")]
    pub host_success_rate: u32,
}
/// Type of ejection that took place
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OutlierEjectionType {
    /// In case upstream host returns certain number of consecutive 5xx.
    /// If
    /// :ref:`outlier_detection.split_external_local_origin_errors<envoy_v3_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is `false`, all type of errors are treated as HTTP 5xx errors.
    /// See :ref:`Cluster outlier detection <arch_overview_outlier_detection>` documentation for
    /// details.
    Consecutive5xx = 0,
    /// In case upstream host returns certain number of consecutive gateway errors
    ConsecutiveGatewayFailure = 1,
    /// Runs over aggregated success rate statistics from every host in cluster
    /// and selects hosts for which ratio of successful replies deviates from other hosts
    /// in the cluster.
    /// If
    /// :ref:`outlier_detection.split_external_local_origin_errors<envoy_v3_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is `false`, all errors (externally and locally generated) are used to calculate success rate
    /// statistics. See :ref:`Cluster outlier detection <arch_overview_outlier_detection>`
    /// documentation for details.
    SuccessRate = 2,
    /// Consecutive local origin failures: Connection failures, resets, timeouts, etc
    /// This type of ejection happens only when
    /// :ref:`outlier_detection.split_external_local_origin_errors<envoy_v3_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is set to `true`.
    /// See :ref:`Cluster outlier detection <arch_overview_outlier_detection>` documentation for
    ConsecutiveLocalOriginFailure = 3,
    /// Runs over aggregated success rate statistics for local origin failures
    /// for all hosts in the cluster and selects hosts for which success rate deviates from other
    /// hosts in the cluster. This type of ejection happens only when
    /// :ref:`outlier_detection.split_external_local_origin_errors<envoy_v3_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is set to `true`.
    /// See :ref:`Cluster outlier detection <arch_overview_outlier_detection>` documentation for
    SuccessRateLocalOrigin = 4,
    /// Runs over aggregated success rate statistics from every host in cluster and selects hosts for
    /// which ratio of failed replies is above configured value.
    FailurePercentage = 5,
    /// Runs over aggregated success rate statistics for local origin failures from every host in
    /// cluster and selects hosts for which ratio of failed replies is above configured value.
    FailurePercentageLocalOrigin = 6,
}
impl OutlierEjectionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OutlierEjectionType::Consecutive5xx => "CONSECUTIVE_5XX",
            OutlierEjectionType::ConsecutiveGatewayFailure => {
                "CONSECUTIVE_GATEWAY_FAILURE"
            }
            OutlierEjectionType::SuccessRate => "SUCCESS_RATE",
            OutlierEjectionType::ConsecutiveLocalOriginFailure => {
                "CONSECUTIVE_LOCAL_ORIGIN_FAILURE"
            }
            OutlierEjectionType::SuccessRateLocalOrigin => "SUCCESS_RATE_LOCAL_ORIGIN",
            OutlierEjectionType::FailurePercentage => "FAILURE_PERCENTAGE",
            OutlierEjectionType::FailurePercentageLocalOrigin => {
                "FAILURE_PERCENTAGE_LOCAL_ORIGIN"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONSECUTIVE_5XX" => Some(Self::Consecutive5xx),
            "CONSECUTIVE_GATEWAY_FAILURE" => Some(Self::ConsecutiveGatewayFailure),
            "SUCCESS_RATE" => Some(Self::SuccessRate),
            "CONSECUTIVE_LOCAL_ORIGIN_FAILURE" => {
                Some(Self::ConsecutiveLocalOriginFailure)
            }
            "SUCCESS_RATE_LOCAL_ORIGIN" => Some(Self::SuccessRateLocalOrigin),
            "FAILURE_PERCENTAGE" => Some(Self::FailurePercentage),
            "FAILURE_PERCENTAGE_LOCAL_ORIGIN" => Some(Self::FailurePercentageLocalOrigin),
            _ => None,
        }
    }
}
/// Represents possible action applied to upstream host
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    /// In case host was excluded from service
    Eject = 0,
    /// In case host was brought back into service
    Uneject = 1,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Eject => "EJECT",
            Action::Uneject => "UNEJECT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EJECT" => Some(Self::Eject),
            "UNEJECT" => Some(Self::Uneject),
            _ => None,
        }
    }
}
