use google_sheets4::api::{BatchClearValuesRequest, BatchUpdateValuesRequest, ValueRange};
use google_sheets4::hyper::Client;
use google_sheets4::oauth2::{self};
use google_sheets4::{hyper, hyper_rustls, Sheets};
use google_sheets4::{
    hyper::client::HttpConnector, hyper_rustls::HttpsConnector, oauth2::authenticator::Authenticator,
};
use levelcrush::{anyhow, project_str, tracing};
use lib_destiny::env::{AppVariable, Env};

use crate::sheets::MasterWorkbook;

pub async fn sync(env: &Env) -> anyhow::Result<()> {
    tracing::info!("Constructing workbook connection");
    let sheet_id = env.get(AppVariable::MasterWorkSheet);
    let mut workbook = MasterWorkbook::get(&sheet_id).await?;

    tracing::info!("Hydrating information");
    workbook.load().await?;

    tracing::info!("Updating from API");
    workbook.api_sync(env).await?;

    tracing::info!("Saving workbook");
    workbook.save().await?;

    Ok(())
}

pub async fn discord_sync(env: &Env) -> anyhow::Result<()> {
    tracing::info!("Constructing workbook connection");
    let sheet_id = env.get(AppVariable::MasterWorkSheet);
    let mut workbook = MasterWorkbook::get(&sheet_id).await?;

    tracing::info!("Hydrating information");
    workbook.load().await?;

    // shadow into a non mutable state now that we have loaded
    let workbook = workbook;

    // construct discord bot to connect to any servers

    // loop throuhg clan sheets and match to player and pull get discord id if possible
    // construct tuple of (discord_id, clan_role)

    // loop throgugh vector of tuples and assign roles via bot

    // done!

    Ok(())
}
