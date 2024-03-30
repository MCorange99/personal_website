
use std::str::FromStr;
use clap::{Parser, Subcommand};
use uuid::Uuid;

use crate::database::{models::Permissions, Database};

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

    #[command(subcommand)]
    pub command: Option<CliArgsCommand>
}

#[derive(Debug, Clone, Subcommand)]
pub enum CliArgsCommand {
    #[command(arg_required_else_help = true)]
    GenerateToken {
        #[arg(long)]
        owner_id: String,
        #[arg(long)]
        permissions: i64,
    },
    #[command(arg_required_else_help = true)]
    CreateUser {
        #[arg(long)]
        email: String,
        #[arg(long)]
        username: String,
        #[arg(long)]
        password: String
    }
}


pub async fn handle_command(cli: &CliArgs, db: &mut Database) -> anyhow::Result<bool> {
    let Some(command) = cli.command.clone() else {
        return Ok(false);
    };


    match command {
        CliArgsCommand::GenerateToken {
            owner_id, permissions
        } => {
            let permissions = Permissions::from_bits(permissions).unwrap();
            let owner_id = Uuid::from_str(&owner_id)?;
            let token = crate::database::models::tokens::Token::create_new(db, owner_id, permissions).await?;
            log::info!("Generated token: {token:#?}");
            Ok(true)
        },
        CliArgsCommand::CreateUser {
            email, username, password
        } => {
            let user = crate::database::models::users::User::create_new(db, email, username, password).await?;
            log::info!("Created user: {user:?}");
            Ok(true)
        },
    }
}
