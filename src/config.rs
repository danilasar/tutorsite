use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_potgres_url")]
    pub postgres_url: String
}

fn default_port() -> u16 {
    8080
}

fn default_potgres_url() -> String {
    "postgres://postgres:postgres@localhost/tutors".to_string()
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

