
use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Port to bind to
    #[arg(short, long, default_value_t=8080)]
    pub port: u16,

    /// Host ip to bind to, usually not required to change
    #[arg(short, long, default_value="0.0.0.0")]
    pub host: String,

    /// Extra debugging output
    #[arg(long, short)]
    pub debug: bool
}