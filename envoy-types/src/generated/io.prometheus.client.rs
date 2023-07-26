#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelPair {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gauge {
    #[prost(double, optional, tag = "1")]
    pub value: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Counter {
    #[prost(double, optional, tag = "1")]
    pub value: ::core::option::Option<f64>,
    #[prost(message, optional, tag = "2")]
    pub exemplar: ::core::option::Option<Exemplar>,
    #[prost(message, optional, tag = "3")]
    pub created_timestamp: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quantile {
    #[prost(double, optional, tag = "1")]
    pub quantile: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "2")]
    pub value: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Summary {
    #[prost(uint64, optional, tag = "1")]
    pub sample_count: ::core::option::Option<u64>,
    #[prost(double, optional, tag = "2")]
    pub sample_sum: ::core::option::Option<f64>,
    #[prost(message, repeated, tag = "3")]
    pub quantile: ::prost::alloc::vec::Vec<Quantile>,
    #[prost(message, optional, tag = "4")]
    pub created_timestamp: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Untyped {
    #[prost(double, optional, tag = "1")]
    pub value: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Histogram {
    #[prost(uint64, optional, tag = "1")]
    pub sample_count: ::core::option::Option<u64>,
    /// Overrides sample_count if > 0.
    #[prost(double, optional, tag = "4")]
    pub sample_count_float: ::core::option::Option<f64>,
    #[prost(double, optional, tag = "2")]
    pub sample_sum: ::core::option::Option<f64>,
    /// Buckets for the conventional histogram.
    ///
    /// Ordered in increasing order of upper_bound, +Inf bucket is optional.
    #[prost(message, repeated, tag = "3")]
    pub bucket: ::prost::alloc::vec::Vec<Bucket>,
    #[prost(message, optional, tag = "15")]
    pub created_timestamp: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    /// schema defines the bucket schema. Currently, valid numbers are -4 \<= n \<= 8.
    /// They are all for base-2 bucket schemas, where 1 is a bucket boundary in each case, and
    /// then each power of two is divided into 2^n logarithmic buckets.
    /// Or in other words, each bucket boundary is the previous boundary times 2^(2^-n).
    /// In the future, more bucket schemas may be added using numbers \< -4 or > 8.
    #[prost(sint32, optional, tag = "5")]
    pub schema: ::core::option::Option<i32>,
    /// Breadth of the zero bucket.
    #[prost(double, optional, tag = "6")]
    pub zero_threshold: ::core::option::Option<f64>,
    /// Count in zero bucket.
    #[prost(uint64, optional, tag = "7")]
    pub zero_count: ::core::option::Option<u64>,
    /// Overrides sb_zero_count if > 0.
    #[prost(double, optional, tag = "8")]
    pub zero_count_float: ::core::option::Option<f64>,
    /// Negative buckets for the native histogram.
    #[prost(message, repeated, tag = "9")]
    pub negative_span: ::prost::alloc::vec::Vec<BucketSpan>,
    /// Use either "negative_delta" or "negative_count", the former for
    /// regular histograms with integer counts, the latter for float
    /// histograms.
    ///
    /// Count delta of each bucket compared to previous one (or to zero for 1st bucket).
    #[prost(sint64, repeated, packed = "false", tag = "10")]
    pub negative_delta: ::prost::alloc::vec::Vec<i64>,
    /// Absolute count of each bucket.
    #[prost(double, repeated, packed = "false", tag = "11")]
    pub negative_count: ::prost::alloc::vec::Vec<f64>,
    /// Positive buckets for the native histogram.
    #[prost(message, repeated, tag = "12")]
    pub positive_span: ::prost::alloc::vec::Vec<BucketSpan>,
    /// Use either "positive_delta" or "positive_count", the former for
    /// regular histograms with integer counts, the latter for float
    /// histograms.
    ///
    /// Count delta of each bucket compared to previous one (or to zero for 1st bucket).
    #[prost(sint64, repeated, packed = "false", tag = "13")]
    pub positive_delta: ::prost::alloc::vec::Vec<i64>,
    /// Absolute count of each bucket.
    #[prost(double, repeated, packed = "false", tag = "14")]
    pub positive_count: ::prost::alloc::vec::Vec<f64>,
}
/// A Bucket of a conventional histogram, each of which is treated as
/// an individual counter-like time series by Prometheus.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bucket {
    /// Cumulative in increasing order.
    #[prost(uint64, optional, tag = "1")]
    pub cumulative_count: ::core::option::Option<u64>,
    /// Overrides cumulative_count if > 0.
    #[prost(double, optional, tag = "4")]
    pub cumulative_count_float: ::core::option::Option<f64>,
    /// Inclusive.
    #[prost(double, optional, tag = "2")]
    pub upper_bound: ::core::option::Option<f64>,
    #[prost(message, optional, tag = "3")]
    pub exemplar: ::core::option::Option<Exemplar>,
}
/// A BucketSpan defines a number of consecutive buckets in a native
/// histogram with their offset. Logically, it would be more
/// straightforward to include the bucket counts in the Span. However,
/// the protobuf representation is more compact in the way the data is
/// structured here (with all the buckets in a single array separate
/// from the Spans).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BucketSpan {
    /// Gap to previous span, or starting point for 1st span (which can be negative).
    #[prost(sint32, optional, tag = "1")]
    pub offset: ::core::option::Option<i32>,
    /// Length of consecutive buckets.
    #[prost(uint32, optional, tag = "2")]
    pub length: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exemplar {
    #[prost(message, repeated, tag = "1")]
    pub label: ::prost::alloc::vec::Vec<LabelPair>,
    #[prost(double, optional, tag = "2")]
    pub value: ::core::option::Option<f64>,
    /// OpenMetrics-style.
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    #[prost(message, repeated, tag = "1")]
    pub label: ::prost::alloc::vec::Vec<LabelPair>,
    #[prost(message, optional, tag = "2")]
    pub gauge: ::core::option::Option<Gauge>,
    #[prost(message, optional, tag = "3")]
    pub counter: ::core::option::Option<Counter>,
    #[prost(message, optional, tag = "4")]
    pub summary: ::core::option::Option<Summary>,
    #[prost(message, optional, tag = "5")]
    pub untyped: ::core::option::Option<Untyped>,
    #[prost(message, optional, tag = "7")]
    pub histogram: ::core::option::Option<Histogram>,
    #[prost(int64, optional, tag = "6")]
    pub timestamp_ms: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricFamily {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub help: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "MetricType", optional, tag = "3")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "4")]
    pub metric: ::prost::alloc::vec::Vec<Metric>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MetricType {
    /// COUNTER must use the Metric field "counter".
    Counter = 0,
    /// GAUGE must use the Metric field "gauge".
    Gauge = 1,
    /// SUMMARY must use the Metric field "summary".
    Summary = 2,
    /// UNTYPED must use the Metric field "untyped".
    Untyped = 3,
    /// HISTOGRAM must use the Metric field "histogram".
    Histogram = 4,
    /// GAUGE_HISTOGRAM must use the Metric field "histogram".
    GaugeHistogram = 5,
}
impl MetricType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MetricType::Counter => "COUNTER",
            MetricType::Gauge => "GAUGE",
            MetricType::Summary => "SUMMARY",
            MetricType::Untyped => "UNTYPED",
            MetricType::Histogram => "HISTOGRAM",
            MetricType::GaugeHistogram => "GAUGE_HISTOGRAM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COUNTER" => Some(Self::Counter),
            "GAUGE" => Some(Self::Gauge),
            "SUMMARY" => Some(Self::Summary),
            "UNTYPED" => Some(Self::Untyped),
            "HISTOGRAM" => Some(Self::Histogram),
            "GAUGE_HISTOGRAM" => Some(Self::GaugeHistogram),
            _ => None,
        }
    }
}
