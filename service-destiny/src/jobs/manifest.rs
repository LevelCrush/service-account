use crate::bungie::manifest::DestinyManifest;
use crate::bungie::BungieClient;
use crate::sync;
use levelcrush::database;
use levelcrush::tokio;
use levelcrush::tracing;

pub async fn run() {
    tracing::info!("Starting Bungie HTTP client");
    let client = BungieClient::new();

    // connect to our database
    tracing::info!("Connecting to database");
    let database = database::connect().await;

    tracing::info!("Fetching new manifest");

    // fetch a new manifest if possible
    let manifest = DestinyManifest::get(client.clone()).await;

    // this type of flow control is new to me. Just in case, copying the link here for anyone new to it as well or if I am using it wrong
    // tl;dr: if let similar to match, but simpler
    // https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
    if let Some(manifest) = manifest {
        tracing::info!("Updating manifest");

        // at the same time, make a external request to the class definitions and activity definitions urls that were provided by the destiny manifest
        let (
            class_definitions,
            activity_definitions,
            activity_type_definitions,
            season_definitions,
            records_definitions,
        ) = tokio::join!(
            manifest.get_class_definition(client.clone()),
            manifest.get_activity_definitions(client.clone()),
            manifest.get_activity_type_definitions(client.clone()),
            manifest.get_season_definitions(client.clone()),
            manifest.get_record_definition(client.clone())
        );

        // start syncing definitions
        tracing::info!("Syncing activity type definitions");
        tokio::join!(
            sync::definition::classes(&class_definitions, &database),
            sync::definition::activity_types(&activity_type_definitions, &database),
            sync::definition::activities(&activity_definitions, &database),
            sync::definition::seasons(&season_definitions, &database),
            sync::definition::records(&records_definitions, &database)
        );
    }
}
