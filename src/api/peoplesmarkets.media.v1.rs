#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaResponse {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub offer_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub market_booth_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub created_at: i64,
    #[prost(int64, tag = "6")]
    pub updated_at: i64,
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaUpload {
    #[prost(string, tag = "1")]
    pub content_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMediaRequest {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub file: ::core::option::Option<MediaUpload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMediaResponse {
    #[prost(message, optional, tag = "1")]
    pub media: ::core::option::Option<MediaResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMediaRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMediaResponse {
    #[prost(message, optional, tag = "1")]
    pub media: ::core::option::Option<MediaResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaOrderBy {
    #[prost(enumeration = "MediaOrderByField", tag = "1")]
    pub field: i32,
    #[prost(enumeration = "super::super::ordering::v1::Direction", tag = "2")]
    pub direction: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaFilter {
    #[prost(enumeration = "MediaFilterField", tag = "1")]
    pub field: i32,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMediaRequest {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
    #[prost(message, optional, tag = "3")]
    pub order_by: ::core::option::Option<MediaOrderBy>,
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<MediaFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMediaResponse {
    #[prost(message, repeated, tag = "1")]
    pub medias: ::prost::alloc::vec::Vec<MediaResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessibleMediaRequest {
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
    #[prost(message, optional, tag = "3")]
    pub order_by: ::core::option::Option<MediaOrderBy>,
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<MediaFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessibleMediaResponse {
    #[prost(message, repeated, tag = "1")]
    pub medias: ::prost::alloc::vec::Vec<MediaResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMediaRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub file: ::core::option::Option<MediaUpload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMediaResponse {
    #[prost(message, optional, tag = "1")]
    pub media: ::core::option::Option<MediaResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMediaRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMediaResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitiateMultipartUploadRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub content_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitiateMultipartUploadResponse {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub upload_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutMultipartChunkRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub upload_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub part_number: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Part {
    #[prost(uint32, tag = "1")]
    pub part_number: u32,
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutMultipartChunkResponse {
    #[prost(message, optional, tag = "1")]
    pub part: ::core::option::Option<Part>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteMultipartUploadRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub upload_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub parts: ::prost::alloc::vec::Vec<Part>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteMultipartUploadResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMediaToOfferRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMediaToOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMediaFromOfferRequest {
    #[prost(string, tag = "1")]
    pub media_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMediaFromOfferResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MediaOrderByField {
    Unspecified = 0,
    CreatedAt = 1,
    UpdatedAt = 2,
}
impl MediaOrderByField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MediaOrderByField::Unspecified => "MEDIA_ORDER_BY_FIELD_UNSPECIFIED",
            MediaOrderByField::CreatedAt => "MEDIA_ORDER_BY_FIELD_CREATED_AT",
            MediaOrderByField::UpdatedAt => "MEDIA_ORDER_BY_FIELD_UPDATED_AT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MEDIA_ORDER_BY_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "MEDIA_ORDER_BY_FIELD_CREATED_AT" => Some(Self::CreatedAt),
            "MEDIA_ORDER_BY_FIELD_UPDATED_AT" => Some(Self::UpdatedAt),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MediaFilterField {
    Unspecified = 0,
    Name = 1,
    OfferId = 2,
}
impl MediaFilterField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MediaFilterField::Unspecified => "MEDIA_FILTER_FIELD_UNSPECIFIED",
            MediaFilterField::Name => "MEDIA_FILTER_FIELD_NAME",
            MediaFilterField::OfferId => "MEDIA_FILTER_FIELD_OFFER_ID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MEDIA_FILTER_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "MEDIA_FILTER_FIELD_NAME" => Some(Self::Name),
            "MEDIA_FILTER_FIELD_OFFER_ID" => Some(Self::OfferId),
            _ => None,
        }
    }
}
/// Generated server implementations.
pub mod media_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MediaServiceServer.
    #[async_trait]
    pub trait MediaService: Send + Sync + 'static {
        async fn create_media(
            &self,
            request: tonic::Request<super::CreateMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMediaResponse>,
            tonic::Status,
        >;
        async fn get_media(
            &self,
            request: tonic::Request<super::GetMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMediaResponse>,
            tonic::Status,
        >;
        async fn list_media(
            &self,
            request: tonic::Request<super::ListMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMediaResponse>,
            tonic::Status,
        >;
        async fn list_accessible_media(
            &self,
            request: tonic::Request<super::ListAccessibleMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAccessibleMediaResponse>,
            tonic::Status,
        >;
        async fn update_media(
            &self,
            request: tonic::Request<super::UpdateMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMediaResponse>,
            tonic::Status,
        >;
        async fn delete_media(
            &self,
            request: tonic::Request<super::DeleteMediaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMediaResponse>,
            tonic::Status,
        >;
        async fn initiate_multipart_upload(
            &self,
            request: tonic::Request<super::InitiateMultipartUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InitiateMultipartUploadResponse>,
            tonic::Status,
        >;
        async fn put_multipart_chunk(
            &self,
            request: tonic::Request<super::PutMultipartChunkRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutMultipartChunkResponse>,
            tonic::Status,
        >;
        async fn complete_multipart_upload(
            &self,
            request: tonic::Request<super::CompleteMultipartUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteMultipartUploadResponse>,
            tonic::Status,
        >;
        async fn add_media_to_offer(
            &self,
            request: tonic::Request<super::AddMediaToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddMediaToOfferResponse>,
            tonic::Status,
        >;
        async fn remove_media_from_offer(
            &self,
            request: tonic::Request<super::RemoveMediaFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveMediaFromOfferResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MediaServiceServer<T: MediaService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MediaService> MediaServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MediaServiceServer<T>
    where
        T: MediaService,
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
                "/peoplesmarkets.media.v1.MediaService/CreateMedia" => {
                    #[allow(non_camel_case_types)]
                    struct CreateMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::CreateMediaRequest>
                    for CreateMediaSvc<T> {
                        type Response = super::CreateMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::create_media(&inner, request).await
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
                        let method = CreateMediaSvc(inner);
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
                "/peoplesmarkets.media.v1.MediaService/GetMedia" => {
                    #[allow(non_camel_case_types)]
                    struct GetMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::GetMediaRequest>
                    for GetMediaSvc<T> {
                        type Response = super::GetMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::get_media(&inner, request).await
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
                        let method = GetMediaSvc(inner);
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
                "/peoplesmarkets.media.v1.MediaService/ListMedia" => {
                    #[allow(non_camel_case_types)]
                    struct ListMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::ListMediaRequest>
                    for ListMediaSvc<T> {
                        type Response = super::ListMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::list_media(&inner, request).await
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
                        let method = ListMediaSvc(inner);
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
                "/peoplesmarkets.media.v1.MediaService/ListAccessibleMedia" => {
                    #[allow(non_camel_case_types)]
                    struct ListAccessibleMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::ListAccessibleMediaRequest>
                    for ListAccessibleMediaSvc<T> {
                        type Response = super::ListAccessibleMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAccessibleMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::list_accessible_media(&inner, request)
                                    .await
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
                        let method = ListAccessibleMediaSvc(inner);
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
                "/peoplesmarkets.media.v1.MediaService/UpdateMedia" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::UpdateMediaRequest>
                    for UpdateMediaSvc<T> {
                        type Response = super::UpdateMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::update_media(&inner, request).await
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
                        let method = UpdateMediaSvc(inner);
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
                "/peoplesmarkets.media.v1.MediaService/DeleteMedia" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteMediaSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::DeleteMediaRequest>
                    for DeleteMediaSvc<T> {
                        type Response = super::DeleteMediaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteMediaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::delete_media(&inner, request).await
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
                        let method = DeleteMediaSvc(inner);
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
                "/peoplesmarkets.media.v1.MediaService/InitiateMultipartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct InitiateMultipartUploadSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::InitiateMultipartUploadRequest>
                    for InitiateMultipartUploadSvc<T> {
                        type Response = super::InitiateMultipartUploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::InitiateMultipartUploadRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::initiate_multipart_upload(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = InitiateMultipartUploadSvc(inner);
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
                "/peoplesmarkets.media.v1.MediaService/PutMultipartChunk" => {
                    #[allow(non_camel_case_types)]
                    struct PutMultipartChunkSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::PutMultipartChunkRequest>
                    for PutMultipartChunkSvc<T> {
                        type Response = super::PutMultipartChunkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutMultipartChunkRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::put_multipart_chunk(&inner, request)
                                    .await
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
                        let method = PutMultipartChunkSvc(inner);
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
                "/peoplesmarkets.media.v1.MediaService/CompleteMultipartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteMultipartUploadSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::CompleteMultipartUploadRequest>
                    for CompleteMultipartUploadSvc<T> {
                        type Response = super::CompleteMultipartUploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CompleteMultipartUploadRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::complete_multipart_upload(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = CompleteMultipartUploadSvc(inner);
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
                "/peoplesmarkets.media.v1.MediaService/AddMediaToOffer" => {
                    #[allow(non_camel_case_types)]
                    struct AddMediaToOfferSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::AddMediaToOfferRequest>
                    for AddMediaToOfferSvc<T> {
                        type Response = super::AddMediaToOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddMediaToOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::add_media_to_offer(&inner, request)
                                    .await
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
                        let method = AddMediaToOfferSvc(inner);
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
                "/peoplesmarkets.media.v1.MediaService/RemoveMediaFromOffer" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveMediaFromOfferSvc<T: MediaService>(pub Arc<T>);
                    impl<
                        T: MediaService,
                    > tonic::server::UnaryService<super::RemoveMediaFromOfferRequest>
                    for RemoveMediaFromOfferSvc<T> {
                        type Response = super::RemoveMediaFromOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveMediaFromOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaService>::remove_media_from_offer(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = RemoveMediaFromOfferSvc(inner);
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
    impl<T: MediaService> Clone for MediaServiceServer<T> {
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
    impl<T: MediaService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MediaService> tonic::server::NamedService for MediaServiceServer<T> {
        const NAME: &'static str = "peoplesmarkets.media.v1.MediaService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutMediaSubscriptionRequest {
    #[prost(string, tag = "1")]
    pub media_subscription_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub buyer_user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub current_period_start: u64,
    #[prost(uint64, tag = "5")]
    pub current_period_end: u64,
    #[prost(string, tag = "6")]
    pub subscription_status: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub payed_at: u64,
    #[prost(uint64, tag = "8")]
    pub payed_until: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutMediaSubscriptionResponse {}
/// Generated server implementations.
pub mod media_subscription_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MediaSubscriptionServiceServer.
    #[async_trait]
    pub trait MediaSubscriptionService: Send + Sync + 'static {
        async fn put_media_subscription(
            &self,
            request: tonic::Request<super::PutMediaSubscriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutMediaSubscriptionResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MediaSubscriptionServiceServer<T: MediaSubscriptionService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MediaSubscriptionService> MediaSubscriptionServiceServer<T> {
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
    for MediaSubscriptionServiceServer<T>
    where
        T: MediaSubscriptionService,
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
                "/peoplesmarkets.media.v1.MediaSubscriptionService/PutMediaSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct PutMediaSubscriptionSvc<T: MediaSubscriptionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: MediaSubscriptionService,
                    > tonic::server::UnaryService<super::PutMediaSubscriptionRequest>
                    for PutMediaSubscriptionSvc<T> {
                        type Response = super::PutMediaSubscriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutMediaSubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MediaSubscriptionService>::put_media_subscription(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = PutMediaSubscriptionSvc(inner);
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
    impl<T: MediaSubscriptionService> Clone for MediaSubscriptionServiceServer<T> {
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
    impl<T: MediaSubscriptionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MediaSubscriptionService> tonic::server::NamedService
    for MediaSubscriptionServiceServer<T> {
        const NAME: &'static str = "peoplesmarkets.media.v1.MediaSubscriptionService";
    }
}
