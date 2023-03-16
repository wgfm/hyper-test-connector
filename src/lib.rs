use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::Poll;

use bytes::BufMut;
use hyper::client::connect::Connection;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::Uri;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tower::Service;

struct RecordingConnector {
    connection: FakeConnection,
awaitput

impl Service<Uri> for RecordingConnector {
    type Response = FakeConnection;

    type Error = Box<dyn Error>;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Uri) -> Self::Future {
        todo!()
    }
}

struct FakeConnection {
    read_buf: bytes::Bytes,
    write_buf: Arc<Mutex<bytes::BytesMut>>,
}

impl AsyncRead for FakeConnection {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        buf.put_slice(&self.read_buf);

        Poll::Ready(Ok(()))
    }
}

impl AsyncWrite for FakeConnection {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        let size = buf.len();
        self.write_buf.lock().unwrap().put(buf);

        Poll::Ready(Ok(size))
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl Connection for FakeConnection {
    fn connected(&self) -> hyper::client::connect::Connected {
        hyper::client::connect::Connected::new()
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    fn test_usage() {
        hyper::client::
    }
}
