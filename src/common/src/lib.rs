use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::io::Result;

pub fn run_command_with_handler<F>(command: &str, args: &[&str], mut callback_function: F) -> Result<()>
where
    F: FnMut(String),
{
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child
        .stdout
        .take()
        .expect("Failed to capture stdout");

    let reader = BufReader::new(stdout);

    for buffer in reader.lines() {
        if let Ok(buffer) = buffer {
            callback_function(buffer);
        }
    }

    let status = child.wait()?;
    println!("Process exited with: {}", status);
    Ok(())
}



// mpstat 的结构体
#[derive(Debug, Default)]
pub struct PidStatBuffer {
    pub timestamp: String,
    pub command: String,
    pub pid: i32,
    pub cpu: Option<f32>,   // 单位: 百分比
    pub mem: Option<f32>,   // 单位: 百分比
}
