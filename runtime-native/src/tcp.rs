use futures::prelude::*;
use romio::async_ready::{AsyncReadReady, AsyncReady, AsyncWriteReady};

use std::io;
use std::net::SocketAddr;
use std::task::{Poll, Waker};

#[derive(Debug)]
pub(crate) struct TcpStream {
    pub romio_stream: romio::tcp::TcpStream,
}

#[derive(Debug)]
pub(crate) struct TcpListener {
    pub romio_listener: romio::tcp::TcpListener,
}

impl runtime_raw::TcpStream for TcpStream {
    fn poll_write_ready(&self, waker: &Waker) -> Poll<io::Result<()>> {
        self.romio_stream.poll_write_ready(&waker).map_ok(|_| ())
    }

    fn poll_read_ready(&self, waker: &Waker) -> Poll<io::Result<()>> {
        self.romio_stream.poll_read_ready(&waker).map_ok(|_| ())
    }

    fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.romio_stream.local_addr()
    }

    fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.romio_stream.peer_addr()
    }

    fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        self.romio_stream.shutdown(how)
    }

    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.romio_stream.as_raw_fd()
    }
}

impl AsyncRead for TcpStream {
    fn poll_read(&mut self, waker: &Waker, mut buf: &mut [u8]) -> Poll<io::Result<usize>> {
        self.romio_stream.poll_read(&waker, &mut buf)
    }
}

impl AsyncWrite for TcpStream {
    fn poll_write(&mut self, waker: &Waker, buf: &[u8]) -> Poll<io::Result<usize>> {
        self.romio_stream.poll_write(&waker, &buf)
    }

    fn poll_flush(&mut self, waker: &Waker) -> Poll<io::Result<()>> {
        self.romio_stream.poll_flush(&waker)
    }

    fn poll_close(&mut self, waker: &Waker) -> Poll<io::Result<()>> {
        self.romio_stream.poll_close(&waker)
    }
}

impl runtime_raw::TcpListener for TcpListener {
    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.romio_listener.local_addr()
    }

    fn poll_accept(&mut self, waker: &Waker) -> Poll<io::Result<Box<dyn runtime_raw::TcpStream>>> {
        self.romio_listener
            .poll_ready(&waker)
            .map_ok(|(romio_stream, _)| {
                Box::new(TcpStream { romio_stream }) as Box<dyn runtime_raw::TcpStream>
            })
    }

    /// Extracts the raw file descriptor.
    #[cfg(unix)]
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        use std::os::unix::io::AsRawFd;
        self.romio_listener.as_raw_fd()
    }
}