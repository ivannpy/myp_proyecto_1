use ::config::{Config, Environment, File};
use serde::Deserialize;
use std::net::Ipv4Addr;

pub struct ServerConfig {
    port: u16,
    host: [u8; 4],
}

#[derive(Deserialize)]
pub struct ServerConfigRaw {
    port: u16,
    host: String,
}

#[derive(Deserialize)]
struct Settings {
    server: ServerConfigRaw,
}

impl ServerConfig {
    // Servidor por defecto
    fn default() -> Self {
        Self {
            port: 7878,
            host: [127, 0, 0, 1],
        }
    }

    pub fn from_raw(raw: ServerConfigRaw) -> Result<Self, String> {
        let host = parse_host(&raw.host)?;
        Ok(Self {
            port: raw.port,
            host,
        })
    }

    pub fn get_host(&self) -> [u8; 4] {
        self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}

fn parse_host(host: &str) -> Result<[u8; 4], String> {
    if host.eq_ignore_ascii_case("localhost") {
        return Ok([127, 0, 0, 1]);
    }

    let ip: Ipv4Addr = host
        .parse()
        .map_err(|_| format!("Host inválido: {host}."))?;

    Ok(ip.octets())
}

pub fn get_config() -> ServerConfig {
    // Intentamos leer el archivo de configuracion
    let settings = Config::builder()
        .add_source(File::with_name("config/dev"))
        .add_source(Environment::with_prefix("APP"))
        .build();

    match settings {
        Ok(settings) => {
            let parsed: Settings = settings
                .try_deserialize()
                .unwrap();

            ServerConfig::from_raw(parsed.server).unwrap()
        },
        Err(_) => ServerConfig::default()
    }

}
