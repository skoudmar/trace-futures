use std::{error::Error, future::Future, net::Ipv4Addr};

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use trace_futures::print_traced;

async fn client(stream_future: impl Future<Output = io::Result<TcpStream>>) -> io::Result<()> {
    let mut stream = stream_future.await?;

    const NUMBER: i16 = 42;
    print_traced(stream.write_i16(NUMBER), "Client sending i16").await?;
    let received = print_traced(stream.read_i64(), "Client receiving i64").await?;

    assert_eq!(NUMBER as i64, received);
    Ok(())
}

async fn server(mut stream: TcpStream) -> io::Result<()> {
    let number = print_traced(stream.read_i16(), "Server receiving i16").await?;
    print_traced(stream.write_i64(number as i64), "Server sending i64").await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_socket =
        print_traced(TcpListener::bind((Ipv4Addr::LOCALHOST, 0)), "Server bind").await?;

    let server_address = server_socket.local_addr()?;

    let client_stream_future = print_traced(TcpStream::connect(server_address), "Client connect");
    let client_handle = tokio::spawn(print_traced(
        client(client_stream_future),
        "Client spawned future",
    ));

    let server_listen_future = print_traced(server_socket.accept(), "Server Accept").await?;
    let server_handle = tokio::spawn(print_traced(
        server(server_listen_future.0),
        "Server spawned future",
    ));

    client_handle.await??;
    server_handle.await??;

    Ok(())
}
