use blue_sky_dao::config::get_db_config;
use blue_sky_dao::dao::jdbc_template::BaseDao;
use blue_sky_dao::dao::mysql_datasource::MySQL;
use blue_sky_dao::dao::user_dao::UserDao;
use blue_sky_dao::error::database_error::DatabaseErrorType;
use blue_sky_entity::dto::user::{Hobby, User};
use chrono::Utc;
use lazy_static::lazy_static;
use log::info;
use uuid::Uuid;

#[test]
fn get_user_by_id() {
    let search_user = &User {
        user_id: String::from("911be6e3-40c7-4373-a771-071cddb9960e"),
        ..Default::default()
    };
    let mysql = &MySQL::initialize(get_db_config().db_url.as_str());
    let user_dao = UserDao { mysql };
    let rs = user_dao.get(search_user);
    if let Ok(user) = rs {
        assert_eq!(search_user.user_id, user.user_id);
    }
}

#[test]
fn select_user_list_without_params() {
    let mysql = &MySQL::initialize(get_db_config().db_url.as_str());
    let user_dao = UserDao { mysql };
    let rs = user_dao.select_list(None);
    if let Ok(users) = rs {
        assert!(users.len() > 0, "users should not be empty");
    }
}

#[test]
fn select_user_list_with_params() {
    let user_name = String::from("zhangsan");
    let user = &User {
        user_name,
        ..Default::default()
    };
    let mysql = &MySQL::initialize(get_db_config().db_url.as_str());
    let user_dao = UserDao { mysql };
    let rs = user_dao.select_list(Some(user));
    if let Ok(users) = rs {
        assert_eq!(users.len(), 1, "users should not be empty");
    }
}
#[test]
fn save_user() {
    let ts = Utc::now().timestamp_millis();
    let user = &User {
        user_id: Uuid::new_v4().to_string(),
        user_name: String::from("zhangsan"),
        password: String::from("123456"),
        hobby: vec![Hobby::DANCING],
        age: 18,
        created_at: ts,
        updated_at: ts,
        ..Default::default()
    };
    let mysql = &MySQL::initialize(get_db_config().db_url.as_str());
    let user_dao = UserDao { mysql };
    let rs = user_dao.save(user);
    if let Ok(count) = rs {
        assert_eq!(count, 1, "user saved 1 time");
    }
}

#[test]
fn update_user() {
    let ts = Utc::now().timestamp_millis();
    let user = &User {
        user_id: String::from("abcd"),
        hobby: vec![Hobby::DANCING, Hobby::JOGGING],
        updated_at: ts,
        ..Default::default()
    };
    let mysql = &MySQL::initialize(get_db_config().db_url.as_str());
    let user_dao = UserDao { mysql };
    let rs = user_dao.update(user);
    if let Ok(count) = rs {
        assert_eq!(count, 1, "user updated 1 time");
    }
}
#[test]
fn delete_user() {
    info!("开始删除用户数据..");
    let user = &User {
        user_id: String::from("911be6e3-40c7-4373-a771-071cddb9960e"),
        ..Default::default()
    };
    let mysql = &MySQL::initialize(get_db_config().db_url.as_str());
    let user_dao = UserDao { mysql };
    let rs = user_dao.delete(user);
    match rs {
        Ok(count) => {
            assert_eq!(count, 1, "user deleted 1 time");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
