pub use sea_orm_migration::prelude::*;

mod m20250101_000000_phase0_init;
mod m20260118_000000_add_charge_point_password;
mod m20260118_000001_add_operators;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000000_phase0_init::Migration),
            Box::new(m20260118_000000_add_charge_point_password::Migration),
            Box::new(m20260118_000001_add_operators::Migration),
        ]
    }
}
