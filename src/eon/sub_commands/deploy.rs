use crate::commands;
pub fn run(sub_cmds: &commands::DeploySubCmds) {
    match sub_cmds {
        commands::DeploySubCmds::Ros1 {code} => {
            ros1_install(code);
        }
        commands::DeploySubCmds::Ros2 {code} => {
            ros2_install(code);
        }
        commands::DeploySubCmds::Docker {} => {
            docker_install();
        }
        commands::DeploySubCmds::Dep {} => {
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