#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// The list of rules to apply to requests.
    #[prost(message, repeated, tag = "1")]
    pub request_rules: ::prost::alloc::vec::Vec<config::Rule>,
    /// The list of rules to apply to responses.
    #[prost(message, repeated, tag = "2")]
    pub response_rules: ::prost::alloc::vec::Vec<config::Rule>,
}
/// Nested message and enum types in `Config`.
pub mod config {
    /// \[\#next-free-field: 7\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyValuePair {
        /// The namespace — if this is empty, the filter's namespace will be used.
        #[prost(string, tag = "1")]
        pub metadata_namespace: ::prost::alloc::string::String,
        /// The key to use within the namespace.
        #[prost(string, tag = "2")]
        pub key: ::prost::alloc::string::String,
        /// The value to pair with the given key.
        ///
        /// When used for a
        /// :ref:`on_header_present <envoy_v3_api_field_extensions.filters.http.header_to_metadata.v3.Config.Rule.on_header_present>`
        /// case, if value is non-empty it'll be used instead of the header value. If both are empty, no metadata is added.
        ///
        /// When used for a :ref:`on_header_missing <envoy_v3_api_field_extensions.filters.http.header_to_metadata.v3.Config.Rule.on_header_missing>`
        /// case, a non-empty value must be provided otherwise no metadata is added.
        #[prost(string, tag = "3")]
        pub value: ::prost::alloc::string::String,
        /// If present, the header's value will be matched and substituted with this. If there is no match or substitution, the header value
        /// is used as-is.
        ///
        /// This is only used for :ref:`on_header_present <envoy_v3_api_field_extensions.filters.http.header_to_metadata.v3.Config.Rule.on_header_present>`.
        ///
        /// Note: if the `value` field is non-empty this field should be empty.
        #[prost(message, optional, tag = "6")]
        pub regex_value_rewrite: ::core::option::Option<
            super::super::super::super::super::super::r#type::matcher::v3::RegexMatchAndSubstitute,
        >,
        /// The value's type — defaults to string.
        #[prost(enumeration = "ValueType", tag = "4")]
        pub r#type: i32,
        /// How is the value encoded, default is NONE (not encoded).
        /// The value will be decoded accordingly before storing to metadata.
        #[prost(enumeration = "ValueEncode", tag = "5")]
        pub encode: i32,
    }
    /// A Rule defines what metadata to apply when a header is present or missing.
    /// \[\#next-free-field: 6\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rule {
        /// Specifies that a match will be performed on the value of a header or a cookie.
        ///
        /// The header to be extracted.
        #[prost(string, tag = "1")]
        pub header: ::prost::alloc::string::String,
        /// The cookie to be extracted.
        #[prost(string, tag = "5")]
        pub cookie: ::prost::alloc::string::String,
        /// If the header or cookie is present, apply this metadata KeyValuePair.
        ///
        /// If the value in the KeyValuePair is non-empty, it'll be used instead
        /// of the header or cookie value.
        #[prost(message, optional, tag = "2")]
        pub on_header_present: ::core::option::Option<KeyValuePair>,
        /// If the header or cookie is not present, apply this metadata KeyValuePair.
        ///
        /// The value in the KeyValuePair must be set, since it'll be used in lieu
        /// of the missing header or cookie value.
        #[prost(message, optional, tag = "3")]
        pub on_header_missing: ::core::option::Option<KeyValuePair>,
        /// Whether or not to remove the header after a rule is applied.
        ///
        /// This prevents headers from leaking.
        /// This field is not supported in case of a cookie.
        #[prost(bool, tag = "4")]
        pub remove: bool,
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
    pub enum ValueType {
        String = 0,
        Number = 1,
        /// The value is a serialized `protobuf.Value <<https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/struct.proto#L62>`\_.>
        ProtobufValue = 2,
    }
    impl ValueType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueType::String => "STRING",
                ValueType::Number => "NUMBER",
                ValueType::ProtobufValue => "PROTOBUF_VALUE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STRING" => Some(Self::String),
                "NUMBER" => Some(Self::Number),
                "PROTOBUF_VALUE" => Some(Self::ProtobufValue),
                _ => None,
            }
        }
    }
    /// ValueEncode defines the encoding algorithm.
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
    pub enum ValueEncode {
        /// The value is not encoded.
        None = 0,
        /// The value is encoded in `Base64 <<https://tools.ietf.org/html/rfc4648#section-4>`\_.>
        /// Note: this is mostly used for STRING and PROTOBUF_VALUE to escape the
        /// non-ASCII characters in the header.
        Base64 = 1,
    }
    impl ValueEncode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueEncode::None => "NONE",
                ValueEncode::Base64 => "BASE64",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "BASE64" => Some(Self::Base64),
                _ => None,
            }
        }
    }
}
