#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsIamConfig {
    /// The `service namespace <<https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces>`\_>
    /// of the Grpc endpoint.
    ///
    /// Example: appmesh
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// The `region <<https://docs.aws.amazon.com/general/latest/gr/rande.html>`\_> hosting the Grpc
    /// endpoint. If unspecified, the extension will use the value in the `AWS_REGION` environment
    /// variable.
    ///
    /// Example: us-west-2
    #[prost(string, tag = "2")]
    pub region: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileBasedMetadataConfig {
    /// Location or inline data of secret to use for authentication of the Google gRPC connection
    /// this secret will be attached to a header of the gRPC connection
    #[prost(message, optional, tag = "1")]
    pub secret_data: ::core::option::Option<super::super::core::v3::DataSource>,
    /// Metadata header key to use for sending the secret data
    /// if no header key is set, "authorization" header will be used
    #[prost(string, tag = "2")]
    pub header_key: ::prost::alloc::string::String,
    /// Prefix to prepend to the secret in the metadata header
    /// if no prefix is set, the default is to use no prefix
    #[prost(string, tag = "3")]
    pub header_prefix: ::prost::alloc::string::String,
}
