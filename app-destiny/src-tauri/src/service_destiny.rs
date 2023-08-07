pub mod clan;
pub mod member;
pub mod network;
pub mod settings;
pub mod search;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReportQueries {
    pub modes: Option<String>,
    pub timestamp_start: Option<String>,
    pub timestamp_end: Option<String>,
    pub season: Option<String>,
}

const PAGINATION_LIMITS_MAX: i32 = 100;
const PAGINATION_LIMITS_MIN: i32 = 10;
const PAGINATION_LIMITS_DEFAULT: i32 = 10;

#[derive(serde::Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

impl PaginationQuery {
    /// converts the page from 1 based to zero based like how we want to feed into our query
    /// also is a guarantee  to get a 0 or higher u32 out since it is optional
    pub fn get_page(&self) -> u32 {
        match self.page {
            Some(page) => (page - 1).clamp(0, i32::MAX) as u32,
            _ => 0,
        }
    }

    ///  gets the limit from the pagination query, and makes sure it is within a specific limit   
    ///  also is a guarantee  to get a 0 or higher u32 out since it is optional
    pub fn get_limit(&self) -> u32 {
        match self.limit {
            Some(limit) => limit.clamp(PAGINATION_LIMITS_MIN, PAGINATION_LIMITS_MAX) as u32,
            _ => PAGINATION_LIMITS_DEFAULT as u32,
        }
    }
}
