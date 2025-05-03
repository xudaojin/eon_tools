use duct::cmd;
use std::io::{self, BufRead, BufReader, Read};

pub async fn mpstat_data_record(_interval: &i8) -> std::io::Result<()> {
    // 使用 duct 库执行命令，获取标准输出和标准错误
    let result = cmd!("mpstat", "1").stderr_to_stdout().reader()?;
    let lines = BufReader::new(result).lines();
    
    Ok(())
}
