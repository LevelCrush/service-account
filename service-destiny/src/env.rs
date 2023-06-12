/// Set of values that can be found in the .env file
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
