use std::io;
use std::net::Shutdown;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
pub(super) struct UnixDatagram {
    pub(super) tokio_datagram: tokio::net::UnixDatagram,
}

impl runtime_raw::UnixDatagram for UnixDatagram {
    fn local_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!();
    }

    fn peer_addr(&self) -> io::Result<SocketAddr> {
        unimplemented!();
    }

    fn poll_send_to(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
        receiver: &SocketAddr,
    ) -> Poll<io::Result<usize>> {
        unimplemented!();
    }

    fn poll_recv_from(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<(usize, SocketAddr)>> {
        unimplemented!();
    }

    fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        unimplemented!();
    }
}