use std::sync::{Arc, Mutex, MutexGuard, Once};
use lazy_static::lazy_static;
use blue_sky_entity::dto::user::User;
use crate::dao::mysql_datasource::MySQL;
use crate::dao::user_dao::UserDao;

pub mod dao;
pub mod error;

lazy_static! {
    static ref  MYSQL_URL: String = String::from("mysql://xx");
    static ref MYSQL: MySQL = MySQL::initialize(MYSQL_URL.as_str());

    static ref USER_DAO: Arc<Mutex<UserDao<'static>>> = {
        let user_dao = UserDao { mysql: &MYSQL };
        Arc::new(Mutex::new(user_dao))
    };

}

pub fn get_user_dao() -> MutexGuard<'static, UserDao<'static>> {
    USER_DAO.lock().unwrap()
}