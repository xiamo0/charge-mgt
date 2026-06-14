use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250101_000000_phase0_init"
    }
}

const INIT_SQL: &str = include_str!("m20250101_000000_phase0_init.sql");

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        for stmt in split_sql_statements(INIT_SQL) {
            db.execute_unprepared(&stmt).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP TABLE IF EXISTS charge_mgt_sent_messages_ocpp_1_6 CASCADE")
            .await?;
        db.execute_unprepared("DROP TABLE IF EXISTS charge_mgt_connectors_ocpp_1_6 CASCADE")
            .await?;
        db.execute_unprepared("DROP TABLE IF EXISTS charge_mgt_charge_points_ocpp_1_6 CASCADE")
            .await?;
        db.execute_unprepared("DROP TYPE IF EXISTS charge_mgt_connector_status")
            .await?;
        Ok(())
    }
}

fn split_sql_statements(sql: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut in_string = false;
    let mut string_char = '"';
    let mut in_single_line_comment = false;
    let mut chars = sql.chars().peekable();
    while let Some(ch) = chars.next() {
        if in_single_line_comment {
            if ch == '\n' {
                in_single_line_comment = false;
                buf.push(ch);
            }
            continue;
        }
        if in_string {
            buf.push(ch);
            if ch == string_char {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' | '\'' => {
                in_string = true;
                string_char = ch;
                buf.push(ch);
            }
            '-' if chars.peek() == Some(&'-') => {
                in_single_line_comment = true;
            }
            ';' => {
                let trimmed = buf.trim().to_string();
                if !trimmed.is_empty() {
                    out.push(trimmed);
                }
                buf.clear();
            }
            _ => buf.push(ch),
        }
    }
    let trimmed = buf.trim().to_string();
    if !trimmed.is_empty() {
        out.push(trimmed);
    }
    out
}
