pub async fn connect_database() -> anyhow::Result<sqlx::SqlitePool> {
    use sqlx::sqlite::SqlitePoolOptions;

    let pool =
        SqlitePoolOptions::new().max_connections(5).connect("sqlite:palnel.db?mode=rwc").await?;

    sqlx::migrate!("../migrations").run(&pool).await?;

    Ok(pool)
}
