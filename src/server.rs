pub mod servercfg;
mod ioutils;

use std::process::Child;

use servercfg::ServerConfig;




//=================================================================
//======================   CONSTANTS & OOP   ======================
//=================================================================
const LOG_PATH: &str = "logs/server_jar.log";

pub struct Server {
    config: Option<ServerConfig>,
    state: State,
    process: Option<Child>,
}

enum State {
    HOSTED(String),
    RUNNING,
    STOPPED,
}

pub enum ServerError {
    NO_CONFIG,
    RUNNING,
    HOSTED(String),
}

impl Server {
    pub fn new() -> Server {
        Server { 
            config: None,
            state: State::STOPPED,
            process: None, 
        }
    }

    pub fn configure(&mut self, config: ServerConfig) {
        self.config = Some(config);
    }

    pub fn get_config(&self) -> &Option<ServerConfig> {
        &self.config
    }

    pub fn is_config(&self) -> bool {
        if let None = self.config { false }
        else { true }
    }

    pub fn run(&mut self) -> Result<(), ServerError> {
        if let None = self.config {
            return Err(ServerError::NO_CONFIG)
        }
        match &(self.state) {
            State::RUNNING => Err(ServerError::RUNNING),
            State::HOSTED(host) => Err(ServerError::HOSTED(String::from(host))),
            State::STOPPED => { 
                self.execute_server_jar()
                .expect("Could not execute the server binary (server/server.jar)"); 
                self.state = State::RUNNING;
                Ok(())
            }
        }
    }

    fn execute_server_jar(&mut self) -> Result<(), std::io::Error> {
        if let Some(config) = &(self.config) {
            let program = "java";
            let dir = "mojang/";
            let mut args = [
                format!("-Xmx{}", config.get_mem_max()),
                format!("-Xms{}", config.get_mem_init()),
                String::from("-jar"),
                String::from("server.jar"),
                String::from("")
            ];

            if !config.get_gui() { args[4] = String::from("--nogui"); }

            self.process = Some(ioutils::terminal::execute_command(
                program, args, dir, LOG_PATH
            )?);
        }
        
        Ok(())
    }
}




//=================================================================
//====================   AUXILIAR FUNCTIONS   =====================
//=================================================================
pub fn get_error_msg(err: ServerError) -> String {
    match err {
        ServerError::NO_CONFIG => String::from("Server has not been configured yet!"),
        ServerError::RUNNING => String::from("Server is already running on your device!\n"),
        ServerError::HOSTED(host) => String::from(format!("Server is being hosted by {}", host.as_str()))
    }
}