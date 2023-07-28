use app::state::AppState;
use clap::Parser;
use database::platform::AccountPlatformType;
use env::AppVariable;
use levelcrush::{server::Server, tokio, tracing};
use std::time::Duration;

mod app;
mod database;
mod env;
mod routes;
mod sync;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Job {
    Server,
    DiscordUpdate,
}

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(help = "The functionality you intend to run")]
    pub job: Job,

    #[arg(help = "Additional arguments to feed to the job")]
    pub args: Vec<String>,
}

async fn job_discord_profiles(args: &[String]) {
    //todo!
    let state = AppState::new().await;
    let limit = {
        if !args.is_empty() {
            match args.first() {
                Some(v) => v.parse::<i64>().unwrap_or_default(),
                _ => 1000,
            }
        } else {
            1000
        }
    };

    let need_update = database::platform::need_update(AccountPlatformType::Discord, limit, &state.database).await;
    for discord_id in need_update.into_iter() {
        tracing::info!("Updating member: {}", discord_id);
        app::discord::member(&discord_id, &state).await;

        // intentionally add a delay between each request of 100ms
        // this is a lazy and innaccurate way of making sure we dont exceed our
        // global rate limit of 50 request per second
        // https://discord.com/developers/docs/topics/rate-limits#global-rate-limit
        // eventually we should examine the request for rate limit info
        // for now this works just fine
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    levelcrush::env();

    let args = Args::parse();
    match args.job {
        Job::Server => {
            let server_port = env::get(AppVariable::ServerPort).parse::<u16>().unwrap_or(3000);
            let app_state = AppState::new().await;

            let rate_limit = env::get(AppVariable::RateLimit).parse::<u64>().unwrap_or(100);
            let rate_limit_per = env::get(AppVariable::RateLimitDuration).parse::<u64>().unwrap_or(10);
            let rate_limit_buffer = env::get(AppVariable::RateLimitBuffer).parse::<u64>().unwrap_or(1024);

            let server_port = server_port;
            Server::new(server_port)
                .enable_cors()
                .enable_session()
                .enable_rate_limit(rate_limit, Duration::from_secs(rate_limit_per), rate_limit_buffer)
                .run(routes::router(), app_state.clone())
                .await;
        }
        Job::DiscordUpdate => {
            job_discord_profiles(&args.args).await;
        }
    };
}
