use crate::cli;

pub fn run(sub_cmds: &cli::DeploySubCmds) {
    match sub_cmds { 
        cli::DeploySubCmds::Ros1 {code} => {
            ros1_install(code);
        }
        cli::DeploySubCmds::Ros2 {code} => {
            ros2_install(code);
        }
        cli::DeploySubCmds::Docker {} => {
            docker_install();
        }
        cli::DeploySubCmds::Dep {} => {
            dep_install();
        }
    }
}


fn ros1_install(_code: &Option<String>) -> bool {
    true
}

fn ros2_install(_code: &Option<String>) -> bool {
    true
}

fn docker_install() -> bool {
    true    
}

fn dep_install() -> bool {
    true
}