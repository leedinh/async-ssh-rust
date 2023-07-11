extern crate async_ssh_ld;
extern crate futures;
extern crate mio;
extern crate tokio_core;

use async_ssh_ld::Session;
use futures::Future;
use mio::TcpStream;
use std::io::prelude::*;

fn main() {
    // asynchronous version
    let ls_out = TcpStream::connect("127.0.0.1:22")
        .and_then(Session::new)
        .and_then(|session| session.authenticate_key("dinh.le", key))
        .and_then(|session| {
            session
                .channel_open()
                .and_then(|channel| channel.exec("ls"))
                .and_then(|channel| tokio_io::io::read_to_end(channel, Vec::new()))
                .map(|(channel, data)| channel.exit_status().map(move |status| (data, status)))
        });

    let core = tokio_core::reactor::Core::new().unwrap();
    let (status, data) = core.run(ls_out).unwrap();
    println!("{}", String::from_utf8_lossy(&data));
    print!("{}", status);

    // synchronous version
    let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_agent("username").unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
}
