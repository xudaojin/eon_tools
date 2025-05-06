use clap_complete::{generate, Shell};
use spdlog::{error, info};
use std::env;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub fn generate_completion(mut app: clap::Command, shell_type: Shell) -> io::Result<()> {
    
    // 获取用户家目录
    let home_path = match env::var("HOME") {
        Ok(var) => {
            PathBuf::from(var)
        }
        Err(e) => {
            error!("Get HOME path failed: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to get HOME path"));
        }
    };
    
    let file_path = match shell_type { 
        Shell::Bash => {
            home_path.join(".bash_completion")
        },
        Shell::Zsh => {
            home_path.join(".zsh_completion")
        },
        _ => {
            home_path.join(".bash_completion")
        },
    };
    
    let mut file = File::create(&file_path)?;
    let app_name = app.get_name().to_string();
    generate(shell_type, &mut app, app_name, &mut file);
    info!("Create Successful");
    info!("file path: [{:?}]", file_path);
    Ok(())
}
