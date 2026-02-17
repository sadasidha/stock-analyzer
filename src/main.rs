mod ctrls;
mod utils;



use std::env;

use clap::{Parser, Subcommand};
use sqlx::{MySqlPool};

use crate::ctrls::pull::pull;


#[derive(Parser)]
#[command(name = "stock-cli")]
#[command(about = "JPX Stock Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Pull stock data (from API / Yahoo / etc.)
    Pull {
        #[arg(long)]
        code: String,
    },

    /// Analyze stock data from DB
    Analyze {
        #[arg(long)]
        code: String,

        #[arg(long, default_value_t = 30)]
        days: i64,
    },
}

#[tokio::main]
async fn main() -> Result<(), APpError> {
    let cli = Cli::parse();

    let database_url =
        std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    match cli.command {
        Commands::Pull { code } => {
            pull(&pool, code).await?;
        }

        Commands::Analyze { code, days } => {
            analyze(&pool, days).await?;
        }
    }
    Ok(())
}
