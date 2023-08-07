pub mod network;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReportQueries {
    pub modes: Option<String>,
    pub timestamp_start: Option<String>,
    pub timestamp_end: Option<String>,
    pub season: Option<String>,
}
