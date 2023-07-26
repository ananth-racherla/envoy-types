/// The runtime fraction matchers computes a hash from the input and matches if runtime feature is enabled
/// for the the resulting hash. Every time the input is considered for a match, its hash must fall within
/// the percentage of matches indicated by this field. For a fraction N/D, a number is computed as a hash
/// of the input on a field in the range \[0,D). If the number is less than or equal to the value of the
/// numerator N, the matcher evaluates to true. A runtime_fraction input matcher can be used to gradually
/// roll out matcher changes without requiring full code or configuration deployments.
/// Note that distribution of matching results is only as good as one of the input.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeFraction {
    /// Match the input against the given runtime key. The specified default value is used if key is not
    /// present in the runtime configuration.
    #[prost(message, optional, tag = "1")]
    pub runtime_fraction: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFractionalPercent,
    >,
    /// Optional seed passed through the hash function. This allows using additional information when computing
    /// the hash value: by changing the seed value, a potentially different outcome can be achieved for the same input.
    #[prost(uint64, tag = "2")]
    pub seed: u64,
}
