use std::sync::Arc;

use colored::Colorize;
use iroh::{ endpoint::{ Incoming, RecvStream, SendStream, VarInt }, Endpoint };
use nanoid::nanoid;
use tokio::{
    io::{ AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf },
    net::{ TcpSocket, TcpStream },
    sync::RwLock,
};

pub async fn establish() -> (Arc<RwLock<Endpoint>>, Vec<u8>, Vec<u8>) {
    let alpn = nanoid!(32).as_bytes().to_vec();
    let endpoint = Arc::new(
        RwLock::new(
            Endpoint::builder().alpns(vec![alpn.clone()]).bind().await.unwrap()
        )
    );
    let nodeid = endpoint.read().await.node_id().as_bytes().to_vec();

    tokio::spawn(internal_handler(endpoint.clone()));

    (endpoint, nodeid, alpn)
}

async fn internal_handler(endpoint: Arc<RwLock<Endpoint>>) {
    let endpoint_read = endpoint.read().await;

    loop {
        let incoming = match endpoint_read.accept().await {
            Some(incoming) => incoming,
            None => {
                break;
            }
        };

        tokio::spawn(incoming_handle(incoming));
    }
}

async fn incoming_handle(incoming: Incoming) {
    let connection = match incoming.await {
        Ok(connection) => connection,
        Err(message) => {
            println!("{} {}", ">".red(), message);
            return;
        }
    };

    let client_stream = match connection.accept_bi().await {
        Ok(client_stream) => client_stream,
        Err(message) => {
            println!("{} {}", ">".red(), message);
            return;
        }
    };

    let v4_addr = "0.0.0.0:0".parse().unwrap();
    let server_proxy = TcpSocket::new_v4().unwrap();

    if server_proxy.bind(v4_addr).is_err() {
        println!(
            "{} {}",
            ">".red(),
            "Can't find a suitable port or IP to create a MCCL proxy layer!"
                .red()
                .bold()
        );
    }

    let proxy_stream = match
        server_proxy.connect("0.0.0.0:25565".parse().unwrap()).await
    {
        Ok(proxy_stream) => proxy_stream,
        Err(message) => {
            println!("{} {}", ">".red(), message);
            return;
        }
    };

    let (reader, writer) = tokio::io::split(proxy_stream);

    tokio::join!(
        stream_reader(reader, client_stream.0),
        stream_writer(writer, client_stream.1)
    );
}

async fn stream_reader(
    mut reader: ReadHalf<TcpStream>,
    mut client_writer: SendStream
) {
    let mut buffer = [0_u8; 4096];

    loop {
        let length = match reader.read(&mut buffer).await {
            Ok(length) => {
                if length != 0 {
                    length
                } else {
                    println!(
                        "{} {}",
                        ">".red(),
                        "Server returned a 0 message in size, aborting..."
                    );
                    break;
                }
            }
            Err(message) => {
                println!(
                    "{} {}\n{}",
                    ">".red(),
                    "Something when wrong. Can't bridge between server and proxy, error logs:",
                    message
                );
                break;
            }
        };

        if let Err(message) = client_writer.write_all(&buffer[..length]).await {
            println!(
                "{} {}\n{}",
                ">".red(),
                "Can't stream the packet back to client proxy, error logs:",
                message
            );
            break;
        }
    }

    let _ = client_writer.finish();
}

async fn stream_writer(
    mut writer: WriteHalf<TcpStream>,
    mut client_reader: RecvStream
) {
    let mut buffer = [0_u8; 4096];

    loop {
        let length = match client_reader.read(&mut buffer).await {
            Ok(length) => {
                if let Some(length) = length {
                    if length != 0 {
                        length
                    } else {
                        println!(
                            "{} {}",
                            ">".red(),
                            "Stream returned a 0 message in size, aborting..."
                        );
                        let _ = client_reader.stop(VarInt::from_u32(0));
                        break;
                    }
                } else {
                    println!("{} {}", ">".yellow(), "Stream finished.");
                    break;
                }
            }
            Err(message) => {
                println!(
                    "{} {}\n{}",
                    ">".red(),
                    "Something when wrong. The client node might be disconnected, error logs:",
                    message
                );
                break;
            }
        };

        if let Err(message) = writer.write_all(&buffer[..length]).await {
            println!(
                "{} {}\n{}",
                ">".red(),
                "Can't stream the packets back to server, error logs:",
                message
            );
            break;
        }
    }
}
