#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use anyhow::Result;
use indoc::indoc;
use postgresql_embedded::{PostgreSQL, Settings, VersionReq};
use sqlx::PgPool;
use tracing::info;

/// Example of how to install and configure the vector extension.
///
/// See: <https://github.com/tensorchord/pgvecto.rs/?tab=readme-ov-file#quick-start>
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().compact().init();

    info!("Installing PostgreSQL");
    let settings = Settings {
        version: VersionReq::parse("=16.3.0")?,
        ..Default::default()
    };
    let mut postgresql = PostgreSQL::new(settings);
    postgresql.setup().await?;

    info!("Installing the vector extension from TensorChord");
    postgresql_extensions::install(
        postgresql.settings(),
        "tensor-chord",
        "pgvecto.rs",
        &VersionReq::parse("=0.3.0")?,
    )
    .await?;

    info!("Starting PostgreSQL");
    postgresql.start().await?;

    let database_name = "vector-example";
    info!("Creating database {database_name}");
    postgresql.create_database(database_name).await?;

    info!("Configuring extension");
    let settings = postgresql.settings();
    let database_url = settings.url(database_name);
    let pool = PgPool::connect(database_url.as_str()).await?;
    configure_extension(&pool).await?;
    pool.close().await;

    info!("Restarting database");
    postgresql.stop().await?;
    postgresql.start().await?;

    info!("Enabling extension");
    let pool = PgPool::connect(database_url.as_str()).await?;
    enable_extension(&pool).await?;

    info!("Creating table");
    create_table(&pool).await?;

    info!("Creating data");
    create_data(&pool).await?;

    info!("Stopping database");
    postgresql.stop().await?;
    Ok(())
}

async fn configure_extension(pool: &PgPool) -> Result<()> {
    sqlx::query("ALTER SYSTEM SET shared_preload_libraries = \"vectors.so\"")
        .execute(pool)
        .await?;
    sqlx::query("ALTER SYSTEM SET search_path = \"$user\", public, vectors")
        .execute(pool)
        .await?;
    Ok(())
}

async fn enable_extension(pool: &PgPool) -> Result<()> {
    sqlx::query("DROP EXTENSION IF EXISTS vectors")
        .execute(pool)
        .await?;
    sqlx::query("CREATE EXTENSION vectors")
        .execute(pool)
        .await?;
    Ok(())
}

async fn create_table(pool: &PgPool) -> Result<()> {
    sqlx::query(indoc! {"
        CREATE TABLE IF NOT EXISTS items (
            id bigserial PRIMARY KEY,
            embedding vector(3) NOT NULL
        )
    "})
        .execute(pool)
        .await?;
    Ok(())
}

async fn create_data(pool: &PgPool) -> Result<()> {
    sqlx::query(indoc! {"
        INSERT INTO items (embedding)
        VALUES
            ('[1,2,3]'),
            ('[4,5,6]')
    "})
    .execute(pool)
    .await?;
    sqlx::query(indoc! {"
        INSERT INTO items (embedding)
        VALUES
            (ARRAY[1, 2, 3]::real[]),
            (ARRAY[4, 5, 6]::real[]
        )
    "})
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
