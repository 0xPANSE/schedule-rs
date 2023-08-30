use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite, ReadBuf};
use std::pin::Pin;
use std::io::{Error as IoError, SeekFrom};
use std::task::{Context, Poll};

pub struct ClusterSnapshot {}

impl AsyncRead for ClusterSnapshot {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        todo!()
    }
}

impl AsyncWrite for ClusterSnapshot {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<anyhow::Result<usize, IoError>> {
        todo!()
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<anyhow::Result<(), IoError>> {
        todo!()
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<anyhow::Result<(), IoError>> {
        todo!()
    }
}

impl AsyncSeek for ClusterSnapshot {
    fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> std::io::Result<()> {
        todo!()
    }

    fn poll_complete(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<u64>> {
        todo!()
    }
}
