/// Pagination struct for database queries that would return too much data otherwise.
pub struct DbPagination {
    page: u32,
    per_page: u8,
}

impl DbPagination {
    pub fn new(page: u32, per_page: u8) -> Self {
        Self { page, per_page }
    }

    /// Indexed from 1, so the first page is denoted by "1".
    ///
    /// Returned as a tuple of [i64] due to SQLx's query! macro requiring i64 for LIMIT and OFFSET.
    pub fn into_limit_and_offset(self) -> (i64, i64) {
        let page_checked = if self.page == 0 {
            log::warn!("Page was passed as 0, setting it to 1");
            1
        } else {
            self.page
        };
        let offset = (page_checked - 1) * self.per_page as u32;
        (self.per_page as i64, offset as i64)
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
