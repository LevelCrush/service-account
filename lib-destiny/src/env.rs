/// Set of values that can be found in the .env file
#[derive(Clone, Debug)]
pub enum AppVariable {
    // server variables
    ServerUrl,
    ServerPort,
    ServerSecret,

    // database url
    DatabaseUrl,

    // http client settings
    DangerAppInvalidCerts,

    // bungie api key
    BungieAPIKey,
    CrawlWorkers,
    PriorityTaskWorkers,
    Network,
    MasterWorkSheet,

    DiscordBotToken,
    DiscordClientId,
    DiscordClientSecret,
}

impl From<AppVariable> for &'static str {
    fn from(app_var: AppVariable) -> Self {
        match app_var {
            AppVariable::ServerUrl => "SERVER_URL",
            AppVariable::ServerPort => "SERVER_PORT",
            AppVariable::ServerSecret => "SERVER_SECRET",
            AppVariable::DatabaseUrl => "DATABASE_URL",
            AppVariable::DangerAppInvalidCerts => "DANGER_APP_INVALID_CERTS",
            AppVariable::BungieAPIKey => "BUNGIE_API_KEY",
            AppVariable::CrawlWorkers => "CRAWL_WORKERS",
            AppVariable::PriorityTaskWorkers => "PRIORITY_TASK_WORKERS",
            AppVariable::Network => "CLAN_NETWORK",
            AppVariable::MasterWorkSheet => "MASTER_WORKSHEET",
            AppVariable::DiscordBotToken => "DISCORD_BOT_TOKEN",
            AppVariable::DiscordClientId => "DISCORD_CLIENT_ID",
            AppVariable::DiscordClientSecret => "DISCORD_CLIENT_SECRET",
        }
    }
}

/// fetches a application variable from the .env file or targeted system environment variables
pub fn get(app_var: AppVariable) -> String {
    std::env::var::<&'static str>(app_var.into()).unwrap_or_default()
}

/* commenting for now,  could of sworn i had used this at one point but cant remember where or why
pub fn exists(app_var: AppVariable) -> bool {
    std::env::var::<&'static str>(app_var.into()).is_ok()
}
*/
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Env {
    bungie_api_key: String,
    workers: i64,
    network: Vec<i64>,
    master_worksheet: String,
    discord_bot_token: String,
    discord_client_id: String,
    discord_client_secret: String,
}

impl Env {
    // parse the env
    pub fn load(config: &str) -> Env {
        // panic is ok in this case if we dont have a proper config baked in
        serde_json::from_str::<Env>(config).unwrap()
    }

    pub fn get(&self, app_var: AppVariable) -> String {
        std::env::var::<&'static str>(app_var.clone().into()).unwrap_or(match app_var {
            AppVariable::BungieAPIKey => self.bungie_api_key.clone(),
            AppVariable::CrawlWorkers => format!("{}", self.workers),
            AppVariable::PriorityTaskWorkers => format!("{}", self.workers),
            AppVariable::Network => self
                .network
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(","),
            AppVariable::MasterWorkSheet => self.master_worksheet.clone(),
            AppVariable::DiscordBotToken => self.discord_bot_token.clone(),
            AppVariable::DiscordClientId => self.discord_client_id.clone(),
            AppVariable::DiscordClientSecret => self.discord_client_secret.clone(),
            default => "".to_string(),
        })
    }

    pub fn get_array(&self, app_var: AppVariable) -> Vec<String> {
        let env_override = std::env::var::<&'static str>(app_var.clone().into());
        if let Ok(override_v) = env_override {
            vec![override_v]
        } else {
            match app_var {
                AppVariable::BungieAPIKey => vec![format!("{}", self.bungie_api_key)],
                AppVariable::CrawlWorkers => vec![format!("{}", self.workers)],
                AppVariable::PriorityTaskWorkers => vec![format!("{}", self.workers)],
                AppVariable::Network => self.network.iter().map(|v| v.to_string()).collect::<Vec<String>>(),
                AppVariable::MasterWorkSheet => vec![self.master_worksheet.clone()],
                AppVariable::DiscordBotToken => vec![self.discord_bot_token.clone()],
                AppVariable::DiscordClientId => vec![self.discord_client_id.clone()],
                AppVariable::DiscordClientSecret => vec![self.discord_client_secret.clone()],
                default => vec!["".to_string()],
            }
        }
    }
}
