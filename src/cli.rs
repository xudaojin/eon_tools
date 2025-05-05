use crate::commands;
use crate::sub_commands;
use clap::{CommandFactory, Parser};
use clap_complete::Shell;


// eon 结构体， 用于解析顶层命令
#[derive(Parser)]
#[command(name = "eon")]
#[command(author = "xudaojin@waytous.com")]
#[command(about = "eon cli tools")]
pub struct Eon {
    #[command(subcommand)]
    pub eon_cmds: Option<EonCmds>
}

// 注册 eon 二级子命令
#[derive(Parser)]
pub enum EonCmds {
    #[command(about = "系统环境部署")]
    Deploy {
        #[command(subcommand)]
        sub_cmds: commands::DeploySubCmds,
    },
    
    #[command(about = "录制指定进程的 CPU 内存占用情况")]
    Pidstat {
        
        #[command(subcommand)]
        sub_cmds: commands::PidstatSubCmds,

    
    },
    
    #[command(about = "录制当前系统整体CPU资源的使用情况")]
    Mpstat {
        
        #[command(subcommand)]
        sub_cmds: commands::MpstatSubCmds,

    },
    
    #[command(about = "创建智能补全功能")]
    Autocompletion{
        #[command(subcommand)]
        sub_cmds: Option<commands::AutocompletionSubCmds>,
    },
    
    
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
            Some(EonCmds::Pidstat { sub_cmds }) => {
                sub_commands::pidstat::run(sub_cmds).await;
            }
            Some(EonCmds::Mpstat { sub_cmds }) => {
                sub_commands::mpstat::run(sub_cmds).await;
            }
            Some(EonCmds::Autocompletion {sub_cmds}) => {
                match sub_cmds {
                    Some(cmd) => {
                        match cmd {
                            commands::AutocompletionSubCmds::Bash { .. } => {
                                sub_commands::autocompletion::generate_completion(Self::command(), Shell::Bash)?;
                            }
                            commands::AutocompletionSubCmds::Zsh { .. } => {
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