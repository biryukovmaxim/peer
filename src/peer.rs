use tonic::{Request, Response, Status, Streaming};

use crate::peer::peer_server::{Peer};
pub use api::peer_server::*;
pub use api::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast::Sender;
use tokio::sync::{mpsc};
use tokio_stream::wrappers::ReceiverStream;

pub mod api {
    tonic::include_proto!("peer"); // The string specified here must match the proto package name
}
#[derive(Debug, Clone)]
pub struct PeerService {
    peers: Arc<Mutex<HashSet<String>>>,
    pub(crate) address: String,
    sender: Arc<Sender<String>>,
}

impl PeerService {
    pub(crate) fn new(address: String, sender: Arc<Sender<String>>) -> Self {
        PeerService {
            peers: Arc::new(Mutex::new(HashSet::new())),
            address,
            sender,
        }
    }
}

#[tonic::async_trait]
impl Peer for PeerService {
    async fn greeting(
        &self,
        request : Request<PeerInfo>,
    ) -> Result<Response<PeerInfoList>, Status> {
        let remote_address = request.into_inner().address;
        match self.peers.lock() {
            Err(err) => Err(Status::internal(err.to_string())),
            Ok(mut peers) => {
                peers.insert(remote_address.clone());
                Ok(Response::new(PeerInfoList {
                    infos: peers
                        .iter()
                        .filter(|peer| **peer != remote_address)
                        .map(|peer| PeerInfo {
                            address: peer.to_string(),
                        })
                        .collect::<Vec<PeerInfo>>(),
                }))
            }
        }
    }

    type ChatStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn chat(
        &self,
        request: Request<Streaming<ChatMessage>>,
    ) -> Result<Response<Self::ChatStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let mut rx2 = self.sender.subscribe();
        let address = self.address.clone();

        tokio::spawn(async move {
            loop {
                let message_value = rx2.recv().await.unwrap();
                let send_res = tx.send(Ok(ChatMessage {
                    from: address.clone(),
                    content: message_value,
                })).await;
                if let Err(err) = send_res {
                    println!("{}", err);
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

/*#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerInfo {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerInfoList {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<PeerInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatMessage {
    #[prost(string, tag = "1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod peer_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PeerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PeerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PeerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> PeerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            PeerClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn greeting(
            &mut self,
            request: impl tonic::IntoRequest<super::PeerInfo>,
        ) -> Result<tonic::Response<super::PeerInfoList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/peer.Peer/Greeting");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn chat(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ChatMessage>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ChatMessage>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/peer.Peer/Chat");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod peer_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PeerServer."]
    #[async_trait]
    pub trait Peer: Send + Sync + 'static {
        async fn greeting(
            &self,
            request: tonic::Request<super::PeerInfo>,
        ) -> Result<tonic::Response<super::PeerInfoList>, tonic::Status>;
        #[doc = "Server streaming response type for the Chat method."]
        type ChatStream: futures_core::Stream<Item = Result<super::ChatMessage, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn chat(
            &self,
            request: tonic::Request<tonic::Streaming<super::ChatMessage>>,
        ) -> Result<tonic::Response<Self::ChatStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PeerServer<T: Peer> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Peer> PeerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PeerServer<T>
    where
        T: Peer,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/peer.Peer/Greeting" => {
                    #[allow(non_camel_case_types)]
                    struct GreetingSvc<T: Peer>(pub Arc<T>);
                    impl<T: Peer> tonic::server::UnaryService<super::PeerInfo> for GreetingSvc<T> {
                        type Response = super::PeerInfoList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PeerInfo>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).greeting(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GreetingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peer.Peer/Chat" => {
                    #[allow(non_camel_case_types)]
                    struct ChatSvc<T: Peer>(pub Arc<T>);
                    impl<T: Peer> tonic::server::StreamingService<super::ChatMessage> for ChatSvc<T> {
                        type Response = super::ChatMessage;
                        type ResponseStream = T::ChatStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::ChatMessage>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).chat(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Peer> Clone for PeerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Peer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Peer> tonic::transport::NamedService for PeerServer<T> {
        const NAME: &'static str = "peer.Peer";
    }
}
*/