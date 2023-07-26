/// Configuration for internal listener.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalListener {
    /// The internal listener client connection buffer size in KiB.
    /// For example, if `buffer_size_kb` is set to 5, then the actual buffer size is
    /// 5 KiB = 5 * 1024 bytes.
    /// If the `buffer_size_kb` is not specified, the buffer size is set to 1024 KiB.
    #[prost(message, optional, tag = "1")]
    pub buffer_size_kb: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
