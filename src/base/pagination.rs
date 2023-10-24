pub struct Pagination {
    pub pg_num: i32,
    pub pg_size: i32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            pg_num: 0,
            pg_size: 20,
        }
    }
}

impl Pagination {
    pub fn new(pg_num: Option<i32>, pg_size: Option<i32>) -> Self {
        let default = Pagination::default();
        Self {
            pg_num: pg_num.unwrap_or(default.pg_num),
            pg_size: pg_size.unwrap_or(default.pg_size),
        }
    }
}
