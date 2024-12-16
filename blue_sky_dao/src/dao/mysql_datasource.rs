use mysql::{Pool, PooledConn};
use crate::dao::jdbc_template::JdbcDataSource;

pub struct MySQLDataSource;
pub struct MySQL {
    pool: Pool,
}
impl MySQL {
    pub fn initialize(jdbc_url: &str) -> MySQL {
        let conn_pool = Pool::new(jdbc_url).expect("Failed to create pool.");
        MySQL { pool: conn_pool }
    }
}
impl JdbcDataSource<PooledConn> for MySQL {
    fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().expect("Failed to get a connection from the pool.")
    }
}