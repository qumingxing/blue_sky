use std::io::Error;
pub trait JdbcDataSource<T> {
    fn get_conn(&self) -> T;
}
pub trait BaseDao<T> {
    fn select_list(&self, t: Option<&T>) -> Result<Vec<T>, Box<dyn std::error::Error>>;
    fn save(&self, dto: &T) -> Result<usize, Box<dyn std::error::Error>>;
    fn get(&self, dto: &T) -> Result<T, Box<dyn std::error::Error>>;
    fn update(&self, dto: &T) -> Result<usize, Box<dyn std::error::Error>>;
    fn delete(&self, dto: &T) -> Result<usize, Box<dyn std::error::Error>>;
}