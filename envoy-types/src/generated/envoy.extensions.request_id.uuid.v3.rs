/// Configuration for the default UUID request ID extension which has the following behavior:
///
/// 1. Request ID is propagated using the :ref:`x-request-id <config_http_conn_man_headers_x-request-id>` header.
///
/// 1. Request ID is a universally unique identifier `(UUID4) <<https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_(random>)>`\_.
///
/// 1. Tracing decision (sampled, forced, etc) is set in 14th nibble of the UUID. By default this will
///    overwrite existing UUIDs received in the `x-request-id` header if the trace sampling decision
///    is changed. The 14th nibble of the UUID4 has been chosen because it is fixed to '4' by the
///    standard. Thus, '4' indicates a default UUID and no trace status. This nibble is swapped to:
///
///    a. '9': Sampled.
///    b. 'a': Force traced due to server-side override.
///    c. 'b': Force traced due to client-side request ID joining.
///
///    See the :ref:`x-request-id <config_http_conn_man_headers_x-request-id>` documentation for
///    more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UuidRequestIdConfig {
    /// Whether the implementation alters the UUID to contain the trace sampling decision as per the
    /// `UuidRequestIdConfig` message documentation. This defaults to true. If disabled no
    /// modification to the UUID will be performed. It is important to note that if disabled,
    /// stable sampling of traces, access logs, etc. will no longer work and only random sampling will
    /// be possible.
    #[prost(message, optional, tag = "1")]
    pub pack_trace_reason: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Set whether to use :ref:`x-request-id<config_http_conn_man_headers_x-request-id>` for sampling or not.
    /// This defaults to true. See the :ref:`context propagation <arch_overview_tracing_context_propagation>`
    /// overview for more information.
    #[prost(message, optional, tag = "2")]
    pub use_request_id_for_trace_sampling: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
}
