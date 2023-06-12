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
}

impl From<AppVariable> for &'static str {
    fn from(app_var: AppVariable) -> Self {
        match app_var {
            AppVariable::ServerUrl => "SERVER_URL",
            AppVariable::ServerPort => "SERVER_PORT",
            AppVariable::ServerSecret => "SERVER_SECRET",
            AppVariable::DatabaseUrl => "DATABASE_URL",
            AppVariable::DangerAppInvalidCerts => "DANGER_APP_INVALID_CERTS",
        }
    }
}

/// fetches a application variable from the .env file or targeted system environment variables
pub fn get(app_var: AppVariable) -> String {
    std::env::var::<&'static str>(app_var.into()).unwrap_or_default()
}

pub fn exists(app_var: AppVariable) -> bool {
    std::env::var::<&'static str>(app_var.into()).is_ok()
}
