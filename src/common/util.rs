use spdlog::error;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio;

pub async fn run_cmd_sync(cmd: &str, args: &[&str]) -> std::io::Result<String> {
    let mut child = tokio::process::Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit()) // stderr 直接输出
        .spawn()?;

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();

    let mut output = String::new();

    while let Some(line) = lines.next_line().await? {
        println!("{}", line); // 实时打印
        output.push_str(&line);
        output.push('\n');
    }

    let status = child.wait().await?;
    error!("Command exited with: {}", status);

    Ok(output)
}
