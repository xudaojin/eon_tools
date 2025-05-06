use crate::commands;
use common;
use spdlog::{error, info};
use std::fs::{metadata, OpenOptions};
use std::io::BufWriter;
use std::path::Path;
use std::{collections::HashMap, io::Write};


// pidstat 命令主入口
pub async  fn run(sub_cmds: &commands::PidstatSubCmds) {
    match sub_cmds {
        commands::PidstatSubCmds::Record{pids, interval} => {
            pidstat_data_record(pids, interval);
        }
        commands::PidstatSubCmds::Show {pids, interval} => {
            pidstat_data_show(pids, interval);
        }
    }
}


/// 采集指定进程的系统资源占用情况
pub fn pidstat_data_record(_pids: &Vec<i32>, _interval: &i8){

    let mut args: Vec<String> = vec!["-ur".to_string()];

    for pid in _pids {
        args.push("-p".to_string());
        args.push(pid.to_string());
    }
    args.push(_interval.to_string());
    let args_ref: Vec<&str> = args.iter().map(AsRef::as_ref).collect();

    let mut process_map = HashMap::new();
    info!("Start record pidstat data");
    match common::run_command_with_handler("pidstat", &args_ref, |buffer| {data_buffer_callback(buffer, &mut process_map);}) {
        Ok(_) => {},
        Err(e) => {
            error!("run pidstat error: {}", e)
        }
    }
}

/// 显示指定进程的系统资源占用情况
pub fn pidstat_data_show(_pids: &Vec<i32>, _interval: &i8){
    let mut args: Vec<String> = vec!["-ur".to_string()];

    for pid in _pids {
        args.push("-p".to_string());
        args.push(pid.to_string());
    }
    args.push(_interval.to_string());
    let args_ref: Vec<&str> = args.iter().map(AsRef::as_ref).collect();

    info!("待开发-参数:{:?}", args_ref);
}


fn data_buffer_callback(buffer: String, process_map: &mut HashMap<i32, common::PidStatBuffer>) {
    // 跳过无效行
    if buffer.is_empty() || buffer.contains("PID") || buffer.contains("Linux") {
        return;
    }

    let parts: Vec<&str> = buffer.split_whitespace().collect();

    // 跳过长度小于5的数据
    if parts.len() < 5 {
        return;
    }

    // 获取时间戳
    let timestamp = parts[0].to_string();

    // 统一处理PID, 不管是 cpu 行还是 mem行
    let pid = match parts.get(2).and_then(|v| v.parse::<i32>().ok()) {
        Some(pid) => pid,
        None => return,
    };

    let entry = process_map.entry(pid).or_default();
    entry.timestamp = timestamp;
    entry.pid = pid;

    // 判断是 CPU 行还是 MEM 行
    // 判断是 CPU 行还是 MEM 行
    if parts.len() >= 10 && parts[3].parse::<f32>().is_ok() {
        // CPU 行
        entry.cpu = parts.get(7).and_then(|v| v.parse::<f32>().ok());
        entry.command = parts.get(9).unwrap_or(&"").to_string();
    } else if parts.len() >= 9 && parts[3].contains('.') {
        // MEM 行
        entry.mem = parts.get(7).and_then(|v| v.parse::<f32>().ok());
        entry.command = parts.get(8).unwrap_or(&"").to_string();
    }

    if entry.cpu.is_some() && entry.mem.is_some() {


        let filename = format!("{}_{}_pidstat_.log", entry.command, entry.pid);
        let path = Path::new(&filename);

        let file_exists = path.exists();
        let file_empty = file_exists
            .then(|| metadata(path).map(|m| m.len() == 0).unwrap_or(true))
            .unwrap_or(true);

        // 打开文件用于追加写入
        if let Ok(file) = OpenOptions::new().create(true).append(true).open(path) {
            let mut writer = BufWriter::new(file);
            // 如果是新文件或空文件，先写表头
            if !file_exists || file_empty {
                writeln!(writer, "Timestamp,Command,PID,CPU(%),MEM(%)").unwrap();
            }
            // 写数据行
            writeln!(
                writer,
                "{},{},{},{:.2},{:.2}",
                entry.timestamp,
                entry.command,
                entry.pid,
                entry.cpu.unwrap(),
                entry.mem.unwrap()
            ).unwrap();


        }
        process_map.remove(&pid);
    }
}



