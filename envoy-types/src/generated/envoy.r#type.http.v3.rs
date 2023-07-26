#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PathTransformation {
    /// A list of operations to apply. Transformations will be performed in the order that they appear.
    #[prost(message, repeated, tag = "1")]
    pub operations: ::prost::alloc::vec::Vec<path_transformation::Operation>,
}
/// Nested message and enum types in `PathTransformation`.
pub mod path_transformation {
    /// A type of operation to alter text.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Operation {
        #[prost(oneof = "operation::OperationSpecifier", tags = "2, 3")]
        pub operation_specifier: ::core::option::Option<operation::OperationSpecifier>,
    }
    /// Nested message and enum types in `Operation`.
    pub mod operation {
        /// Should text be normalized according to RFC 3986? This typically is used for path headers
        /// before any processing of requests by HTTP filters or routing. This applies percent-encoded
        /// normalization and path segment normalization. Fails on characters disallowed in URLs
        /// (e.g. NULLs). See `Normalization and Comparison <<https://tools.ietf.org/html/rfc3986#section-6>`\_> for details of normalization. Note that
        /// this options does not perform `case normalization <<https://tools.ietf.org/html/rfc3986#section-6.2.2.1>`\_>
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct NormalizePathRfc3986 {}
        /// Determines if adjacent slashes are merged into one. A common use case is for a request path
        /// header. Using this option in `:ref: PathNormalizationOptions <envoy_v3_api_msg_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.PathNormalizationOptions>`
        /// will allow incoming requests with path `//dir///file` to match against route with `prefix`
        /// match set to `/dir`. When using for header transformations, note that slash merging is not
        /// part of `HTTP spec <<https://tools.ietf.org/html/rfc3986>`\_> and is provided for convenience.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MergeSlashes {}
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OperationSpecifier {
            /// Enable path normalization per RFC 3986.
            #[prost(message, tag = "2")]
            NormalizePathRfc3986(NormalizePathRfc3986),
            /// Enable merging adjacent slashes.
            #[prost(message, tag = "3")]
            MergeSlashes(MergeSlashes),
        }
    }
}
/// Cookie defines an API for obtaining or generating HTTP cookie.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cookie {
    /// The name that will be used to obtain cookie value from downstream HTTP request or generate
    /// new cookie for downstream.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Duration of cookie. This will be used to set the expiry time of a new cookie when it is
    /// generated. Set this to 0 to use a session cookie.
    #[prost(message, optional, tag = "2")]
    pub ttl: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Path of cookie. This will be used to set the path of a new cookie when it is generated.
    /// If no path is specified here, no path will be set for the cookie.
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
}
