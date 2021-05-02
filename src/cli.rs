use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};

static mut CFG: Option<Config> = None;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub action: Action,
    pub port: u16,
    pub html_path: String,
}

#[derive(Debug,  Serialize, Deserialize)]
pub enum Action {
    RegisterService,
    RemoveService,
    RunService,
    RunDirect
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigBuilder {
    pub action: Option<String>,
    pub port: Option<u16>,
    pub html_path: Option<String>,
}

pub fn get_config(from_service: bool) -> &'static Config {
   unsafe {
        if CFG.is_some() {
            return CFG.as_ref().unwrap();
        }

            CFG =   Some(create_config(from_service));

        CFG.as_ref().unwrap()
    }
}

pub fn create_config(from_service: bool) -> Config {
    let config_toml_path =  std::env::current_exe().unwrap().with_file_name("config.toml");
    println!("The config toml path is {:?}", config_toml_path);
    
    let file = std::fs::read_to_string(
       config_toml_path
    ).expect("Please check config.toml file");

    let cfg: ConfigBuilder = toml::from_str(&file).expect("Invalid config.toml file");
    
    let action  = if from_service == false {
        println!("Please specify the action. ");
        println!("
            1. Register the service
            2. Remove the service
            3. Run the service (requires registration)
            4. Run directly on terminal
        ");
    
        print!("Enter the action (1,2,3,4): ");
        let stdin = io::stdin();
        let line1 = stdin.lock().lines().next().expect("Invalid input supplied").expect("Invalid input supplied 2");
        let value = line1.parse::<isize>().expect("invalid data given");
    
       match  value {
            1 => Action::RegisterService,
            2 => Action::RemoveService,
            3 => Action::RunService,
            4 => Action::RunDirect,
            _ => Action::RunDirect,
        }
    } else {
        Action::RunDirect
    };

    Config {
        action,
        port: cfg.port.unwrap_or(1729),
        html_path: cfg.html_path.expect("Please specify html path"),
    }
}
