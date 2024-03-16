use lib_account::*;

use clap::Parser;
use levelcrush::{clap, tokio, tracing};

#[derive(clap::ValueEnum, Clone, Debug)]
enum Job {
    Server,
    DiscordUpdate,
    MigrateUp,
    MigrateUpAll,
    MigrateDown,
    MigrateDownAll,
    MigrateFresh,
    MigrateRefresh,
}

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(help = "The functionality you intend to run")]
    pub job: Job,

    #[arg(help = "Additional arguments to feed to the job")]
    pub args: Vec<String>,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tracing::info!("Merging .env into process enviorment settings");
    levelcrush::env();

    tracing::info!("Parsing command line arguments");
    let args = Args::parse();

    tracing::info!("Running job");
    let result = match args.job {
        Job::Server => jobs::server::run(10, 10).await,
        Job::DiscordUpdate => jobs::discord::run(&args.args).await,
        Job::MigrateUp => {
            let amount = args
                .args
                .first()
                .map_or(1, |v| v.parse::<u32>().unwrap_or(1));

            lib_account::jobs::migrate::up(amount).await
        }
        Job::MigrateDown => {
            let amount = args
                .args
                .first()
                .map_or(1, |v| v.parse::<u32>().unwrap_or(1));

            lib_account::jobs::migrate::down(amount).await
        }
        Job::MigrateUpAll => lib_account::jobs::migrate::up_all().await,
        Job::MigrateDownAll => lib_account::jobs::migrate::down_all().await,
        Job::MigrateFresh => lib_account::jobs::migrate::fresh().await,
        Job::MigrateRefresh => lib_account::jobs::migrate::refresh().await,
    };

    if let Err(err) = result {
        tracing::error!("An error was encountered during the job:\r\n{}", err);
    }
}
