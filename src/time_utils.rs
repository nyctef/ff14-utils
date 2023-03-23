use chrono::{DateTime, Duration, Utc};

pub fn hm_ago(dur: Duration) -> String {
    // xxm ago for durations < 1h, yyhxxm ago for durations >= 1h
    if dur.num_minutes() < 60 {
        format!("{}m ago", dur.num_minutes())
    } else {
        format!("{}h{}m ago", dur.num_hours(), dur.num_minutes() % 60)
    }
}

pub fn hm_ago_from_now(t: DateTime<Utc>) -> String {
    hm_ago(Utc::now() - t)
}
