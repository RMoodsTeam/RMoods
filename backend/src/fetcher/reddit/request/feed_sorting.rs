use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Represents a time period for sorting posts in a feed.
///
/// Used in [FeedSorting]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FeedSortingTime {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All,
}

impl Display for FeedSortingTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FeedSortingTime::Hour => "hour",
            FeedSortingTime::Day => "day",
            FeedSortingTime::Week => "week",
            FeedSortingTime::Month => "month",
            FeedSortingTime::Year => "year",
            FeedSortingTime::All => "all",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

/// Represents a sorting method for posts in a feed.
///
/// * `Controversial` and `Top` require a time period as [FeedSortingTime]
/// * `Hot`, `New`, and `Rising` do not require a time period
/// *  Used in [FeedRequestParams]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "kind", content = "time")]
pub enum FeedSorting {
    #[default]
    Hot,
    New,
    Rising,
    Top(FeedSortingTime),
    Controversial(FeedSortingTime),
}

impl Display for FeedSorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FeedSorting::Hot => "hot",
            FeedSorting::New => "new",
            FeedSorting::Rising => "rising",
            FeedSorting::Top(_) => "top",
            FeedSorting::Controversial(_) => "controversial",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

impl FeedSorting {
    /// Get the time period for sorting posts, if applicable.
    pub fn time(&self) -> Option<FeedSortingTime> {
        match self {
            FeedSorting::Top(time) | FeedSorting::Controversial(time) => Some(*time),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(FeedSortingTime::Hour.to_string(), "hour");
        assert_eq!(FeedSortingTime::Day.to_string(), "day");
        assert_eq!(FeedSortingTime::Week.to_string(), "week");
        assert_eq!(FeedSortingTime::Month.to_string(), "month");
        assert_eq!(FeedSortingTime::Year.to_string(), "year");
        assert_eq!(FeedSortingTime::All.to_string(), "all");

        assert_eq!(FeedSorting::Hot.to_string(), "hot");
        assert_eq!(FeedSorting::New.to_string(), "new");
        assert_eq!(FeedSorting::Rising.to_string(), "rising");
        assert_eq!(FeedSorting::Top(FeedSortingTime::Hour).to_string(), "top");
        assert_eq!(
            FeedSorting::Controversial(FeedSortingTime::Day).to_string(),
            "controversial"
        );
    }

    #[test]
    fn test_time() {
        assert_eq!(FeedSorting::Hot.time(), None);
        assert_eq!(FeedSorting::New.time(), None);
        assert_eq!(FeedSorting::Rising.time(), None);
        assert_eq!(
            FeedSorting::Top(FeedSortingTime::Hour).time(),
            Some(FeedSortingTime::Hour)
        );
        assert_eq!(
            FeedSorting::Controversial(FeedSortingTime::Day).time(),
            Some(FeedSortingTime::Day)
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(FeedSorting::default(), FeedSorting::Hot);
    }

    #[test]
    fn test_deserialization_timed() {
        let json = r#"{"kind": "top", "time": "all"}"#;
        let feed_sorting: FeedSorting = serde_json::from_str(json).unwrap();
        assert_eq!(feed_sorting, FeedSorting::Top(FeedSortingTime::All));
    }

    #[test]
    fn test_deserialization_untimed() {
        let json = r#"{"kind": "hot"}"#;
        let feed_sorting: FeedSorting = serde_json::from_str(json).unwrap();
        assert_eq!(feed_sorting, FeedSorting::Hot);
    }

    #[test]
    #[should_panic]
    fn test_deserialization_untimed_with_time_given() {
        let json = r#"{"kind": "hot", "time": "hour"}"#;
        let feed_sorting: FeedSorting = serde_json::from_str(json).unwrap();
        assert_eq!(feed_sorting, FeedSorting::Hot);
    }
}
