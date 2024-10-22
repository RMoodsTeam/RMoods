use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Represents a time period for sorting posts in a feed.
///
/// Used in [FeedSorting]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
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
