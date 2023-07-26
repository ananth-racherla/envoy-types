/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdmissionControl {
    /// If set to false, the admission control filter will operate as a pass-through filter. If the
    /// message is unspecified, the filter will be enabled.
    #[prost(message, optional, tag = "1")]
    pub enabled: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFeatureFlag,
    >,
    /// The sliding time window over which the success rate is calculated. The window is rounded to the
    /// nearest second. Defaults to 30s.
    #[prost(message, optional, tag = "3")]
    pub sampling_window: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Rejection probability is defined by the formula::
    ///
    /// ```text
    /// max(0, (rq_count -  rq_success_count / sr_threshold) / (rq_count + 1)) ^ (1 / aggression)
    /// ```
    ///
    /// The aggression dictates how heavily the admission controller will throttle requests upon SR
    /// dropping at or below the threshold. A value of 1 will result in a linear increase in
    /// rejection probability as SR drops. Any values less than 1.0, will be set to 1.0. If the
    /// message is unspecified, the aggression is 1.0. See `the admission control documentation <<https://www.envoyproxy.io/docs/envoy/latest/configuration/http/http_filters/admission_control_filter.html>`\_>
    /// for a diagram illustrating this.
    #[prost(message, optional, tag = "4")]
    pub aggression: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeDouble,
    >,
    /// Dictates the success rate at which the rejection probability is non-zero. As success rate drops
    /// below this threshold, rejection probability will increase. Any success rate above the threshold
    /// results in a rejection probability of 0. Defaults to 95%.
    #[prost(message, optional, tag = "5")]
    pub sr_threshold: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimePercent,
    >,
    /// If the average RPS of the sampling window is below this threshold, the request
    /// will not be rejected, even if the success rate is lower than sr_threshold.
    /// Defaults to 0.
    #[prost(message, optional, tag = "6")]
    pub rps_threshold: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeUInt32,
    >,
    /// The probability of rejection will never exceed this value, even if the failure rate is rising.
    /// Defaults to 80%.
    #[prost(message, optional, tag = "7")]
    pub max_rejection_probability: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimePercent,
    >,
    /// Defines how a request is considered a success/failure.
    #[prost(oneof = "admission_control::EvaluationCriteria", tags = "2")]
    pub evaluation_criteria: ::core::option::Option<
        admission_control::EvaluationCriteria,
    >,
}
/// Nested message and enum types in `AdmissionControl`.
pub mod admission_control {
    /// Default method of specifying what constitutes a successful request. All status codes that
    /// indicate a successful request must be explicitly specified if not relying on the default
    /// values.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SuccessCriteria {
        /// If HTTP criteria are unspecified, all HTTP status codes below 500 are treated as successful
        /// responses.
        ///
        /// .. note::
        ///
        /// ```text
        /// The default HTTP codes considered successful by the admission controller are done so due
        /// to the unlikelihood that sending fewer requests would change their behavior (for example:
        /// redirects, unauthorized access, or bad requests won't be alleviated by sending less
        /// traffic).
        /// ```
        #[prost(message, optional, tag = "1")]
        pub http_criteria: ::core::option::Option<success_criteria::HttpCriteria>,
        /// GRPC status codes to consider as request successes. If unspecified, defaults to: Ok,
        /// Cancelled, Unknown, InvalidArgument, NotFound, AlreadyExists, Unauthenticated,
        /// FailedPrecondition, OutOfRange, PermissionDenied, and Unimplemented.
        ///
        /// .. note::
        ///
        /// ```text
        /// The default gRPC codes that are considered successful by the admission controller are
        /// chosen because of the unlikelihood that sending fewer requests will change the behavior.
        /// ```
        #[prost(message, optional, tag = "2")]
        pub grpc_criteria: ::core::option::Option<success_criteria::GrpcCriteria>,
    }
    /// Nested message and enum types in `SuccessCriteria`.
    pub mod success_criteria {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct HttpCriteria {
            /// Status code ranges that constitute a successful request. Configurable codes are in the
            /// range \[100, 600).
            #[prost(message, repeated, tag = "1")]
            pub http_success_status: ::prost::alloc::vec::Vec<
                super::super::super::super::super::super::super::r#type::v3::Int32Range,
            >,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GrpcCriteria {
            /// Status codes that constitute a successful request.
            /// Mappings can be found at: <https://github.com/grpc/grpc/blob/master/doc/statuscodes.md.>
            #[prost(uint32, repeated, packed = "false", tag = "1")]
            pub grpc_success_status: ::prost::alloc::vec::Vec<u32>,
        }
    }
    /// Defines how a request is considered a success/failure.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EvaluationCriteria {
        #[prost(message, tag = "2")]
        SuccessCriteria(SuccessCriteria),
    }
}
