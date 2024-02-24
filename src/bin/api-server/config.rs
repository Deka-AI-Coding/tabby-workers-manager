use clap::Parser;

#[derive(Debug, Parser, Clone)]
pub struct Config {
    /// Port to serve from
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    /// Key used to auth
    #[arg(long)]
    pub key: String,
}
