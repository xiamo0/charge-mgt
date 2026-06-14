pub mod db;
pub mod kafka;

use sea_orm::DatabaseConnection;

pub type DbPool = DatabaseConnection;
