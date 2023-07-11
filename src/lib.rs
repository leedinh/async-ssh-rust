extern crate futures;
extern crate thrussh;
extern crate tokio_io;

use tokio_io::{AsyncRead, AsyncWrite};
pub struct Session<S: AsyncRead + AsyncWrite> {
    connection: thrussh::client::Connection<S, Self>,
}

pub struct  SessionFuture<S> {}


impl<S> {
    
}

impl<S: AsyncRead + AsyncWrite> Session<S> {
    pub fn new(stream: S) -> Self {
        Session {
            connection: thrussh::client::Connection::new(stream, Self::default()),
        }
    }
}

pub struct OpenedChannel {}

pub struct Channel {}
