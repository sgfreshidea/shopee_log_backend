use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use std::time::Duration;
#[cfg(windows)]
use windows_service::service;
#[cfg(windows)]
use windows_service::service_control_handler;

#[cfg(windows)]
windows_service::define_windows_service!(service_entry_point, service_main);

#[cfg(windows)]
const SERVICE_NAME: &str = "Shopee Log Collector";
#[cfg(windows)]
const SERVICE_TYPE: service::ServiceType = service::ServiceType::OWN_PROCESS;

#[cfg(windows)]
fn service_main(_arguments: Vec<OsString>) {
    let event_handler =
        move |control_event| -> service_control_handler::ServiceControlHandlerResult {
            match control_event {
                service::ServiceControl::Interrogate => {
                    service_control_handler::ServiceControlHandlerResult::NoError
                }
                service::ServiceControl::Stop => {
                    service_control_handler::ServiceControlHandlerResult::NoError
                }
                _ => service_control_handler::ServiceControlHandlerResult::NotImplemented,
            }
        };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler).unwrap();
    status_handle
        .set_service_status(service::ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: service::ServiceState::Running,
            controls_accepted: service::ServiceControlAccept::STOP,
            exit_code: service::ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
        })
        .unwrap();

    run_server();

    // In future we can pass oneshot channel
    status_handle
        .set_service_status(service::ServiceStatus {
            service_type: SERVICE_TYPE,
            current_state: service::ServiceState::Stopped,
            controls_accepted: service::ServiceControlAccept::empty(),
            exit_code: service::ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::default(),
        })
        .unwrap();
}

pub mod controllers;
pub mod helpers;
pub mod models;
pub mod routes;

#[cfg(unix)]
fn main() {
    run_server();
}

#[cfg(windows)]
fn main() {
    windows_service::service_dispatcher::start(SERVICE_NAME, service_entry_point).unwrap();
}

fn run_server() {
    let traces =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "shopee_logs_collector=trace".to_owned());

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1729);

    tracing_subscriber::fmt()
        .with_env_filter(traces)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let db = models::blank_db();
    let api = routes::all(db);

    let fs = warp::fs::dir(
        std::env::var("HTML_DIR")
            .unwrap_or_else(|_| "/Users/quantum/Desktop/code/shopee/out".to_owned()),
    );
    let routes = fs.or(api.with(warp::trace::request()));

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(warp::serve(routes).run(([127, 0, 0, 1], port)));
}
