use colored::Colorize;
use iroh::{ endpoint::Incoming, Endpoint };
use nanoid::nanoid;
use tokio::net::{ TcpListener, TcpSocket };

pub async fn establish(alpn: Vec<u8>) {
    let endpoint = Endpoint::builder().alpns(vec![alpn]).bind().await.unwrap();

    loop {
        let incoming = match endpoint.accept().await {
            Some(incoming) => incoming,
            None => {
                continue;
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

    tokio::join!()
}

async fn stream_reader() {}
