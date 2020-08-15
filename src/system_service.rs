use crate::cli;

use std::{
    env,
    ffi::OsString,
    ptr, slice,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc, Arc,
    },
    thread,
    time::{Duration, Instant},
};

use winapi::{
    ctypes::c_void,
    shared::{
        minwindef::ULONG,
        ntdef::{LUID, PVOID, WCHAR},
        ntstatus::STATUS_SUCCESS,
    },
    um::{
        ntlsa::{
            LsaEnumerateLogonSessions, LsaFreeReturnBuffer, LsaGetLogonSessionData,
            SECURITY_LOGON_SESSION_DATA,
        },
        sysinfoapi::GetSystemDirectoryW,
    },
};

use windows_service::{
    service::{
        PowerEventParam, Service, ServiceAccess, ServiceAction, ServiceActionType, ServiceControl,
        ServiceControlAccept, ServiceDependency, ServiceErrorControl, ServiceExitCode,
        ServiceFailureActions, ServiceFailureResetPeriod, ServiceInfo, ServiceSidType,
        ServiceStartType, ServiceState, ServiceStatus, ServiceType, SessionChangeReason,
    },
    service_control_handler::{self, ServiceControlHandlerResult, ServiceStatusHandle},
    service_dispatcher,
    service_manager::{ServiceManager, ServiceManagerAccess},
};



static SERVICE_NAME: &'static str = "ShopeeLogCollector";
static SERVICE_DISPLAY_NAME: &'static str = "Shopee Log Collector Service";
static SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;

pub fn run() -> Result<(), String> {
    // Start the service dispatcher.
    // This will block current thread until the service stopped and spawn `service_main` on a
    // background thread.
    service_dispatcher::start(SERVICE_NAME, service_main)
        .map_err(|e| e.display_chain_with_msg("Failed to start a service dispatcher"))
}

windows_service::define_windows_service!(service_main, handle_service_main);

pub fn handle_service_main(_arguments: Vec<OsString>) {
    tracing::info!("Service started.");
    match run_service() {
        Ok(()) => tracing::info!("Service stopped."),
        Err(error) => tracing::error!("{}", error),
    };
}


fn run_service() -> Result<(), String> {

    let (event_tx, event_rx) = mpsc::channel();


     // Register service event handler
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            // Notifies a service to report its current status information to the service
            // control manager. Always return NO_ERROR even if not implemented.
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,

            ServiceControl::Stop
            | ServiceControl::Preshutdown
            | ServiceControl::PowerEvent(_)
            | ServiceControl::SessionChange(_) => {
                event_tx.send(control_event).unwrap();
                ServiceControlHandlerResult::NoError
            }

            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

       let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)
        .map_err(|e| e.display_chain_with_msg("Failed to register a service control handler"))?;


         let mut persistent_service_status = PersistentServiceStatus::new(status_handle);
    persistent_service_status
        .set_pending_start(Duration::from_secs(1))
        .unwrap();


         let clean_shutdown = Arc::new(AtomicBool::new(false));
