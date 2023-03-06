use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    rustcraft::net::listener::Listener::new()
        .bind("0.0.0.0:25565")
        .await?;

    Ok(())
}
