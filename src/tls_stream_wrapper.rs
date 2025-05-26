use async_dup::{Arc, Mutex};
use futures_rustls::server::TlsStream;
use smol::io::{AsyncRead, Result, AsyncWrite};
use smol::net::TcpStream;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Clone)]
pub(crate) struct TlsStreamWrapper(Arc<Mutex<TlsStream<TcpStream>>>);

impl TlsStreamWrapper {
    pub(crate) fn new(stream: TlsStream<TcpStream>) -> Self {
        Self(Arc::new(Mutex::new(stream)))
    }
}

impl AsyncRead for TlsStreamWrapper {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        Pin::new(&mut &*self.0).poll_read(cx, buf)
    }
}

impl AsyncWrite for TlsStreamWrapper {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        Pin::new(&mut &*self.0).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut &*self.0).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut &*self.0).poll_close(cx)
    }
}
