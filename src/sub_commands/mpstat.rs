use spdlog::{error, info};
use crate::commands;
pub async fn run(sub_cmds: &commands::MpstatSubCmds) {
    match sub_cmds { 
        commands::MpstatSubCmds::Record {interval} => {
            if let Err(e) = mpstat_data_record(interval).await {
                error!("{:?}", e);
            }
        }
    }
}
async fn mpstat_data_record(_interval: &i8) -> std::io::Result<()> {
    info!("dwd");
    Ok(())
}
