use tokio::process::{Command, Child};
use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use std::process::Stdio;

// mpstat 的结构体
#[derive(Debug)]
pub struct PidStatBuffer {
    pub timestamp: String,
    pub command: String,
    pub pid: i32,
    pub cpu: f32,   // 单位: 百分比
    pub mem: f32,   // 单位: 百分比
}

pub struct CmdOutput {
    lines: Lines<BufReader<tokio::process::ChildStdout>>,
    child: Child,
}

impl CmdOutput {
    pub async fn next_line(&mut self) -> std::io::Result<Option<String>> {
        self.lines.next_line().await
    }

    pub async fn wait(&mut self) -> std::io::Result<()> {
        self.child.wait().await.map(|_| ())
    }
}

pub async fn run_cmd_async(cmd: &str, args: &[&str]) -> std::io::Result<CmdOutput> {
    let mut command = Command::new(cmd);
    command.args(args);
    command.stdout(Stdio::piped());

    let mut child = command.spawn()?;
    let stdout = child.stdout.take().expect("no stdout");

    let reader = BufReader::new(stdout).lines();

    Ok(CmdOutput {
        lines: reader,
        child,
    })
}