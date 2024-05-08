use std::io;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

const BUF_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";

    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Dredis: listening on {}", addr);
    loop {
        let (socket, raddr) = listener.accept().await?;
        tracing::info!("Dredis: accepted connection from {}", raddr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(socket, raddr).await {
                tracing::warn!("Dredis: connection error: {:?}", e);
            }
        });
    }
}

async fn process_redis_conn(
    mut stream: tokio::net::TcpStream,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    println!("handle_client");
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
            Ok(0) => {
                println!("client disconnected");
                break;
            }
            Ok(n) => {
                println!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf[..n]);
                tracing::info!("Dredis: received: {:?}", line);
                if let Err(e) = stream.write_all(b"+Ok\r\n").await {
                    println!("failed to write to socket; err = {:?}", e);
                    return Err(e.into());
                }
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    continue;
                } else {
                    println!("failed to read from socket; err = {:?}", e);
                    return Err(e.into());
                }
            }
        }
    }
    tracing::warn!("Dredis: {} client disconnected", addr);
    Ok(())
}
