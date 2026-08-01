pub mod db;

use sea_orm::DatabaseConnection;

pub type DbPool = DatabaseConnection;
