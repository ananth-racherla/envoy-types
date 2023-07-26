/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gzip {
    /// Value from 1 to 9 that controls the amount of internal memory used by zlib. Higher values
    /// use more memory, but are faster and produce better compression results. The default value is 5.
    #[prost(message, optional, tag = "1")]
    pub memory_level: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// A value used for selecting the zlib compression level. This setting will affect speed and
    /// amount of compression applied to the content. "BEST_COMPRESSION" provides higher compression
    /// at the cost of higher latency and is equal to "COMPRESSION_LEVEL_9". "BEST_SPEED" provides
    /// lower compression with minimum impact on response time, the same as "COMPRESSION_LEVEL_1".
    /// "DEFAULT_COMPRESSION" provides an optimal result between speed and compression. According
    /// to zlib's manual this level gives the same result as "COMPRESSION_LEVEL_6".
    /// This field will be set to "DEFAULT_COMPRESSION" if not specified.
    #[prost(enumeration = "gzip::CompressionLevel", tag = "2")]
    pub compression_level: i32,
    /// A value used for selecting the zlib compression strategy which is directly related to the
    /// characteristics of the content. Most of the time "DEFAULT_STRATEGY" will be the best choice,
    /// which is also the default value for the parameter, though there are situations when
    /// changing this parameter might produce better results. For example, run-length encoding (RLE)
    /// is typically used when the content is known for having sequences which same data occurs many
    /// consecutive times. For more information about each strategy, please refer to zlib manual.
    #[prost(enumeration = "gzip::CompressionStrategy", tag = "3")]
    pub compression_strategy: i32,
    /// Value from 9 to 15 that represents the base two logarithmic of the compressor's window size.
    /// Larger window results in better compression at the expense of memory usage. The default is 12
    /// which will produce a 4096 bytes window. For more details about this parameter, please refer to
    /// zlib manual > deflateInit2.
    #[prost(message, optional, tag = "4")]
    pub window_bits: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Value for Zlib's next output buffer. If not set, defaults to 4096.
    /// See <https://www.zlib.net/manual.html> for more details. Also see
    /// <https://github.com/envoyproxy/envoy/issues/8448> for context on this filter's performance.
    #[prost(message, optional, tag = "5")]
    pub chunk_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
/// Nested message and enum types in `Gzip`.
pub mod gzip {
    /// All the values of this enumeration translate directly to zlib's compression strategies.
    /// For more information about each strategy, please refer to zlib manual.
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
    pub enum CompressionStrategy {
        DefaultStrategy = 0,
        Filtered = 1,
        HuffmanOnly = 2,
        Rle = 3,
        Fixed = 4,
    }
    impl CompressionStrategy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CompressionStrategy::DefaultStrategy => "DEFAULT_STRATEGY",
                CompressionStrategy::Filtered => "FILTERED",
                CompressionStrategy::HuffmanOnly => "HUFFMAN_ONLY",
                CompressionStrategy::Rle => "RLE",
                CompressionStrategy::Fixed => "FIXED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT_STRATEGY" => Some(Self::DefaultStrategy),
                "FILTERED" => Some(Self::Filtered),
                "HUFFMAN_ONLY" => Some(Self::HuffmanOnly),
                "RLE" => Some(Self::Rle),
                "FIXED" => Some(Self::Fixed),
                _ => None,
            }
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
    pub enum CompressionLevel {
        DefaultCompression = 0,
        BestSpeed = 1,
        CompressionLevel2 = 2,
        CompressionLevel3 = 3,
        CompressionLevel4 = 4,
        CompressionLevel5 = 5,
        CompressionLevel6 = 6,
        CompressionLevel7 = 7,
        CompressionLevel8 = 8,
        CompressionLevel9 = 9,
    }
    impl CompressionLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CompressionLevel::DefaultCompression => "DEFAULT_COMPRESSION",
                CompressionLevel::BestSpeed => "BEST_SPEED",
                CompressionLevel::CompressionLevel2 => "COMPRESSION_LEVEL_2",
                CompressionLevel::CompressionLevel3 => "COMPRESSION_LEVEL_3",
                CompressionLevel::CompressionLevel4 => "COMPRESSION_LEVEL_4",
                CompressionLevel::CompressionLevel5 => "COMPRESSION_LEVEL_5",
                CompressionLevel::CompressionLevel6 => "COMPRESSION_LEVEL_6",
                CompressionLevel::CompressionLevel7 => "COMPRESSION_LEVEL_7",
                CompressionLevel::CompressionLevel8 => "COMPRESSION_LEVEL_8",
                CompressionLevel::CompressionLevel9 => "COMPRESSION_LEVEL_9",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT_COMPRESSION" => Some(Self::DefaultCompression),
                "BEST_SPEED" => Some(Self::BestSpeed),
                "COMPRESSION_LEVEL_2" => Some(Self::CompressionLevel2),
                "COMPRESSION_LEVEL_3" => Some(Self::CompressionLevel3),
                "COMPRESSION_LEVEL_4" => Some(Self::CompressionLevel4),
                "COMPRESSION_LEVEL_5" => Some(Self::CompressionLevel5),
                "COMPRESSION_LEVEL_6" => Some(Self::CompressionLevel6),
                "COMPRESSION_LEVEL_7" => Some(Self::CompressionLevel7),
                "COMPRESSION_LEVEL_8" => Some(Self::CompressionLevel8),
                "COMPRESSION_LEVEL_9" => Some(Self::CompressionLevel9),
                _ => None,
            }
        }
    }
}
