use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

// #[cfg(windows)]
// mod system_service;

pub mod cli;
pub mod controllers;
pub mod helpers;
pub mod models;
pub mod routes;

fn main() {
    let config = cli::get_config();

    let exit_code = match run_platform(config) {
        Ok(_) => 0,
        Err(error) => {
            println!("{}", error);
            // Todo: Log
            1
        }
    };

    std::process::exit(exit_code);
}

// #[cfg(not(windows))]
fn run_platform(config: &cli::Config) -> Result<(), String> {
    run_server(config)
}

// #[cfg(windows)]
// fn run_platform(config: &cli::Config) -> Result<(), String> {
//     if config.run_as_service {
//         println!("Running as service.");
//         system_service::run()
//     } else if config.register_service {
//         let install_result = system_service::install_service().map_err(|e| e.display_chain());
//         if install_result.is_ok() {
//             println!("Installed the service.");
//         }
//         install_result
//     } else {
//         println!("Running Directly");
//         run_server(config)
//     }
// }

fn run_server(config: &cli::Config) -> Result<(), String> {
    let traces =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "shopee_logs_collector=trace".to_owned());

    let port = config.port;

    tracing_subscriber::fmt()
        .with_env_filter(traces)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let db = models::blank_db();
    let api = routes::all(db);

    let fs = warp::fs::dir(config.html_path.to_owned());

    let routes = fs.or(api.with(warp::trace::request()));

    let mut rt = tokio::runtime::Runtime::new().map_err(|_| "Couldn't start runtime")?;

    info!("Listening on port {}", port);
    info!("Serving {}", config.html_path);

    rt.block_on(warp::serve(routes).run(([127, 0, 0, 1], port)));

    Ok(())
}
