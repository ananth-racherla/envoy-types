/// Tap configuration.
///
/// \[\#comment:TODO(mattklein123): Rate limiting\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TapConfig {
    /// The match configuration. If the configuration matches the data source being tapped, a tap will
    /// occur, with the result written to the configured output.
    /// Exactly one of :ref:`match <envoy_v3_api_field_config.tap.v3.TapConfig.match>` and
    /// :ref:`match_config <envoy_v3_api_field_config.tap.v3.TapConfig.match_config>` must be set. If both
    /// are set, the :ref:`match <envoy_v3_api_field_config.tap.v3.TapConfig.match>` will be used.
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub match_config: ::core::option::Option<MatchPredicate>,
    /// The match configuration. If the configuration matches the data source being tapped, a tap will
    /// occur, with the result written to the configured output.
    /// Exactly one of :ref:`match <envoy_v3_api_field_config.tap.v3.TapConfig.match>` and
    /// :ref:`match_config <envoy_v3_api_field_config.tap.v3.TapConfig.match_config>` must be set. If both
    /// are set, the :ref:`match <envoy_v3_api_field_config.tap.v3.TapConfig.match>` will be used.
    #[prost(message, optional, tag = "4")]
    pub r#match: ::core::option::Option<
        super::super::common::matcher::v3::MatchPredicate,
    >,
    /// The tap output configuration. If a match configuration matches a data source being tapped,
    /// a tap will occur and the data will be written to the configured output.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<OutputConfig>,
    /// \\[\#not-implemented-hide:\\] Specify if Tap matching is enabled. The % of requests\connections for
    /// which the tap matching is enabled. When not enabled, the request\connection will not be
    /// recorded.
    ///
    /// .. note::
    ///
    /// This field defaults to 100/:ref:`HUNDRED <envoy_v3_api_enum_type.v3.FractionalPercent.DenominatorType>`.
    #[prost(message, optional, tag = "3")]
    pub tap_enabled: ::core::option::Option<
        super::super::core::v3::RuntimeFractionalPercent,
    >,
}
/// Tap match configuration. This is a recursive structure which allows complex nested match
/// configurations to be built using various logical operators.
/// \[\#next-free-field: 11\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchPredicate {
    #[prost(oneof = "match_predicate::Rule", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub rule: ::core::option::Option<match_predicate::Rule>,
}
/// Nested message and enum types in `MatchPredicate`.
pub mod match_predicate {
    /// A set of match configurations used for logical operations.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchSet {
        /// The list of rules that make up the set.
        #[prost(message, repeated, tag = "1")]
        pub rules: ::prost::alloc::vec::Vec<super::MatchPredicate>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rule {
        /// A set that describes a logical OR. If any member of the set matches, the match configuration
        /// matches.
        #[prost(message, tag = "1")]
        OrMatch(MatchSet),
        /// A set that describes a logical AND. If all members of the set match, the match configuration
        /// matches.
        #[prost(message, tag = "2")]
        AndMatch(MatchSet),
        /// A negation match. The match configuration will match if the negated match condition matches.
        #[prost(message, tag = "3")]
        NotMatch(::prost::alloc::boxed::Box<super::MatchPredicate>),
        /// The match configuration will always match.
        #[prost(bool, tag = "4")]
        AnyMatch(bool),
        /// HTTP request headers match configuration.
        #[prost(message, tag = "5")]
        HttpRequestHeadersMatch(super::HttpHeadersMatch),
        /// HTTP request trailers match configuration.
        #[prost(message, tag = "6")]
        HttpRequestTrailersMatch(super::HttpHeadersMatch),
        /// HTTP response headers match configuration.
        #[prost(message, tag = "7")]
        HttpResponseHeadersMatch(super::HttpHeadersMatch),
        /// HTTP response trailers match configuration.
        #[prost(message, tag = "8")]
        HttpResponseTrailersMatch(super::HttpHeadersMatch),
        /// HTTP request generic body match configuration.
        #[prost(message, tag = "9")]
        HttpRequestGenericBodyMatch(super::HttpGenericBodyMatch),
        /// HTTP response generic body match configuration.
        #[prost(message, tag = "10")]
        HttpResponseGenericBodyMatch(super::HttpGenericBodyMatch),
    }
}
/// HTTP headers match configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpHeadersMatch {
    /// HTTP headers to match.
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<super::super::route::v3::HeaderMatcher>,
}
/// HTTP generic body match configuration.
/// List of text strings and hex strings to be located in HTTP body.
/// All specified strings must be found in the HTTP body for positive match.
/// The search may be limited to specified number of bytes from the body start.
///
/// .. attention::
///
/// Searching for patterns in HTTP body is potentially cpu intensive. For each specified pattern, http body is scanned byte by byte to find a match.
/// If multiple patterns are specified, the process is repeated for each pattern. If location of a pattern is known, `bytes_limit` should be specified
/// to scan only part of the http body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpGenericBodyMatch {
    /// Limits search to specified number of bytes - default zero (no limit - match entire captured buffer).
    #[prost(uint32, tag = "1")]
    pub bytes_limit: u32,
    /// List of patterns to match.
    #[prost(message, repeated, tag = "2")]
    pub patterns: ::prost::alloc::vec::Vec<http_generic_body_match::GenericTextMatch>,
}
/// Nested message and enum types in `HttpGenericBodyMatch`.
pub mod http_generic_body_match {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GenericTextMatch {
        #[prost(oneof = "generic_text_match::Rule", tags = "1, 2")]
        pub rule: ::core::option::Option<generic_text_match::Rule>,
    }
    /// Nested message and enum types in `GenericTextMatch`.
    pub mod generic_text_match {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Rule {
            /// Text string to be located in HTTP body.
            #[prost(string, tag = "1")]
            StringMatch(::prost::alloc::string::String),
            /// Sequence of bytes to be located in HTTP body.
            #[prost(bytes, tag = "2")]
            BinaryMatch(::prost::alloc::vec::Vec<u8>),
        }
    }
}
/// Tap output configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// Output sinks for tap data. Currently a single sink is allowed in the list. Once multiple
    /// sink types are supported this constraint will be relaxed.
    #[prost(message, repeated, tag = "1")]
    pub sinks: ::prost::alloc::vec::Vec<OutputSink>,
    /// For buffered tapping, the maximum amount of received body that will be buffered prior to
    /// truncation. If truncation occurs, the :ref:`truncated <envoy_v3_api_field_data.tap.v3.Body.truncated>` field will be set. If not specified, the
    /// default is 1KiB.
    #[prost(message, optional, tag = "2")]
    pub max_buffered_rx_bytes: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// For buffered tapping, the maximum amount of transmitted body that will be buffered prior to
    /// truncation. If truncation occurs, the :ref:`truncated <envoy_v3_api_field_data.tap.v3.Body.truncated>` field will be set. If not specified, the
    /// default is 1KiB.
    #[prost(message, optional, tag = "3")]
    pub max_buffered_tx_bytes: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Indicates whether taps produce a single buffered message per tap, or multiple streamed
    /// messages per tap in the emitted :ref:`TraceWrapper <envoy_v3_api_msg_data.tap.v3.TraceWrapper>` messages. Note that streamed tapping does not
    /// mean that no buffering takes place. Buffering may be required if data is processed before a
    /// match can be determined. See the HTTP tap filter :ref:`streaming <config_http_filters_tap_streaming>` documentation for more information.
    #[prost(bool, tag = "4")]
    pub streaming: bool,
}
/// Tap output sink configuration.
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputSink {
    /// Sink output format.
    #[prost(enumeration = "output_sink::Format", tag = "1")]
    pub format: i32,
    #[prost(oneof = "output_sink::OutputSinkType", tags = "2, 3, 4, 5")]
    pub output_sink_type: ::core::option::Option<output_sink::OutputSinkType>,
}
/// Nested message and enum types in `OutputSink`.
pub mod output_sink {
    /// Output format. All output is in the form of one or more :ref:`TraceWrapper <envoy_v3_api_msg_data.tap.v3.TraceWrapper>` messages. This enumeration indicates
    /// how those messages are written. Note that not all sinks support all output formats. See
    /// individual sink documentation for more information.
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
    pub enum Format {
        /// Each message will be written as JSON. Any :ref:`body <envoy_v3_api_msg_data.tap.v3.Body>`
        /// data will be present in the :ref:`as_bytes <envoy_v3_api_field_data.tap.v3.Body.as_bytes>` field. This means that body data will be
        /// base64 encoded as per the `proto3 JSON mappings <<https://developers.google.com/protocol-buffers/docs/proto3#json>`\_.>
        JsonBodyAsBytes = 0,
        /// Each message will be written as JSON. Any :ref:`body <envoy_v3_api_msg_data.tap.v3.Body>`
        /// data will be present in the :ref:`as_string <envoy_v3_api_field_data.tap.v3.Body.as_string>` field. This means that body data will be
        /// string encoded as per the `proto3 JSON mappings <<https://developers.google.com/protocol-buffers/docs/proto3#json>`\_.> This format type is
        /// useful when it is known that that body is human readable (e.g., JSON over HTTP) and the
        /// user wishes to view it directly without being forced to base64 decode the body.
        JsonBodyAsString = 1,
        /// Binary proto format. Note that binary proto is not self-delimiting. If a sink writes
        /// multiple binary messages without any length information the data stream will not be
        /// useful. However, for certain sinks that are self-delimiting (e.g., one message per file)
        /// this output format makes consumption simpler.
        ProtoBinary = 2,
        /// Messages are written as a sequence tuples, where each tuple is the message length encoded
        /// as a `protobuf 32-bit varint <<https://developers.google.com/protocol-buffers/docs/reference/cpp/google.protobuf.io.coded_stream>`\_>
        /// followed by the binary message. The messages can be read back using the language specific
        /// protobuf coded stream implementation to obtain the message length and the message.
        ProtoBinaryLengthDelimited = 3,
        /// Text proto format.
        ProtoText = 4,
    }
    impl Format {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Format::JsonBodyAsBytes => "JSON_BODY_AS_BYTES",
                Format::JsonBodyAsString => "JSON_BODY_AS_STRING",
                Format::ProtoBinary => "PROTO_BINARY",
                Format::ProtoBinaryLengthDelimited => "PROTO_BINARY_LENGTH_DELIMITED",
                Format::ProtoText => "PROTO_TEXT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "JSON_BODY_AS_BYTES" => Some(Self::JsonBodyAsBytes),
                "JSON_BODY_AS_STRING" => Some(Self::JsonBodyAsString),
                "PROTO_BINARY" => Some(Self::ProtoBinary),
                "PROTO_BINARY_LENGTH_DELIMITED" => Some(Self::ProtoBinaryLengthDelimited),
                "PROTO_TEXT" => Some(Self::ProtoText),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OutputSinkType {
        /// Tap output will be streamed out the :http:post:`/tap` admin endpoint.
        ///
        /// .. attention::
        ///
        /// It is only allowed to specify the streaming admin output sink if the tap is being
        /// configured from the :http:post:`/tap` admin endpoint. Thus, if an extension has
        /// been configured to receive tap configuration from some other source (e.g., static
        /// file, XDS, etc.) configuring the streaming admin output type will fail.
        #[prost(message, tag = "2")]
        StreamingAdmin(super::StreamingAdminSink),
        /// Tap output will be written to a file per tap sink.
        #[prost(message, tag = "3")]
        FilePerTap(super::FilePerTapSink),
        /// \\[\#not-implemented-hide:\\]
        /// GrpcService to stream data to. The format argument must be PROTO_BINARY.
        /// \[\#comment: TODO(samflattery): remove cleanup in uber_per_filter.cc once implemented\]
        #[prost(message, tag = "4")]
        StreamingGrpc(super::StreamingGrpcSink),
        /// Tap output will be buffered in a single block before flushing to the :http:post:`/tap` admin endpoint
        ///
        /// .. attention::
        ///
        /// It is only allowed to specify the buffered admin output sink if the tap is being
        /// configured from the :http:post:`/tap` admin endpoint. Thus, if an extension has
        /// been configured to receive tap configuration from some other source (e.g., static
        /// file, XDS, etc.) configuring the buffered admin output type will fail.
        #[prost(message, tag = "5")]
        BufferedAdmin(super::BufferedAdminSink),
    }
}
/// Streaming admin sink configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingAdminSink {}
/// BufferedAdminSink configures a tap output to collect traces without returning them until
/// one of multiple criteria are satisfied.
/// Similar to StreamingAdminSink, it is only allowed to specify the buffered admin output
/// sink if the tap is being configured from the `/tap` admin endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferedAdminSink {
    /// Stop collecting traces when the specified number are collected.
    /// If other criteria for ending collection are reached first, this value will not be used.
    #[prost(uint64, tag = "1")]
    pub max_traces: u64,
    /// Acts as a fallback to prevent the client from waiting for long periods of time.
    /// After timeout has occurred, a buffer flush will be triggered, returning the traces buffered so far.
    /// This may result in returning fewer traces than were requested, and in the case that no traces are
    /// buffered during this time, no traces will be returned.
    /// Specifying 0 for the timeout value (or not specifying a value at all) indicates an infinite timeout.
    #[prost(message, optional, tag = "2")]
    pub timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
}
/// The file per tap sink outputs a discrete file for every tapped stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilePerTapSink {
    /// Path prefix. The output file will be of the form \<path_prefix>\_<id>.pb, where <id> is an
    /// identifier distinguishing the recorded trace for stream instances (the Envoy
    /// connection ID, HTTP stream ID, etc.).
    #[prost(string, tag = "1")]
    pub path_prefix: ::prost::alloc::string::String,
}
/// \\[\#not-implemented-hide:\\] Streaming gRPC sink configuration sends the taps to an external gRPC
/// server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingGrpcSink {
    /// Opaque identifier, that will be sent back to the streaming grpc server.
    #[prost(string, tag = "1")]
    pub tap_id: ::prost::alloc::string::String,
    /// The gRPC server that hosts the Tap Sink Service.
    #[prost(message, optional, tag = "2")]
    pub grpc_service: ::core::option::Option<super::super::core::v3::GrpcService>,
}
