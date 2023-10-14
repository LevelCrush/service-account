mod env;
mod jobs;
mod routes;

use clap::Parser;

// use the tokio install that we are using with our level crush library
use levelcrush::{tokio, tracing};
use lib_destiny::env::Env;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Job {
    Server,
    SyncManifest,
    ClanInfo,
    ClanRoster,
    ClanCrawl,
    ClanNotNetworkCrawl,
    ClanMakeNetwork,
    ClanMakeNonNetwork,
    MemberProfile,
    MemberActivity,
    MemberCrawl,
    MemberCrawlDeep,
    NetworkCrawl,
    InstanceCrawl,
    InstanceProfiles,
    Reset,
    Purge,
    Setup,
    SheetsTest,
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
    // setup the levelcrush env
    levelcrush::env();

    // parse command line arguments
    let args = Args::parse();

    let env = env::load();

    let result = match args.job {
        Job::Server => jobs::server::run(&env).await,
        Job::SyncManifest => lib_destiny::jobs::manifest::run(&env).await,
        Job::ClanInfo => lib_destiny::jobs::clan::info(&args.args, &env).await,
        Job::ClanRoster => lib_destiny::jobs::clan::roster(&args.args, &env).await,
        Job::ClanCrawl => lib_destiny::jobs::clan::crawl_direct(&args.args, &env).await,
        Job::ClanNotNetworkCrawl => lib_destiny::jobs::clan::crawl_non_network(&env).await,
        Job::ClanMakeNetwork => lib_destiny::jobs::clan::make_network(&args.args, &env).await,
        Job::ClanMakeNonNetwork => lib_destiny::jobs::clan::make_non_network(&args.args, &env).await,
        Job::MemberProfile => lib_destiny::jobs::member::profile(&args.args, &env).await,
        Job::MemberActivity => lib_destiny::jobs::activity::history(&args.args, &env).await,
        Job::MemberCrawl => lib_destiny::jobs::member::crawl_profile(&args.args, &env).await,
        Job::MemberCrawlDeep => lib_destiny::jobs::member::crawl_profile_deep(&args.args, &env).await,
        Job::NetworkCrawl => lib_destiny::jobs::clan::crawl_network2(&env).await,
        Job::InstanceCrawl => lib_destiny::jobs::activity::crawl_instances(&args.args, &env).await,
        Job::InstanceProfiles => lib_destiny::jobs::activity::instance_member_profiles(&args.args, &env).await,
        Job::Reset => lib_destiny::jobs::reset::run().await,
        Job::Purge => lib_destiny::jobs::purge::run().await,
        Job::Setup => jobs::setup::run(&env).await,
        Job::SheetsTest => jobs::sheets::test_job().await,
    };

    if let Err(err) = result {
        tracing::error!("An error was encountered during the job:\r\n{}", err);
    }
}
