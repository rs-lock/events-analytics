use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TopProductsQuery {
    pub period: Option<String>,
    pub limit: Option<u16>,
    pub metric: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserActivityQuery {
    pub from: Option<String>,
    pub to: Option<String>,
}
