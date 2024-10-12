use derive_getters::Getters;
use serde::{Deserialize, Serialize};

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

impl ToString for FeedSortingTime {
    fn to_string(&self) -> String {
        match self {
            FeedSortingTime::Hour => "hour",
            FeedSortingTime::Day => "day",
            FeedSortingTime::Week => "week",
            FeedSortingTime::Month => "month",
            FeedSortingTime::Year => "year",
            FeedSortingTime::All => "all",
        }
        .to_string()
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

impl ToString for FeedSorting {
    fn to_string(&self) -> String {
        match self {
            FeedSorting::Hot => "hot",
            FeedSorting::New => "new",
            FeedSorting::Rising => "rising",
            FeedSorting::Top(_) => "top",
            FeedSorting::Controversial(_) => "controversial",
        }
        .to_string()
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

// /// Represents the number of Reddit API requests that should be done to fullfill the report request.
// ///
// /// Each Reddit API request is done with the maximum per-request limit of 100 posts.
// #[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
// pub enum RequestSize {
//     #[default]
//     Small,
//     Medium,
//     Large,
//     Custom(u16),
// }

// impl From<RequestSize> for u16 {
//     fn from(value: RequestSize) -> Self {
//         match value {
//             RequestSize::Small => 50,
//             RequestSize::Medium => 250,
//             RequestSize::Large => 500,
//             RequestSize::Custom(n) => n,
//         }
//     }
// }

// impl ToString for RequestSize {
//     fn to_string(&self) -> String {
//         let n = u16::from(*self);
//         n.to_string()
//     }
// }

// /// Represents the parameters for fetching a feed of posts.
// #[derive(Debug, Serialize, Deserialize, Getters, Default)]
// pub struct FeedRequestParams {
//     /// Number of requests to use to fetch the feed
//     #[serde(flatten)]
//     pub size: RequestSize,
//     /// Sorting method for posts
//     #[serde(flatten)]
//     pub sorting: FeedSorting,
// }

// impl FeedRequestParams {
//     pub fn into_http_request_part(&self) -> Vec<(String, String)> {

//     }
// }
