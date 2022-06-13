use std::net::SocketAddr;

use mini_redis::{Result, Connection, Frame};

use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "0.0.0.0:6379".parse()?;

    let listener = TcpListener::bind(addr).await?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                process(stream).await;
            },
            Err(e) => eprintln!("error on handle conntction {}", e)
        }
    }
}

async fn process(stream: TcpStream) {
    let mut connection = Connection::new(stream);
    if let Ok(Some(frame)) = connection.read_frame().await {
        println!("GOT: {:?}", frame);
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}