mod cli;
mod sub_commands;
mod common;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = cli::Eon::new();
    app.run().await?;
    Ok(())
}
