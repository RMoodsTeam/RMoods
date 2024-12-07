/// Pagination struct for database queries that would return too much data otherwise.
pub struct DbPagination {
    page: u32,
    per_page: u32,
}
