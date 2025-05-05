use clap::Parser;

#[derive(Parser)]
pub enum AutocompletionSubCmds {
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
#[derive(Parser)]
pub enum PidstatSubCmds {
    
    #[command(about = "pidstat data record")]
    Record {
        #[arg(long, short, num_args = 1.., required = false, value_delimiter = ' ', help = "需要录制的进程 PID 值，多个 pid 通过空格隔开")]
        pids: Vec<i32>,

        #[arg(long, short, default_value_t = 1, help = "采样周期，默认1s")]
        interval: i8
    },
    
    Show {
        #[arg(long, short, num_args = 1.., required = false, value_delimiter = ' ', help = "需要查看的进程 PID 值，多个 pid 通过空格隔开")]
        pids: Vec<i32>,

        #[arg(long, short, default_value_t = 1, help = "采样周期，默认1s")]
        interval: i8
    }
}

#[derive(Parser)]
pub enum MpstatSubCmds {
    
    #[command(about = "mpstat data record")]
    Record {
        #[arg(long, short, default_value_t = 1, help = "采样周期，默认1s")]
        interval: i8
    }
}


