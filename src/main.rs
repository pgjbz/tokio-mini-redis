use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use bytes::Bytes;
use mini_redis::{Command, Connection, Frame, Result};

use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "0.0.0.0:6379".parse()?;

    let listener = TcpListener::bind(addr).await?;
    let db: Db = Db::default();
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                /*
                    One of the advantages of using Tokio is that asynchronous
                    code allows you to work on many tasks concurrently
                    source: https://tokio.rs/tokio/tutorial/spawning
                */
                let db = db.clone();
                tokio::spawn(async move { process(stream, db).await });
            }
            Err(e) => eprintln!("error on handle conntction {}", e),
        }
    }
}

async fn process(stream: TcpStream, db: Db) {
    let mut connection = Connection::new(stream);
    while let Ok(Some(frame)) = connection.read_frame().await {
        println!("GOT: {:?}", frame);
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                let mut lock = db.lock().await;
                lock.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_owned())
            }
            Command::Get(cmd) => {
                let lock = db.lock().await;
                lock.get(cmd.key())
                    .map(|val| Frame::Bulk(val.clone()))
                    .or_else(|| Some(Frame::Simple("nil".to_owned())))
                    .unwrap()
            }
            _ => Frame::Error("unimplemented".to_string()),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
