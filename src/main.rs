mod cli;
mod sub_commands;
mod common;
mod commands;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = cli::Eon::new();
    app.run().await?;
    Ok(())
}
