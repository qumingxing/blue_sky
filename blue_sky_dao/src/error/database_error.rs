use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseErrorType {
    DbOperatorErr(String),
}

impl fmt::Display for DatabaseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let DatabaseErrorType::DbOperatorErr(err) = self {
            write!(f, "Occurs Error: {}", err)
        } else {
            unreachable!()
        }
    }
}