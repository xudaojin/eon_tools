use crate::sub_commands;
use clap::{CommandFactory, Parser};
use clap_complete::Shell;
use spdlog::error;

#[derive(Parser)]
#[command(name = "eon")]
#[command(author = "xudaojin@waytous.com")]
#[command(about = "eon cli tools")]
pub struct Eon {
    #[command(subcommand)]
    pub eon_cmds: Option<EonCmds>
}

#[derive(Parser)]
pub enum EonCmds {
    #[command(about = "系统环境部署")]
    Deploy {
        #[command(subcommand)]
        sub_cmds: DeploySubCmds,
    },
    
    #[command(about = "录制指定进程的 CPU 内存占用情况")]
    PidstatRecord {

        #[arg(long, short, num_args = 1.., required = false, value_delimiter = ' ', help = "需要录制的进程 PID 值，多个 pid 通过空格隔开")]
        pids: Vec<i32>,

        #[arg(long, short, default_value_t = 1, help = "采样周期，默认1s")]
        interval: i8
    },
    
    #[command(about = "录制当前系统整体 CPU 资源的使用情况")]
    MpstatRecord {
        #[arg(long, short, default_value_t = 1, help = "采样周期，默认1s")]
        interval: i8
    },
    
    #[command(about = "创建智能补全功能")]
    Autocompletion{
        #[command(subcommand)]
        sub_cmds: Option<AutocompletionSubCmds>,
    },
    
    
}

#[derive(Parser)]
pub enum  AutocompletionSubCmds {

    #[command(about = "创建 Bash 补全功能")]
    Bash {},

    #[command(about = "创建 Zsh 补全功能")]
    Zsh {},
}

#[derive(Parser)]
pub enum DeploySubCmds {
    #[command(about = "一键部署ROS1")]
    Ros1 {
        #[arg(long, short, value_parser = ["noetic", "bionic"], help = "ROS1 版本代号")]
        code: Option<String>
    },
    
    #[command(about = "一键部署 ROS2")]
    Ros2 {
        #[arg(long, short, value_parser = ["noetic", "bionic"], help = "ROS2 版本代号")]
        code: Option<String>
    },
    
    #[command(about = "一键部署 Docker")]
    Docker {
        
    },
    
    #[command(about = "一键部署基础环境")]
    Dep {
        
    }
}


impl Eon {
    pub fn new() -> Self {
        Eon::parse()
    }
    
     pub async fn run(&self) -> std::io::Result<()>{
        match &self.eon_cmds { 

            Some(EonCmds::Deploy { sub_cmds }) => {
                sub_commands::deploy::run(sub_cmds)
            }
            Some(EonCmds::PidstatRecord {pids, interval}) => {
                if let Err(e) = sub_commands::pidstat::pidstat_data_record(pids, interval).await {
                    error!("pidstat record process failure: {}", e);
                }
            }
            Some(EonCmds::MpstatRecord {interval}) => {
                if let Err(e) = sub_commands::mpstat::mpstat_data_record(interval).await {
                    error!("mpstat record process failure: {}", e);
                }
            }
            Some(EonCmds::Autocompletion {sub_cmds}) => {
                match sub_cmds {
                    Some(cmd) => {
                        match cmd {
                            AutocompletionSubCmds::Bash { .. } => {
                                sub_commands::autocompletion::generate_completion(Self::command(), Shell::Bash)?;
                            }
                            AutocompletionSubCmds::Zsh { .. } => {
                                sub_commands::autocompletion::generate_completion(Self::command(), Shell::Zsh)?;
                            }
                        }
                    }
                    None => {
                        sub_commands::autocompletion::generate_completion(Self::command(), Shell::Bash)?;
                    }
                }
            }
            None => {
                Eon::command().print_help()?;
            }

        }
        Ok(())
    }
}