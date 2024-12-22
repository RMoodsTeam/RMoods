use serde::Deserialize;
use serde_with::serde_as;

/// Pagination struct for database queries that would return too much data otherwise.
#[serde_as]
// `serde_as` is used as a workaround for a known `serde` bug: https://github.com/serde-rs/serde/issues/1183
#[derive(Clone, Debug, Deserialize)]
pub struct DbPagination {
    /// The page number to fetch. Indexed from 1.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    page: u32,
    /// The number of items per page. 30 by default.
    #[serde_as(as = "serde_with::DisplayFromStr")]
    per_page: u32,
}

impl DbPagination {
    /// Create a new pagination struct with the provided page and per_page values.
    pub fn new(page: u32, per_page: u32) -> Self {
        Self { page, per_page }
    }

    /// Indexed from 1, so the first page is denoted by "1".
    ///
    /// Returned as a tuple of [i64] due to SQLx's query! macro requiring i64 for LIMIT and OFFSET.
    /// If the page is 0, a warning is logged, and it is set to 1.
    pub fn into_limit_and_offset(self) -> (i64, i64) {
        let page_checked = if self.page == 0 {
            log::warn!("Page was passed as 0, setting it to 1");
            1
        } else {
            self.page
        };
        let offset = (page_checked - 1) * self.per_page;
        (self.per_page as i64, offset as i64)
    }
}

impl Default for DbPagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 30,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_limit_offset() {
        let pagination = DbPagination {
            page: 1,
            per_page: 10,
        };
        assert_eq!(pagination.into_limit_and_offset(), (10, 0));

        let pagination = DbPagination {
            page: 2,
            per_page: 10,
        };
        assert_eq!(pagination.into_limit_and_offset(), (10, 10));

        let pagination = DbPagination {
            page: 3,
            per_page: 10,
        };
        assert_eq!(pagination.into_limit_and_offset(), (10, 20));
    }
}
