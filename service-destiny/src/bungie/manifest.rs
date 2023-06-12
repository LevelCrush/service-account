// aliases
use super::definitions::{DestinyRecordDefinitionMap, DestinySeasonDefinitionMap};
use crate::bungie::definitions::{
    DestinyActivityDefinitionMap, DestinyActivityTypeDefinitionMap, DestinyClassDefinitionMap, DestinyDefinition,
};
use crate::bungie::BungieClient;
use levelcrush::tracing;
use std::collections::HashMap;

/// A limited representation of the destiny manifest endpoint.
/// We only include the fields that we want to actually use in our application
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct DestinyManifest {
    pub version: String,

    #[serde(rename = "jsonWorldComponentContentPaths")]
    pub json_world_component_content_paths: HashMap<String, HashMap<String, String>>,

    #[serde(default = "default_locale")]
    locale: String,
}

fn default_locale() -> String {
    "en".to_string()
}

impl DestinyManifest {
    /// fetches a new destiny manifest from the bungie endpoint, requires an active connection
    pub async fn get(client: BungieClient) -> Option<DestinyManifest> {
        let request = client.get("/Destiny2/Manifest").send::<DestinyManifest>().await;

        request.response
    }

    /// generic function to get the definitions based off the content urls provided by the destiny manifest
    async fn get_definition<T: serde::de::DeserializeOwned + Default>(
        &self,
        definition: DestinyDefinition,
        bungie: BungieClient,
    ) -> T {
        let definition: &str = definition.into();
        tracing::info!("Getting Definition: {}", definition);
        let default_string = String::new();
        let default_hashmap = HashMap::<String, String>::new();

        let class_content_path = self
            .json_world_component_content_paths
            .get(self.locale.as_str())
            .unwrap_or(&default_hashmap)
            .get(definition)
            .unwrap_or(&default_string);

        tracing::info!("Content Path: {}", class_content_path);

        let mut definitions = None;
        if !class_content_path.is_empty() {
            let endpoint = format!("https://bungie.net{}", class_content_path);
            tracing::info!("Content Endpoint: {}", endpoint);

            let response = bungie.http_client.get(endpoint).send().await;
            if response.is_ok() {
                tracing::info!("Parsing!");
                let json = response.unwrap().json::<T>().await;
                if json.is_ok() {
                    tracing::info!("Parsing was ok!");
                    definitions = json.ok();
                } else {
                    let err = json.err().unwrap();
                    tracing::error!("{}", err);
                }
            }
        }

        definitions.unwrap_or_default()
    }

    pub async fn get_class_definition(&self, bungie: BungieClient) -> DestinyClassDefinitionMap {
        self.get_definition::<DestinyClassDefinitionMap>(DestinyDefinition::Class, bungie)
            .await
    }

    pub async fn get_season_definitions(&self, bungie: BungieClient) -> DestinySeasonDefinitionMap {
        self.get_definition::<DestinySeasonDefinitionMap>(DestinyDefinition::Seasons, bungie)
            .await
    }

    pub async fn get_record_definition(&self, bungie: BungieClient) -> DestinyRecordDefinitionMap {
        self.get_definition::<DestinyRecordDefinitionMap>(DestinyDefinition::Records, bungie)
            .await
    }

    pub async fn get_activity_definitions(&self, bungie: BungieClient) -> DestinyActivityDefinitionMap {
        self.get_definition::<DestinyActivityDefinitionMap>(DestinyDefinition::Activity, bungie)
            .await
    }

    pub async fn get_activity_type_definitions(&self, bungie: BungieClient) -> DestinyActivityTypeDefinitionMap {
        tracing::info!("Calling  definition fetch, activity type");

        self.get_definition::<DestinyActivityTypeDefinitionMap>(DestinyDefinition::ActivityType, bungie)
            .await
    }
}
