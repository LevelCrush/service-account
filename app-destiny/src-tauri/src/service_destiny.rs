pub mod network;
pub mod settings;
pub mod member;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReportQueries {
    pub modes: Option<String>,
    pub timestamp_start: Option<String>,
    pub timestamp_end: Option<String>,
    pub season: Option<String>,
}
