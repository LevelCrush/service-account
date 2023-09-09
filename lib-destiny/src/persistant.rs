use levelcrush::{anyhow, tokio::fs};

#[derive(Default, Debug, Clone)]
pub struct PersistantCache<T>
where
    T: serde::de::DeserializeOwned,
{
    pub data: T,

    load_path: String,
}

impl<T> PersistantCache<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Default,
{
    /// Load a json file from local storage from the path specified and parse into the desired type
    pub async fn load<P>(path: P) -> anyhow::Result<PersistantCache<T>>
    where
        P: Into<String>,
    {
        let path = path.into();
        let load_path = path.clone();

        let file_exists = fs::try_exists(load_path.clone()).await?;
        if file_exists {
            let file_results = fs::read_to_string(path).await?;
            let data = serde_json::from_str::<'_, T>(&file_results)?;

            Ok(PersistantCache { data, load_path })
        } else {
            Ok(PersistantCache {
                data: T::default(),
                load_path,
            })
        }
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// write  to the original load location the current data in the persistant cache
    pub async fn save(&self) -> anyhow::Result<()> {
        let target_path = self.load_path.clone();
        let serialized_data = serde_json::to_string::<T>(&self.data)?;
        fs::write(target_path, &serialized_data).await?;
        Ok(())
    }
}
