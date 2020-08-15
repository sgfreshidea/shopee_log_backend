use clap::{App, Arg};
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Config {
    pub run_as_service: bool,
    pub register_service: bool,
    pub port: u16,
    pub html_path: String,
}

pub fn get_config() -> &'static Config {
    static CFG: Lazy<Config> = Lazy::new(|| create_config());

    &*CFG
}

pub fn create_config() -> Config {
    let app = create_app();
    let matches = app.get_matches();

    let run_as_service = cfg!(windows) && matches.is_present("run_as_service");
    let register_service = cfg!(windows) && matches.is_present("register_service");

    Config {
        run_as_service,
        register_service,
        port: matches
            .value_of("port")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1729),

        html_path: matches
            .value_of("html_path")
            .and_then(|v| Some(v.to_owned()))
            .unwrap(),
    }
}

fn create_app() -> App<'static, 'static> {
    let mut app = App::new("Shopee Log Collector".to_owned())
        .version("1.0.0")
        .about("Shopee Log Collector")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Set level of verbosity"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .required(false)
                .takes_value(true)
                .help("Set Port to listen to"),
        )
        .arg(
            Arg::with_name("html_path")
                .long("html_path")
                .required(true)
                .takes_value(true)
                .help("Set html path which shall be served as homepage"),
        );

    if cfg!(windows) {
        app = app.arg(
            Arg::with_name("run_as_service")
                .long("run-as-service")
                .help("Run as a system service. On Windows this option must be used when running a system service"),
        ).arg(
            Arg::with_name("register_service")
                .long("register-service")
                .help("Register itself as a system service"),
        )
    }

    app
}
