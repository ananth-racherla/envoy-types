#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceMonitor {
    /// The name of the resource monitor to instantiate. Must match a registered
    /// resource monitor type.
    /// See the :ref:`extensions listed in typed_config below <extension_category_envoy.resource_monitors>` for the default list of available resource monitor.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Configuration for the resource monitor being instantiated.
    /// \[\#extension-category: envoy.resource_monitors\]
    #[prost(oneof = "resource_monitor::ConfigType", tags = "3")]
    pub config_type: ::core::option::Option<resource_monitor::ConfigType>,
}
/// Nested message and enum types in `ResourceMonitor`.
pub mod resource_monitor {
    /// Configuration for the resource monitor being instantiated.
    /// \[\#extension-category: envoy.resource_monitors\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag = "3")]
        TypedConfig(super::super::super::super::super::google::protobuf::Any),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThresholdTrigger {
    /// If the resource pressure is greater than or equal to this value, the trigger
    /// will enter saturation.
    #[prost(double, tag = "1")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaledTrigger {
    /// If the resource pressure is greater than this value, the trigger will be in the
    /// :ref:`scaling <arch_overview_overload_manager-triggers-state>` state with value
    /// `(pressure - scaling_threshold) / (saturation_threshold - scaling_threshold)`.
    #[prost(double, tag = "1")]
    pub scaling_threshold: f64,
    /// If the resource pressure is greater than this value, the trigger will enter saturation.
    #[prost(double, tag = "2")]
    pub saturation_threshold: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trigger {
    /// The name of the resource this is a trigger for.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "trigger::TriggerOneof", tags = "2, 3")]
    pub trigger_oneof: ::core::option::Option<trigger::TriggerOneof>,
}
/// Nested message and enum types in `Trigger`.
pub mod trigger {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TriggerOneof {
        #[prost(message, tag = "2")]
        Threshold(super::ThresholdTrigger),
        #[prost(message, tag = "3")]
        Scaled(super::ScaledTrigger),
    }
}
/// Typed configuration for the "envoy.overload_actions.reduce_timeouts" action. See
/// :ref:`the docs <config_overload_manager_reducing_timeouts>` for an example of how to configure
/// the action with different timeouts and minimum values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaleTimersOverloadActionConfig {
    /// A set of timer scaling rules to be applied.
    #[prost(message, repeated, tag = "1")]
    pub timer_scale_factors: ::prost::alloc::vec::Vec<
        scale_timers_overload_action_config::ScaleTimer,
    >,
}
/// Nested message and enum types in `ScaleTimersOverloadActionConfig`.
pub mod scale_timers_overload_action_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ScaleTimer {
        /// The type of timer this minimum applies to.
        #[prost(enumeration = "TimerType", tag = "1")]
        pub timer: i32,
        #[prost(oneof = "scale_timer::OverloadAdjust", tags = "2, 3")]
        pub overload_adjust: ::core::option::Option<scale_timer::OverloadAdjust>,
    }
    /// Nested message and enum types in `ScaleTimer`.
    pub mod scale_timer {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OverloadAdjust {
            /// Sets the minimum duration as an absolute value.
            #[prost(message, tag = "2")]
            MinTimeout(
                super::super::super::super::super::super::google::protobuf::Duration,
            ),
            /// Sets the minimum duration as a percentage of the maximum value.
            #[prost(message, tag = "3")]
            MinScale(super::super::super::super::super::r#type::v3::Percent),
        }
    }
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
    pub enum TimerType {
        /// Unsupported value; users must explicitly specify the timer they want scaled.
        Unspecified = 0,
        /// Adjusts the idle timer for downstream HTTP connections that takes effect when there are no active streams.
        /// This affects the value of :ref:`HttpConnectionManager.common_http_protocol_options.idle_timeout <envoy_v3_api_field_config.core.v3.HttpProtocolOptions.idle_timeout>`
        HttpDownstreamConnectionIdle = 1,
        /// Adjusts the idle timer for HTTP streams initiated by downstream clients.
        /// This affects the value of :ref:`RouteAction.idle_timeout <envoy_v3_api_field_config.route.v3.RouteAction.idle_timeout>` and
        /// :ref:`HttpConnectionManager.stream_idle_timeout <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.stream_idle_timeout>`
        HttpDownstreamStreamIdle = 2,
        /// Adjusts the timer for how long downstream clients have to finish transport-level negotiations
        /// before the connection is closed.
        /// This affects the value of
        /// :ref:`FilterChain.transport_socket_connect_timeout <envoy_v3_api_field_config.listener.v3.FilterChain.transport_socket_connect_timeout>`.
        TransportSocketConnect = 3,
    }
    impl TimerType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimerType::Unspecified => "UNSPECIFIED",
                TimerType::HttpDownstreamConnectionIdle => {
                    "HTTP_DOWNSTREAM_CONNECTION_IDLE"
                }
                TimerType::HttpDownstreamStreamIdle => "HTTP_DOWNSTREAM_STREAM_IDLE",
                TimerType::TransportSocketConnect => "TRANSPORT_SOCKET_CONNECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "HTTP_DOWNSTREAM_CONNECTION_IDLE" => {
                    Some(Self::HttpDownstreamConnectionIdle)
                }
                "HTTP_DOWNSTREAM_STREAM_IDLE" => Some(Self::HttpDownstreamStreamIdle),
                "TRANSPORT_SOCKET_CONNECT" => Some(Self::TransportSocketConnect),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverloadAction {
    /// The name of the overload action. This is just a well-known string that listeners can
    /// use for registering callbacks. Custom overload actions should be named using reverse
    /// DNS to ensure uniqueness.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A set of triggers for this action. The state of the action is the maximum
    /// state of all triggers, which can be scalar values between 0 and 1 or
    /// saturated. Listeners are notified when the overload action changes state.
    /// An overload manager action can only have one trigger for a given resource
    /// e.g. :ref:`Trigger.name <envoy_v3_api_field_config.overload.v3.Trigger.name>` must be unique
    /// in this list.
    #[prost(message, repeated, tag = "2")]
    pub triggers: ::prost::alloc::vec::Vec<Trigger>,
    /// Configuration for the action being instantiated.
    #[prost(message, optional, tag = "3")]
    pub typed_config: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
}
/// A point within the connection or request lifecycle that provides context on
/// whether to shed load at that given stage for the current entity at the
/// point.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadShedPoint {
    /// This is just a well-known string for the LoadShedPoint.
    /// Deployment specific LoadShedPoints e.g. within a custom extension should
    /// be prefixed by the company / deployment name to avoid colliding with any
    /// open source LoadShedPoints.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A set of triggers for this LoadShedPoint. The LoadShedPoint will use the
    /// the maximum state of all triggers, which can be scalar values between 0 and
    /// 1 or saturated. A LoadShedPoint can only have one trigger for a given
    /// resource e.g. :ref:`Trigger.name <envoy_v3_api_field_config.overload.v3.Trigger.name>` must be unique in
    /// this list.
    #[prost(message, repeated, tag = "2")]
    pub triggers: ::prost::alloc::vec::Vec<Trigger>,
}
/// Configuration for which accounts the WatermarkBuffer Factories should
/// track.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferFactoryConfig {
    /// The minimum power of two at which Envoy starts tracking an account.
    ///
    /// Envoy has 8 power of two buckets starting with the provided exponent below.
    /// Concretely the 1st bucket contains accounts for streams that use
    /// \[2^minimum_account_to_track_power_of_two,
    /// 2^(minimum_account_to_track_power_of_two + 1)) bytes.
    /// With the 8th bucket tracking accounts
    /// \>= 128 * 2^minimum_account_to_track_power_of_two.
    ///
    /// The maximum value is 56, since we're using uint64_t for bytes counting,
    /// and that's the last value that would use the 8 buckets. In practice,
    /// we don't expect the proxy to be holding 2^56 bytes.
    ///
    /// If omitted, Envoy should not do any tracking.
    #[prost(uint32, tag = "1")]
    pub minimum_account_to_track_power_of_two: u32,
}
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverloadManager {
    /// The interval for refreshing resource usage.
    #[prost(message, optional, tag = "1")]
    pub refresh_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// The set of resources to monitor.
    #[prost(message, repeated, tag = "2")]
    pub resource_monitors: ::prost::alloc::vec::Vec<ResourceMonitor>,
    /// The set of overload actions.
    #[prost(message, repeated, tag = "3")]
    pub actions: ::prost::alloc::vec::Vec<OverloadAction>,
    /// The set of load shed points.
    #[prost(message, repeated, tag = "5")]
    pub loadshed_points: ::prost::alloc::vec::Vec<LoadShedPoint>,
    /// Configuration for buffer factory.
    #[prost(message, optional, tag = "4")]
    pub buffer_factory_config: ::core::option::Option<BufferFactoryConfig>,
}
