mod env;
mod jobs;
mod routes;

use clap::Parser;

// use the tokio install that we are using with our level crush library
use levelcrush::{tokio, tracing};

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

    let result = match args.job {
        Job::Server => jobs::server::run().await,
        Job::SyncManifest => lib_destiny::jobs::manifest::run().await,
        Job::ClanInfo => lib_destiny::jobs::clan::info(&args.args).await,
        Job::ClanRoster => lib_destiny::jobs::clan::roster(&args.args).await,
        Job::ClanCrawl => lib_destiny::jobs::clan::crawl_direct(&args.args).await,
        Job::ClanNotNetworkCrawl => lib_destiny::jobs::clan::crawl_non_network().await,
        Job::ClanMakeNetwork => lib_destiny::jobs::clan::make_network(&args.args).await,
        Job::ClanMakeNonNetwork => lib_destiny::jobs::clan::make_non_network(&args.args).await,
        Job::MemberProfile => lib_destiny::jobs::member::profile(&args.args).await,
        Job::MemberActivity => lib_destiny::jobs::activity::history(&args.args).await,
        Job::MemberCrawl => lib_destiny::jobs::member::crawl_profile(&args.args).await,
        Job::MemberCrawlDeep => lib_destiny::jobs::member::crawl_profile_deep(&args.args).await,
        Job::NetworkCrawl => lib_destiny::jobs::clan::crawl_network2().await,
        Job::InstanceCrawl => lib_destiny::jobs::activity::crawl_instances(&args.args).await,
        Job::InstanceProfiles => lib_destiny::jobs::activity::instance_member_profiles(&args.args).await,
        Job::Reset => lib_destiny::jobs::reset::run().await,
        Job::Purge => lib_destiny::jobs::purge::run().await,
    };

    if let Err(err) = result {
        tracing::error!("An error was encountered during the job:\r\n{}", err);
    }
}
