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
    Ok(())
}
