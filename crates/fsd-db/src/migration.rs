// Embedded SQL migration runners for fsd-db (desktop + shared).

use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};

use crate::DbError;

// ── Desktop migrations ────────────────────────────────────────────────────────

const DESKTOP_MIGRATIONS: &[(&str, &str)] = &[(
    "desktop_001_initial",
    include_str!("../migrations/desktop/001_initial.sql"),
)];

// ── Shared migrations ─────────────────────────────────────────────────────────

const SHARED_MIGRATIONS: &[(&str, &str)] = &[(
    "shared_001_initial",
    include_str!("../migrations/shared/001_initial.sql"),
)];

// ── Migrator ──────────────────────────────────────────────────────────────────

pub struct DesktopMigrator;
pub struct SharedMigrator;

impl DesktopMigrator {
    pub async fn run(db: &DatabaseConnection) -> Result<(), DbError> {
        run_migrations(db, DESKTOP_MIGRATIONS).await
    }
}

impl SharedMigrator {
    pub async fn run(db: &DatabaseConnection) -> Result<(), DbError> {
        run_migrations(db, SHARED_MIGRATIONS).await
    }
}

// ── private ───────────────────────────────────────────────────────────────────

async fn run_migrations(
    db: &DatabaseConnection,
    migrations: &[(&str, &str)],
) -> Result<(), DbError> {
    ensure_tracking_table(db).await?;
    for (name, sql) in migrations {
        if is_applied(db, name).await? {
            continue;
        }
        apply(db, name, sql).await?;
    }
    Ok(())
}

async fn ensure_tracking_table(db: &DatabaseConnection) -> Result<(), DbError> {
    let sql = "CREATE TABLE IF NOT EXISTS _migrations (\
        name TEXT PRIMARY KEY, \
        applied_at INTEGER NOT NULL DEFAULT (strftime('%s','now'))\
    )";
    exec(db, sql).await
}

async fn is_applied(db: &DatabaseConnection, name: &str) -> Result<bool, DbError> {
    let sql = format!("SELECT COUNT(*) FROM _migrations WHERE name = '{name}'");
    let result = db
        .query_one(Statement::from_string(db.get_database_backend(), sql))
        .await
        .map_err(|e| DbError::SeaOrm(e.to_string()))?;
    Ok(result
        .map(|row| row.try_get::<i64>("", "COUNT(*)").unwrap_or(0) > 0)
        .unwrap_or(false))
}

async fn apply(db: &DatabaseConnection, name: &str, sql: &str) -> Result<(), DbError> {
    for stmt in sql.split(';').map(str::trim).filter(|s| !s.is_empty()) {
        exec(db, stmt).await?;
    }
    let record = format!("INSERT INTO _migrations (name) VALUES ('{name}')");
    exec(db, &record).await
}

async fn exec(db: &DatabaseConnection, sql: &str) -> Result<(), DbError> {
    db.execute(Statement::from_string(
        db.get_database_backend(),
        sql.to_string(),
    ))
    .await
    .map(|_| ())
    .map_err(|e| DbError::SeaOrm(e.to_string()))
}
