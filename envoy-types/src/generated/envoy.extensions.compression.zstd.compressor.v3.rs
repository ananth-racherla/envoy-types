/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Zstd {
    /// Set compression parameters according to pre-defined compression level table.
    /// Note that exact compression parameters are dynamically determined,
    /// depending on both compression level and source content size (when known).
    /// Value 0 means default, and default level is 3.
    /// Setting a level does not automatically set all other compression parameters
    /// to default. Setting this will however eventually dynamically impact the compression
    /// parameters which have not been manually set. The manually set
    /// ones will 'stick'.
    #[prost(message, optional, tag = "1")]
    pub compression_level: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// A 32-bits checksum of content is written at end of frame. If not set, defaults to false.
    #[prost(bool, tag = "2")]
    pub enable_checksum: bool,
    /// The higher the value of selected strategy, the more complex it is,
    /// resulting in stronger and slower compression.
    /// Special: value 0 means "use default strategy".
    #[prost(enumeration = "zstd::Strategy", tag = "3")]
    pub strategy: i32,
    /// A dictionary for compression. Zstd offers dictionary compression, which greatly improves
    /// efficiency on small files and messages. Each dictionary will be generated with a dictionary ID
    /// that can be used to search the same dictionary during decompression.
    /// Please refer to `zstd manual <<https://github.com/facebook/zstd/blob/dev/programs/zstd.1.md#dictionary-builder>`\_>
    /// to train a specific dictionary for compression.
    #[prost(message, optional, tag = "4")]
    pub dictionary: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    /// Value for compressor's next output buffer. If not set, defaults to 4096.
    #[prost(message, optional, tag = "5")]
    pub chunk_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
/// Nested message and enum types in `Zstd`.
pub mod zstd {
    /// Reference to <http://facebook.github.io/zstd/zstd_manual.html>
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
    pub enum Strategy {
        Default = 0,
        Fast = 1,
        Dfast = 2,
        Greedy = 3,
        Lazy = 4,
        Lazy2 = 5,
        Btlazy2 = 6,
        Btopt = 7,
        Btultra = 8,
        Btultra2 = 9,
    }
    impl Strategy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Strategy::Default => "DEFAULT",
                Strategy::Fast => "FAST",
                Strategy::Dfast => "DFAST",
                Strategy::Greedy => "GREEDY",
                Strategy::Lazy => "LAZY",
                Strategy::Lazy2 => "LAZY2",
                Strategy::Btlazy2 => "BTLAZY2",
                Strategy::Btopt => "BTOPT",
                Strategy::Btultra => "BTULTRA",
                Strategy::Btultra2 => "BTULTRA2",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "FAST" => Some(Self::Fast),
                "DFAST" => Some(Self::Dfast),
                "GREEDY" => Some(Self::Greedy),
                "LAZY" => Some(Self::Lazy),
                "LAZY2" => Some(Self::Lazy2),
                "BTLAZY2" => Some(Self::Btlazy2),
                "BTOPT" => Some(Self::Btopt),
                "BTULTRA" => Some(Self::Btultra),
                "BTULTRA2" => Some(Self::Btultra2),
                _ => None,
            }
        }
    }
}
