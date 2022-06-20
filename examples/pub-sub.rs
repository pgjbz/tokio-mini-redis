use tokio_stream::StreamExt;

use mini_redis::client;

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("0.0.0.0:6379").await?;
    client.publish("numbers", "zero".into()).await?;
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "six".into()).await?;
    client.publish("numbers", "7".into()).await?;
    client.publish("numbers", "8".into()).await?;
    client.publish("numbers", "nine".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("0.0.0.0:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    let messages = subscriber
        .into_stream()
        .filter(|msg| matches!(msg, Ok(msg) if msg.content.len() == 1))
        .take(3);
    tokio::pin!(messages);
    while let Some(msg) = messages.next().await {
        println!("GOT = {:?}", msg);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    tokio::spawn(async {
        let _ = publish().await;
    });
    subscribe().await?;
    println!("DONE");
    Ok(())
}
