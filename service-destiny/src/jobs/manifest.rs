use levelcrush::anyhow;
use levelcrush::database;
use levelcrush::tokio;
use levelcrush::tracing;
use lib_destiny::api::manifest::DestinyManifest;
use lib_destiny::bungie::BungieClient;

use crate::env;
use crate::env::AppVariable;
use crate::sync;

pub async fn run() -> anyhow::Result<()> {
    tracing::info!("Starting Bungie HTTP client");
    let api_key = env::get(AppVariable::BungieAPIKey);
    let client = BungieClient::new(api_key);

    // connect to our database
    tracing::info!("Connecting to database");
    let database = database::connect("destiny.sqlite", 1).await;

    tracing::info!("Fetching new manifest");

    // fetch a new manifest if possible
    let manifest = DestinyManifest::get(client.clone()).await?;

    // this type of flow control is new to me. Just in case, copying the link here for anyone new to it as well or if I am using it wrong
    // tl;dr: if let similar to match, but simpler
    // https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
    if let Some(manifest) = manifest {
        tracing::info!("Updating manifest");

        // at the same time, make a external request to the class definitions and activity definitions urls that were provided by the destiny manifest
        tracing::info!("Fetching class definitions");
        let class_definitions = manifest.get_class_definition(client.clone()).await?;

        tracing::info!("Syncing class definitions");
        sync::definition::classes(&class_definitions, &database).await;

        tracing::info!("Fetching activity definitions");
        let activity_definitions = manifest.get_activity_definitions(client.clone()).await?;

        tracing::info!("Syncing activity definitions");
        sync::definition::activities(&activity_definitions, &database).await;

        tracing::info!("Fetching activity type definitions");
        let activity_type_definitions = manifest.get_activity_type_definitions(client.clone()).await?;

        tracing::info!("Syncing activity type definitions");
        sync::definition::activity_types(&activity_type_definitions, &database).await;

        tracing::info!("Fetching season definitions");
        let season_definitions = manifest.get_season_definitions(client.clone()).await?;

        tracing::info!("Syncing season definitions");
        sync::definition::seasons(&season_definitions, &database).await;

        tracing::info!("Fetching triumph record definitions");
        let records_definitions = manifest.get_record_definition(client.clone()).await?;

        tracing::info!("Syncing triumph definitions");
        sync::definition::records(&records_definitions, &database).await;
    }
    Ok(())
}
