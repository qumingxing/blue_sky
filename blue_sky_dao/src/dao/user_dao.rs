use std::io::{Error, ErrorKind};
use mysql::{params};
use mysql::prelude::{Queryable};
use serde_json::json;
use blue_sky_entity::dto::user::{User};
use crate::dao::jdbc_template::{BaseDao, JdbcDataSource};
use crate::dao::mysql_datasource::MySQL;
use crate::error::database_error::DatabaseErrorType;

pub struct UserDao<'a> {
    pub mysql: &'a MySQL,
}

impl BaseDao<User> for UserDao<'_> {
    fn select_list(&self, dto: Option<&User>) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let mut sql: String = String::from("select user_id,user_name,password,hobby,age,created_at,updated_at from t_user");
        if let Some(t) = dto {
            if !t.user_name.is_empty() {
                sql = format!("{} where user_name = '{}'", sql, t.user_name);
            }
        }
        let result: Vec<User> = self.mysql.get_conn().query_map(sql,
                                                                |(user_id, user_name, password, hobby, age, created_at, updated_at)
                                                                 : (String, String, String, Option<String>, Option<u32>, i64, i64)| {
                                                                    let hobby = match hobby {
                                                                        Some(v) => serde_json::from_str(&v).unwrap(),
                                                                        None => Vec::new()
                                                                    };

                                                                    User {
                                                                        user_id,
                                                                        user_name,
                                                                        password,
                                                                        hobby,
                                                                        age: age.unwrap_or_default(),
                                                                        created_at,
                                                                        updated_at,
                                                                        ..Default::default()
                                                                    }
                                                                })
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        Ok(result)
    }

    fn save(&self, dto: &User) -> Result<usize, Box<dyn std::error::Error>> {
        self.mysql.get_conn().exec_drop(
            r#"INSERT INTO t_user
        (user_id, user_name, password,hobby,age,created_at,updated_at)
         VALUES (:user_id, :user_name, :password,:hobby,:age,:created_at,:updated_at)"#,
            params! {"user_id"=>&dto.user_id,"user_name"=>&dto.user_name,
                "password"=>&dto.password,"hobby"=>json!(&dto.hobby),"age"=>&dto.age,"created_at"=>&dto.updated_at,
                "updated_at"=>&dto.created_at})
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        Ok(1)
    }

    fn get(&self, dto: &User) -> Result<User, Box<dyn std::error::Error>> {
        let user = self.mysql.get_conn().exec_first("select user_id,user_name,password,hobby,age,created_at,updated_at from t_user where user_id=:user_id", params! {"user_id"=>&dto.user_id})
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        match user {
            Some(r) => Ok(r),
            None => Err(Box::new(Error::new(ErrorKind::NotFound, "users not found"))),
        }
    }

    fn update(&self, dto: &User) -> Result<usize, Box<dyn std::error::Error>> {
        let mut query = "UPDATE t_user SET".to_string();
        let mut updates = Vec::new();

        if dto.age > 0 {
            updates.push(format!(" age = '{}'", dto.age));
        }
        if !dto.hobby.is_empty() {
            updates.push(format!(" hobby = '{}'", json!(&dto.hobby)));
        }
        if !dto.password.is_empty() {
            updates.push(format!(" password = {}", dto.password));
        }

        if updates.is_empty() {
            return Ok(1);
        }

        query.push_str(&updates.join(", "));
        query.push_str(&format!(" WHERE user_id = '{}'", dto.user_id));

        self.mysql.get_conn().query_drop(query)
            .map_err(|e| Box::new(DatabaseErrorType::DbOperatorErr(format!("用户数据更新失败:{}", e))) as Box<dyn std::error::Error>)?;
        Ok(1)
    }

    fn delete(&self, dto: &User) -> Result<usize, Box<dyn std::error::Error>> {
        if dto.user_id.is_empty() {
            return Err(Box::new(Error::new(ErrorKind::NotFound, "users not found")));
        }

        self.mysql.get_conn().exec_drop("DELETE FROM t_user where user_id=:user_id", params! {"user_id"=>&dto.user_id})
            .map_err(|e| Box::new(DatabaseErrorType::DbOperatorErr(format!("用户数据删除失败:{}", e.to_string()))) as Box<dyn std::error::Error>)?;
        Ok(1)
    }
}