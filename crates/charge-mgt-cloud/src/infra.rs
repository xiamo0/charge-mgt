pub mod db;
pub mod http_client;

use sea_orm::DatabaseConnection;

pub type DbPool = DatabaseConnection;
