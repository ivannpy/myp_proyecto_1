use ::config::{Config, Environment, File};

pub struct ServerConfig {
    port: u16,
    host: [u8; 4],
}

impl ServerConfig {
    // Servidor por defecto
    pub fn new() -> Self {
        Self {
            port: 7878,
            host: [127, 0, 0, 1],
        }
    }

    pub fn get_host(&self) -> [u8; 4] {
        self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}

pub fn get_config() {
    let settings = Config::builder()
        .add_source(File::with_name("config/dev"))
        .add_source(Environment::with_prefix("APP"))
        .build()
        .unwrap();
    
    //let server_config = settings.get("cache");
    println!("{:?}", settings);
}
