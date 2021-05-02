use shopee_logs_collector::{system_service,utils};
use tracing::info;
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

    info!("Starting to run service...");
   
   let service =  system_service::run();
    let exit_code = match service {
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
