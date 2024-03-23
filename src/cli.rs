
use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Port to bind to
    #[arg(short, long)]
    pub port: Option<u16>,

    /// Host ip to bind to, usually not required to change
    #[arg(long)]
    pub host: Option<String>,

    /// Extra debugging output
    #[arg(long, short)]
    pub debug: bool,

    #[arg(long, short, default_value="./config.toml")]
    pub config: camino::Utf8PathBuf,
}