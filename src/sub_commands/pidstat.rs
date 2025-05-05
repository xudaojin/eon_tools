use spdlog::error;
use crate::common;
use crate::commands;

// pidstat 命令主入口
pub async  fn run(sub_cmds: &commands::PidstatSubCmds) {
    match sub_cmds {
        commands::PidstatSubCmds::Record{pids, interval} => {
            pidstat_data_record(pids, interval).await;
        }
        commands::PidstatSubCmds::Show {pids, interval} => {
            pidstat_data_show(pids, interval).await;
        }
    }
}


/// 采集指定进程的系统资源占用情况
pub async fn pidstat_data_record(_pids: &Vec<i32>, _interval: &i8){


}

/// 显示指定进程的系统资源占用情况
pub async fn pidstat_data_show(_pids: &Vec<i32>, _interval: &i8){
    get_data_process(_pids, _interval).await;
}


// 获取指定进程数据
async fn get_data_process(pids: &Vec<i32>, interval: &i8){
    let mut args: Vec<String> = vec!["-ur".to_string()];

    for pid in pids {
        args.push("-p".to_string());
        args.push(pid.to_string());
    }
    args.push(interval.to_string());
    let args_ref: Vec<&str> = args.iter().map(AsRef::as_ref).collect();

    match common::util::run_cmd_async("pidstat", &args_ref).await { 
        Ok(mut result) => {
            loop {
                match result.next_line().await { 
                    
                    // get data
                    Ok(Some(line)) => {
                        // 跳过空行
                        if line.is_empty() {
                            continue;
                        }
                        
                        // 通过空格进行分割
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        
                        
                        // let timestamp = fields[0].to_string();
                        // let command = fields[9].to_string();
                        // let pid: i32 = fields[2].parse().unwrap_or(0);
                        // let cpu: f32 = fields[7].parse().unwrap_or(0.0);
                        // let mem: f32 = fields[8].parse().unwrap_or(0.0);
                        
                        // let buffer = common::util::PidStatBuffer{
                        //     timestamp,
                        //     command,
                        //     pid,
                        //     cpu,
                        //     mem,
                        // };

                        println!("{:?}", fields);
                    }
                    
                    // EOF
                    Ok(None) => break,
                    
                    Err(e) => {
                        error!("get data error: {:?}", e);
                        break;
                    }
                }
            }
        }
        Err(e) =>{
            error!("{}",e)
        }
    }

}



