use crate::common;
use spdlog::info;

pub async fn pidstat_data_record(_pids: &Vec<i32>, _interval: &i8) -> std::io::Result<()> {

    // 获取传入的pid 列表并进行组装存储为 String 类型
    let mut args: Vec<String> = Vec::new();
    for pid in _pids {
        if !args.is_empty() {
            args.push(' '.to_string());
        }
        args.push("-p ".to_string());
        args.push(pid.to_string());
    }

    args.push(_interval.to_string());

    // 把args 转换成 vec str 格式
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    println!("{:?}", args_str);
    // 构造需要传入的参数
    let result = common::util::run_cmd_sync("pidstat", &args_str).await?;
    info!("{:?}", result);
    Ok(())
}