use levelcrush::macros::ExternalAPIResponse;

#[ExternalAPIResponse]
pub struct DiscordValidationResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[ExternalAPIResponse]
pub struct DiscordUserResponse {
    pub id: Option<String>,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub global_name: Option<String>,
    pub display_name: Option<String>,
}

#[derive(serde::Serialize, ts_rs::TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-accounts/")]
pub struct LinkGeneratedResponse {
    pub code: String,
}
