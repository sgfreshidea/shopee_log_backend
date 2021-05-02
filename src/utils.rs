use tracing::info;
use warp::Filter;
use crate::cli;
use crate::models;
use crate::routes;


pub fn run_server(shutdown_rx: Option<tokio::sync::mpsc::Receiver<()>>, from_service: bool) -> Result<(), String> {
    info!("Booting the server");
    let config = cli::get_config(from_service);

    let port = config.port;

    let db = models::blank_db();
    let api = routes::all(db.clone());

    let fs = warp::fs::dir(config.html_path.to_owned());

    let routes = fs.or(api.with(warp::trace::request()));

    info!("Listening on  http://127.0.0.1:{}", port);
    info!("Serving {} ", config.html_path);


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
