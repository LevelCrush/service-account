use levelcrush::anyhow;
use lib_destiny::{
    database,
    env::{AppVariable, Env},
    jobs,
};

pub async fn run(env: &Env) -> anyhow::Result<()> {
    // purging database

    jobs::reset::run().await?;

    // setup manifest
    jobs::manifest::run(env).await?;

    // clan info the default work
    let network = env.get_array(AppVariable::Network);
    jobs::clan::info(&network, env).await?;

    // mark them as network
    jobs::clan::make_network(&network, env).await?;

    // crawl the network now
    jobs::clan::crawl_network2(env).await?;

    Ok(())
}
