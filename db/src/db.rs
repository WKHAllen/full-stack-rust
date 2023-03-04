use anyhow::Result;
use sqlx::sqlite::SqlitePool;

pub struct DB {
    pub(crate) pool: SqlitePool,
}

impl DB {
    pub async fn open(name: &str) -> Result<Self> {
        let root_path = project_root::get_project_root()?;
        let conn_str = format!("sqlite:{}/assets/{}.db", root_path.display(), name);
        let pool = SqlitePool::connect(&conn_str).await?;

        Ok(Self { pool })
    }
}
