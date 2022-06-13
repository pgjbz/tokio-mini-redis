use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("0.0.0.0:6379").await?;
    client.set("foo", "bar".into()).await?;
    let result = client.get("foo").await?.unwrap();
    dbg!(result);
    Ok(())
}
