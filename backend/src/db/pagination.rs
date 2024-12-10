/// Pagination struct for database queries that would return too much data otherwise.
pub struct DbPagination {
    page: u32,
    per_page: u32,
}

impl DbPagination {
    /// Indexed from 1, so the first page is denoted by "1".
    ///
    /// Returned as a tuple of [i64] due to SQLx's query! macro requiring i64 for LIMIT and OFFSET.
    pub fn into_limit_and_offset(self) -> (i64, i64) {
        let offset = (self.page - 1) * self.per_page;
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
