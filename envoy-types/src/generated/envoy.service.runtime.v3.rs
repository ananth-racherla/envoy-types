/// \\[\#not-implemented-hide:\\] Not configuration. Workaround c++ protobuf issue with importing
/// services: <https://github.com/google/protobuf/issues/4221>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtdsDummy {}
/// RTDS resource type. This describes a layer in the runtime virtual filesystem.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Runtime {
    /// Runtime resource name. This makes the Runtime a self-describing xDS
    /// resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub layer: ::core::option::Option<
        super::super::super::super::google::protobuf::Struct,
    >,
}
/// Generated client implementations.
pub mod runtime_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Discovery service for Runtime resources.
    #[derive(Debug, Clone)]
    pub struct RuntimeDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RuntimeDiscoveryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RuntimeDiscoveryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            RuntimeDiscoveryServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn stream_runtime(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::discovery::v3::DiscoveryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::discovery::v3::DiscoveryResponse,
                >,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.runtime.v3.RuntimeDiscoveryService/StreamRuntime",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.runtime.v3.RuntimeDiscoveryService",
                        "StreamRuntime",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn delta_runtime(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::discovery::v3::DeltaDiscoveryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::discovery::v3::DeltaDiscoveryResponse,
                >,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.runtime.v3.RuntimeDiscoveryService/DeltaRuntime",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.runtime.v3.RuntimeDiscoveryService",
                        "DeltaRuntime",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        pub async fn fetch_runtime(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::discovery::v3::DiscoveryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::discovery::v3::DiscoveryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.runtime.v3.RuntimeDiscoveryService/FetchRuntime",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.runtime.v3.RuntimeDiscoveryService",
                        "FetchRuntime",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod runtime_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RuntimeDiscoveryServiceServer.
    #[async_trait]
    pub trait RuntimeDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the StreamRuntime method.
        type StreamRuntimeStream: futures_core::Stream<
                Item = std::result::Result<
                    super::super::super::discovery::v3::DiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_runtime(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::discovery::v3::DiscoveryRequest>,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::StreamRuntimeStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the DeltaRuntime method.
        type DeltaRuntimeStream: futures_core::Stream<
                Item = std::result::Result<
                    super::super::super::discovery::v3::DeltaDiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn delta_runtime(
            &self,
            request: tonic::Request<
                tonic::Streaming<
                    super::super::super::discovery::v3::DeltaDiscoveryRequest,
                >,
            >,
        ) -> std::result::Result<
            tonic::Response<Self::DeltaRuntimeStream>,
            tonic::Status,
        >;
        async fn fetch_runtime(
            &self,
            request: tonic::Request<super::super::super::discovery::v3::DiscoveryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::discovery::v3::DiscoveryResponse>,
            tonic::Status,
        >;
    }
    /// Discovery service for Runtime resources.
    #[derive(Debug)]
    pub struct RuntimeDiscoveryServiceServer<T: RuntimeDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RuntimeDiscoveryService> RuntimeDiscoveryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for RuntimeDiscoveryServiceServer<T>
    where
        T: RuntimeDiscoveryService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/envoy.service.runtime.v3.RuntimeDiscoveryService/StreamRuntime" => {
                    #[allow(non_camel_case_types)]
                    struct StreamRuntimeSvc<T: RuntimeDiscoveryService>(pub Arc<T>);
                    impl<
                        T: RuntimeDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::discovery::v3::DiscoveryRequest,
                    > for StreamRuntimeSvc<T> {
                        type Response = super::super::super::discovery::v3::DiscoveryResponse;
                        type ResponseStream = T::StreamRuntimeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::discovery::v3::DiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_runtime(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamRuntimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/envoy.service.runtime.v3.RuntimeDiscoveryService/DeltaRuntime" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaRuntimeSvc<T: RuntimeDiscoveryService>(pub Arc<T>);
                    impl<
                        T: RuntimeDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::discovery::v3::DeltaDiscoveryRequest,
                    > for DeltaRuntimeSvc<T> {
                        type Response = super::super::super::discovery::v3::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaRuntimeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::discovery::v3::DeltaDiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delta_runtime(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeltaRuntimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/envoy.service.runtime.v3.RuntimeDiscoveryService/FetchRuntime" => {
                    #[allow(non_camel_case_types)]
                    struct FetchRuntimeSvc<T: RuntimeDiscoveryService>(pub Arc<T>);
                    impl<
                        T: RuntimeDiscoveryService,
                    > tonic::server::UnaryService<
                        super::super::super::discovery::v3::DiscoveryRequest,
                    > for FetchRuntimeSvc<T> {
                        type Response = super::super::super::discovery::v3::DiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::discovery::v3::DiscoveryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).fetch_runtime(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FetchRuntimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: RuntimeDiscoveryService> Clone for RuntimeDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: RuntimeDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RuntimeDiscoveryService> tonic::server::NamedService
    for RuntimeDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.service.runtime.v3.RuntimeDiscoveryService";
    }
}
