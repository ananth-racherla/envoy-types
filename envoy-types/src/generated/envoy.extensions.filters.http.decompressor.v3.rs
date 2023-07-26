#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Decompressor {
    /// A decompressor library to use for both request and response decompression. Currently only
    /// :ref:`envoy.compression.gzip.compressor<envoy_v3_api_msg_extensions.compression.gzip.decompressor.v3.Gzip>`
    /// is included in Envoy.
    /// \[\#extension-category: envoy.compression.decompressor\]
    #[prost(message, optional, tag = "1")]
    pub decompressor_library: ::core::option::Option<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    /// Configuration for request decompression. Decompression is enabled by default if left empty.
    #[prost(message, optional, tag = "2")]
    pub request_direction_config: ::core::option::Option<
        decompressor::RequestDirectionConfig,
    >,
    /// Configuration for response decompression. Decompression is enabled by default if left empty.
    #[prost(message, optional, tag = "3")]
    pub response_direction_config: ::core::option::Option<
        decompressor::ResponseDirectionConfig,
    >,
}
/// Nested message and enum types in `Decompressor`.
pub mod decompressor {
    /// Common configuration for filter behavior on both the request and response direction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommonDirectionConfig {
        /// Runtime flag that controls whether the filter is enabled for decompression or not. If set to false, the
        /// filter will operate as a pass-through filter. If the message is unspecified, the filter will be enabled.
        #[prost(message, optional, tag = "1")]
        pub enabled: ::core::option::Option<
            super::super::super::super::super::super::config::core::v3::RuntimeFeatureFlag,
        >,
        /// If set to true, will decompress response even if a `no-transform` cache control header is set.
        #[prost(bool, tag = "2")]
        pub ignore_no_transform_header: bool,
    }
    /// Configuration for filter behavior on the request direction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RequestDirectionConfig {
        #[prost(message, optional, tag = "1")]
        pub common_config: ::core::option::Option<CommonDirectionConfig>,
        /// If set to true, and response decompression is enabled, the filter modifies the Accept-Encoding
        /// request header by appending the decompressor_library's encoding. Defaults to true.
        #[prost(message, optional, tag = "2")]
        pub advertise_accept_encoding: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::BoolValue,
        >,
    }
    /// Configuration for filter behavior on the response direction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResponseDirectionConfig {
        #[prost(message, optional, tag = "1")]
        pub common_config: ::core::option::Option<CommonDirectionConfig>,
    }
}
