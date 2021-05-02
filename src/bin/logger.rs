use tracing::info;
use shopee_logs_collector::{cli,utils,system_service};
use cli::Action;
use std::process::Command;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    let log_file_dir = std::env::current_exe().unwrap().with_file_name("");
    let file_appender =
    RollingFileAppender::new(Rotation::NEVER, log_file_dir, "shopee_logger.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter("shopee_logs_collector=trace")
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    info!("Starting...");

    let exit_code = match run_platform() {
        Ok(_) => 0,
        Err(error) => {
            info!("There is error in main");
            println!("{:?}", error);

            info!(?error);
            1
        }
    };

    info!("Exiting...");

    std::process::exit(exit_code);
}

// We assume we are running this on windows only
// shirshak55: As we are not using it for linux/osx 
// I think it is better we optimize this for windows only
fn run_platform() -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting config");
    let config = cli::get_config(false);

    info!("Action: {:?}", config.action);
    match config.action {
        Action::RegisterService => {
            system_service::install_service()?;
            println!("Succesfully Installed")
        }
        Action::RemoveService => {
            system_service::install_service()?;
            println!("Succesfully Installed")
        }
        Action::RunDirect => {
            utils::run_server(None, false)?;
        }
        Action::RunService => {
            println!("Make sure you are running using admin rights");
            let output = Command::new("sc").args(&["start","shopee_service"]).output().expect("Failed to launch");
            println!("Stdout {:?}", output.stdout);
            println!("Stderr {:?}", output.stderr);
        }
    }

    info!("Exiting run_platform.");
    Ok(())
}