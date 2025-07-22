/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;

/// Connection wrapper for both TCP and TLS streams
pub enum Stream {
    /// Plain TCP stream
    Tcp(TcpStream),
    /// TLS encrypted stream
    Tls(TlsStream<TcpStream>),
}

impl Stream {
    /// Read data from the stream
    pub async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Stream::Tcp(stream) => stream.read(buf).await,
            Stream::Tls(stream) => stream.read(buf).await,
        }
    }

    /// Write all data to the stream
    pub async fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        match self {
            Stream::Tcp(stream) => stream.write_all(buf).await,
            Stream::Tls(stream) => stream.write_all(buf).await,
        }
    }

    /// Flush the stream
    pub async fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Stream::Tcp(stream) => stream.flush().await,
            Stream::Tls(stream) => stream.flush().await,
        }
    }

    /// Shutdown the stream
    pub async fn shutdown(&mut self) -> std::io::Result<()> {
        match self {
            Stream::Tcp(stream) => stream.shutdown().await,
            Stream::Tls(stream) => stream.shutdown().await,
        }
    }
}
