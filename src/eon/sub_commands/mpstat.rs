use std::fs::{File, OpenOptions};
use std::io::{stdout, Write};
use std::path::Path;
use spdlog::{error, info};
use crate::commands;
use crossterm::{
    ExecutableCommand,
    cursor::{self, MoveTo},
    style::Print,
    terminal::{Clear, ClearType},
};
pub async fn run(sub_cmds: &commands::MpstatSubCmds) {
    match sub_cmds { 
        commands::MpstatSubCmds::Record {interval, output} => {
            mpstat_data_record(interval, output)
        }
    }
}
fn mpstat_data_record(_interval: &i8, output: &str){
    
    let path = Path::new(&output);
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(path).unwrap();
    
    info!("start record mpstat data");
    info!("file name: {}", &output);
    
    // 线写入表头
    file.write_all("Timestamp,Usr,Sys,Idle\n".as_ref()).unwrap();
    
    let interval = _interval.to_string();
    match common::run_command_with_handler("mpstat", &[interval.as_str()], |buffer| mpstat_record_data_callback(buffer, &mut file)) {
        Ok(_) => {},
        Err(e) => {
            error!("run mpstat error: {}", e);
        }
    }

}

fn mpstat_record_data_callback(data: String, file: &mut File) {
    
    // 跳过无效行
    if data.is_empty() || data.contains("CPU") || data.contains("Linux") {
        return;
    }

    let fields: Vec<&str> = data.split_whitespace().collect();

    // 获取每一帧的时间戳
    let timestamp = fields[0].to_string().replace("时", "-").replace("分", "-").replace("秒", "");

    // 获取用户级进程的CPU 使用占比
    let usr_cpu_pre = fields[2].to_string();

    // 获取系统级进程CPU 使用占比
    let sys_cpu_pre = fields[4].to_string();

    // 获取CPU 空闲时间占比
    let cpu_idle = fields[11].to_string();
    
    let buffer_data = format!("{},{},{},{}\n", timestamp, usr_cpu_pre, sys_cpu_pre, cpu_idle);
    
    file.write_all(buffer_data.as_ref()).unwrap();
    
    
    let mut stdout = stdout();
    // 获取当前光标位置
    let (_, current_row) = cursor::position().unwrap();

    // 将光标移动到当前行的起始位置
    stdout.execute(MoveTo(0, current_row)).unwrap();

    // 清空当前行内容
    stdout.execute(Clear(ClearType::CurrentLine)).unwrap();

    // 打印新的数值
    stdout.execute(Print(format!("recording data: {}", timestamp))).unwrap();

}


