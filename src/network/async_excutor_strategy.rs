use crate::Result;
use futures::Future;
use std::time::Duration;

#[cfg(feature = "tokio-runtime")]
pub(crate) type TcpStreamReader = tokio::io::ReadHalf<tokio::net::TcpStream>;
#[cfg(feature = "tokio-runtime")]
pub(crate) type TcpStreamWriter = tokio::io::WriteHalf<tokio::net::TcpStream>;

#[cfg(feature = "async-std-runtime")]
pub(crate) type TcpStreamReader =
    tokio_util::compat::Compat<futures::io::ReadHalf<async_std::net::TcpStream>>;
#[cfg(feature = "async-std-runtime")]
pub(crate) type TcpStreamWriter =
    tokio_util::compat::Compat<futures::io::WriteHalf<async_std::net::TcpStream>>;

#[cfg(feature = "tokio-runtime")]
pub(crate) async fn tcp_connect(addr: &str) -> Result<(TcpStreamReader, TcpStreamWriter)> {
    println!("Connecting to {addr}...");
    let stream = tokio::net::TcpStream::connect(addr).await?;
    let (reader, writer) = tokio::io::split(stream);
    println!("Connected to {addr}");

    Ok((reader, writer))
}

#[cfg(feature = "async-std-runtime")]
pub(crate) async fn tcp_connect(addr: &str) -> Result<(TcpStreamReader, TcpStreamWriter)> {
    use futures::AsyncReadExt;
    use tokio_util::compat::{FuturesAsyncReadCompatExt, FuturesAsyncWriteCompatExt};

    let stream = async_std::net::TcpStream::connect(addr).await?;
    let (reader, writer) = stream.split();
    let reader = reader.compat();
    let writer = writer.compat_write();

    Ok((reader, writer))
}

#[cfg(feature = "tokio-runtime")]
pub(crate) fn spawn<F, T>(future: F)
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    tokio::spawn(future);
}

#[cfg(feature = "async-std-runtime")]
pub(crate) fn spawn<F, T>(future: F)
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    async_std::task::spawn(future);
}

#[allow(dead_code)]
#[cfg(feature = "tokio-runtime")]
pub(crate) async fn sleep(duration: Duration) {
    tokio::time::sleep(duration).await;
}

#[allow(dead_code)]
#[cfg(feature = "async-std-runtime")]
pub(crate) async fn sleep(duration: Duration) {
    async_std::task::sleep(duration).await;
}
