use std::{ffi::OsString, time::Duration};
use tracing::info;

use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher, Result,
};
const SERVICE_NAME: &str = "shopee_bot_log_collector";
const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

pub fn run() -> Result<()> {
    info!("system_service::run started.");
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;

    info!("system_service::run exited.");

    Ok(())
}

define_windows_service!(ffi_service_main, my_service_main);

pub fn my_service_main(_arguments: Vec<OsString>) {
    info!("my service main started.");
    if let Err(e) = run_service() {
        info!("There is error when running the service.");
        info!("my_service_main {:?}", e);
    }
    info!("my service main exited.");
}

pub fn run_service() -> Result<()> {
    info!("Start run_service.");
    // Create a channel to be able to poll a stop event from the service worker loop.
    let (shutdown_tx, shutdown_rx) = tokio::sync::mpsc::channel(10);

    // Define system service event handler that will be receiving service events.
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        let mut shutdown_tx = shutdown_tx.clone();
        match control_event {
            // Notifies a service to report its current status information to the service
            // control manager. Always return NoError even if not implemented.
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

            // Handle stop
            ServiceControl::Stop => {
                tokio::spawn(async move {
                    shutdown_tx.send(()).await.unwrap();
                });
                ServiceControlHandlerResult::NoError
            }

            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    // Register system service event handler.
    // The returned status handle should be used to report service status changes to the system.
    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    info!("Letting system know service is running");
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    // Run the app

    info!("Booting the server from service");

    crate::run_server(Some(shutdown_rx)).unwrap();

    info!("Closing the server as it seems warp server has closed due to panic?");

    // Tell the system that service has stopped.
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    Ok(())
}

pub fn install_service() -> Result<()> {
    use windows_service::{
        service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType},
        service_manager::{ServiceManager, ServiceManagerAccess},
    };

    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    info!("Starting to install service");
    let service_binary_path = ::std::env::current_exe()
        .unwrap()
        .with_file_name("shopee_logs_collector.exe");

    let service_binary_str = service_binary_path.to_string_lossy();

    info!("File Binary Path {:?}", service_binary_str);

    let service_info = ServiceInfo {
        name: OsString::from(SERVICE_NAME),
        display_name: OsString::from("Shopee Log Service"),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::OnDemand,
        error_control: ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None, // run as System
        account_password: None,
    };
    let _service = service_manager.create_service(&service_info, ServiceAccess::empty())?;

    info!("Service Installed without any error");
    Ok(())
}
