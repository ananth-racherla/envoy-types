#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpProtocolOptions {
    /// The idle timeout for the connection. The idle timeout is defined as the period in which
    /// the connection is not associated with a downstream connection. When the idle timeout is
    /// reached, the connection will be closed.
    ///
    /// If not set, the default idle timeout is 10 minutes. To disable idle timeouts, explicitly set this to 0.
    ///
    /// .. warning::
    /// Disabling this timeout has a highly likelihood of yielding connection leaks due to lost TCP
    /// FIN packets, etc.
    #[prost(message, optional, tag = "1")]
    pub idle_timeout: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
}
