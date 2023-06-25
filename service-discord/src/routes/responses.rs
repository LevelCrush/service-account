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
    pub avatar: String,
    pub global_name: Option<String>,
    pub display_name: Option<String>,
}
