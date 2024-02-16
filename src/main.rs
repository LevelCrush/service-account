use lib_account::*;

use clap::Parser;
use levelcrush::{clap, tokio, tracing};

#[derive(clap::ValueEnum, Clone, Debug)]
enum Job {
    Server,
    DiscordUpdate,
    Purge,
    Reset,
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
        Job::Server => jobs::server::run().await,
        Job::DiscordUpdate => jobs::discord::run(&args.args).await,
        Job::Purge => jobs::purge::run().await,
        Job::Reset => jobs::reset::run().await,
    };

    if let Err(err) = result {
        tracing::error!("An error was encountered during the job:\r\n{}", err);
    }
}
