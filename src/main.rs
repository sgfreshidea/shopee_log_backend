use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

#[cfg(windows)]
mod system_service;

pub mod cli;
pub mod controllers;
pub mod helpers;
pub mod models;
pub mod routes;

fn main() {
    let log_file_dir = std::env::current_exe().unwrap().with_file_name("");
    let file_appender =
        RollingFileAppender::new(Rotation::NEVER, log_file_dir, "shopee_logger.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
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

#[cfg(not(windows))]
fn run_platform() -> Result<(), Box<dyn std::error::Error>> {
    run_server(config)
}

#[cfg(windows)]
fn run_platform() -> Result<(), Box<dyn std::error::Error>> {
    let config = cli::get_config();

    info!(
        "Run As Service: {} , Register Service: {}",
        config.run_as_service, config.register_service
    );

    if config.run_as_service {
        system_service::run()?;
    } else if config.register_service {
        system_service::install_service()?;
    } else {
        println!("Running Directly");
        run_server(None)?;
    }

    info!("Exiting run_platform.");

    Ok(())
}

fn run_server(shutdown_rx: Option<tokio::sync::mpsc::Receiver<()>>) -> Result<(), String> {
    info!("Booting the server");
    let config = cli::get_config();

    let port = config.port;

    let db = models::blank_db();
    let api = routes::all(db.clone());

    let fs = warp::fs::dir(config.html_path.to_owned());

    let routes = fs.or(api.with(warp::trace::request()));

    info!("Listening on  http://127.0.0.1:{}", port);
    info!("Serving {}", config.html_path);

    let server = warp::serve(routes).run(([127, 0, 0, 1], port));
    let clear_logs_future = crate::models::clear_database_periodically(db.clone());

    let mut rt = tokio::runtime::Runtime::new().map_err(|_| "Error on tokio runtime".to_owned())?;

    let server_future = async { tokio::join!(server, clear_logs_future) };

    if shutdown_rx.is_some() {
        let mut shutdown_rx = shutdown_rx.unwrap();
        let fut = async move {
            tokio::select! {
            _ = server_future => { println!("Warp Server has stopped");},
            _ =  shutdown_rx.recv() => {println!("Windows server has told warp server to stop");}
            }
        };

        rt.block_on(fut);
    } else {
        rt.block_on(server_future);
    }

    Ok(())
}
