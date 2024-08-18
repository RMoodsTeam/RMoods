use std::{default, fmt::Display};

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FeedSorting {
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
    pub fn time(&self) -> Option<FeedSortingTime> {
        match self {
            FeedSorting::Top(time) | FeedSorting::Controversial(time) => Some(*time),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum RequestSize {
    #[default]
    Small,
    Medium,
    Large,
    Custom(u16),
}

impl From<RequestSize> for u16 {
    fn from(value: RequestSize) -> Self {
        match value {
            RequestSize::Small => 50,
            RequestSize::Medium => 250,
            RequestSize::Large => 100,
            RequestSize::Custom(n) => n,
        }
    }
}

impl ToString for RequestSize {
    fn to_string(&self) -> String {
        let n = u16::from(*self);
        n.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct FeedRequestParams {
    #[serde(flatten)]
    pub size: RequestSize,
    #[serde(flatten)]
    pub sorting: FeedSorting,
}

impl default::Default for FeedRequestParams {
    fn default() -> Self {
        FeedRequestParams {
            size: RequestSize::Small,
            sorting: FeedSorting::Hot,
        }
    }
}
