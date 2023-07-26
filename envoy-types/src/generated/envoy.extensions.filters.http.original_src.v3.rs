/// The Original Src filter binds upstream connections to the original source address determined
/// for the request. This address could come from something like the Proxy Protocol filter, or it
/// could come from trusted http headers.
/// \[\#extension: envoy.filters.http.original_src\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OriginalSrc {
    /// Sets the SO_MARK option on the upstream connection's socket to the provided value. Used to
    /// ensure that non-local addresses may be routed back through envoy when binding to the original
    /// source address. The option will not be applied if the mark is 0.
    #[prost(uint32, tag = "1")]
    pub mark: u32,
}
