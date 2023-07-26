/// gRPC reverse bridge filter configuration
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterConfig {
    /// The content-type to pass to the upstream when the gRPC bridge filter is applied.
    /// The filter will also validate that the upstream responds with the same content type.
    #[prost(string, tag = "1")]
    pub content_type: ::prost::alloc::string::String,
    /// If true, Envoy will assume that the upstream doesn't understand gRPC frames and
    /// strip the gRPC frame from the request, and add it back in to the response. This will
    /// hide the gRPC semantics from the upstream, allowing it to receive and respond with a
    /// simple binary encoded protobuf. In order to calculate the `Content-Length` header value, Envoy
    /// will buffer the upstream response unless :ref:`response_size_header <envoy_v3_api_field_extensions.filters.http.grpc_http1_reverse_bridge.v3.FilterConfig.response_size_header>`
    /// is set, in which case Envoy will use the value of an upstream header to calculate the content
    /// length.
    #[prost(bool, tag = "2")]
    pub withhold_grpc_frames: bool,
    /// When :ref:`withhold_grpc_frames <envoy_v3_api_field_extensions.filters.http.grpc_http1_reverse_bridge.v3.FilterConfig.withhold_grpc_frames>`
    /// is true, this option controls how Envoy calculates the `Content-Length`. When
    /// `response_size_header` is empty, Envoy will buffer the upstream response to calculate its
    /// size. When `response_size_header` is set to a non-empty string, Envoy will stream the response
    /// to the downstream and it will use the value of the response header with this name to set the
    /// `Content-Length` header and gRPC frame size. If the header with this name is repeated, only
    /// the first value will be used.
    ///
    /// Envoy will treat the upstream response as an error if this option is specified and the header
    /// is missing or if the value does not match the actual response body size.
    #[prost(string, tag = "3")]
    pub response_size_header: ::prost::alloc::string::String,
}
/// gRPC reverse bridge filter configuration per virtualhost/route/weighted-cluster level.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterConfigPerRoute {
    /// If true, disables gRPC reverse bridge filter for this particular vhost or route.
    /// If disabled is specified in multiple per-filter-configs, the most specific one will be used.
    #[prost(bool, tag = "1")]
    pub disabled: bool,
}
