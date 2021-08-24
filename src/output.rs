use crate::session_stats::SessionStats;

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawlOutput {
    pub seeds: Vec<String>,
    pub links: Vec<String>,
    pub stats: Option<SessionStats>,
}
