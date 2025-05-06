mod cli;
mod commands;
pub mod sub_commands;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = cli::Eon::new();
    app.run().await?;
    Ok(())
}
