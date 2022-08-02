use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Executor;
use sqlx::Row;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool: SqlitePool = SqlitePoolOptions::new()
        .connect("sqlite://foobar.db?mode=rwc")
        .await?;

    let create_query = format!("CREATE TABLE IF NOT EXISTS foo (bar REAL(4, 2))");
    let create_query = sqlx::query(&create_query);
    let res = pool.execute(create_query).await?;

    let v: f64 = 5.0;
    let insert_query = format!("INSERT INTO foo(bar) VALUES ({}) RETURNING bar", v);
    let insert_query = sqlx::query(&insert_query);
    let row = pool.fetch_one(insert_query).await?;
    assert_eq!(dbg!(row.get::<f64, _>(0)), v);

    Ok(())
}
