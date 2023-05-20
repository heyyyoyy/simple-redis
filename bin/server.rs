use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            println!("New connection - {}", addr);
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed read from socket: {:?}", e);
                        return;
                    }
                };
                let msg = if let b"PING\n" = &buf[..n] {
                    b"PONG\n"
                } else {
                    &buf[..n]
                };
                if let Err(e) = socket.write_all(msg).await {
                    eprintln!("Failed write to socket: {:?}", e);
                    return;
                }
            }
        });
    }
}