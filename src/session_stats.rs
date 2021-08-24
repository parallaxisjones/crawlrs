use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct SessionStats {
    total_visited: u64,
    #[serde(default)]
    #[serde(with = "date_serde")]
    started_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(with = "date_serde")]
    finished_at: Option<DateTime<Utc>>,
}

impl SessionStats {
    pub fn new() -> SessionStats {
        SessionStats {
            total_visited: 0,
            started_at: None,
            finished_at: None,
        }
    }

    pub fn add_visit(&mut self, count: Option<u64>) {
        match count {
            Some(visited) => self.total_visited += visited as u64,
            None => self.total_visited += 1,
        };
    }
    pub fn start_session(&mut self) {
        self.started_at = Some(Utc::now());
    }
    pub fn finish_session(&mut self) {
        self.finished_at = Some(Utc::now());
    }
    pub fn elapsed_time(&self) -> Duration {
        match (self.started_at, self.finished_at) {
            (Some(started), Some(finished)) => finished - started,
            (Some(started), None) => Utc::now() - started,
            _ => panic!("Session has not been started or finished"),
        }
    }
    #[allow(dead_code)]
    pub fn print_stats(&self) {
        println!(
            "Start: {}\nEnd: {}\nTotal visited: {}\nElapsed Time: {} sec",
            self.started_at.unwrap(),
            self.finished_at.unwrap(),
            self.total_visited,
            self.elapsed_time().num_seconds()
        );
    }
}

mod date_serde {
    use chrono::NaiveDateTime;
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(ref d) = *date {
            return s.serialize_str(&d.format("%Y-%m-%d").to_string());
        }
        s.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            return Ok(Some(DateTime::<Utc>::from_utc(
                NaiveDateTime::parse_from_str(&s, "%Y-%m-%d").unwrap(),
                Utc,
            )));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    #[test]
    fn test_start_session() {
        let mut session_stats = SessionStats::new();
        session_stats.start_session();
        assert!(session_stats.started_at.unwrap() < Utc::now());
    }

    #[test]
    fn test_finish_session() {
        let mut session_stats = SessionStats::new();
        session_stats.finish_session();
        assert!(session_stats.finished_at.unwrap() < Utc::now());
    }

    #[test]
    fn test_add_single_visit() {
        let mut session_stats = SessionStats::new();
        session_stats.add_visit(None);
        assert_eq!(session_stats.total_visited, 1);
    }

    #[test]
    fn test_add_visit_count() {
        let mut session_stats = SessionStats::new();
        session_stats.add_visit(Some(5));
        assert_eq!(session_stats.total_visited, 5);
    }

    #[test]
    fn test_elapsed_time() {
        const SLEEP_TIME: i64 = 10;
        let ten_millis = time::Duration::from_millis(SLEEP_TIME as u64);
        let mut session_stats = SessionStats::new();
        session_stats.start_session();
        thread::sleep(ten_millis);
        session_stats.finish_session();
        assert!(session_stats.elapsed_time().num_milliseconds() == SLEEP_TIME);
    }
}
