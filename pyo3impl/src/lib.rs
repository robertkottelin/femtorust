use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, Row};
use tokio::runtime::Runtime;

// Function to create a new Tokio runtime
fn get_sync_runtime() -> Runtime {
    tokio::runtime::Runtime::new().unwrap()
}

// Async function to create the database
async fn create_database(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS data (
            id TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(pool)
    .await?;

    Ok(())
}

// Async function to insert data
async fn insert_data(pool: &Pool<Sqlite>, id: &str, value: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO data (id, value) VALUES (?, ?)")
        .bind(id)
        .bind(value)
        .execute(pool)
        .await?;

    Ok(())
}

// Async function to get value by ID
async fn get_value_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT value FROM data WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(row.try_get(0)?)
}

// Convert sqlx::Error to PyResult (PyErr)
fn sqlx_error_to_pyerr(e: sqlx::Error) -> PyErr {
    PyRuntimeError::new_err(format!("Database error: {}", e))
}

// Synchronous wrapper for creating database
#[pyfunction]
fn create_database_sync() -> PyResult<()> {
    let rt = get_sync_runtime();
    rt.block_on(async {
        let database_url = "sqlite:femtorust.db"; // Fixed database URL
        let pool = SqlitePoolOptions::new().connect(database_url).await
            .map_err(sqlx_error_to_pyerr)?;
        create_database(&pool).await
            .map_err(sqlx_error_to_pyerr)
    })
}

// Synchronous wrapper for inserting data
#[pyfunction]
fn insert_data_sync(id: String, value: String) -> PyResult<()> {
    let rt = get_sync_runtime();
    rt.block_on(async {
        let database_url = "sqlite:femtorust.db"; // Fixed database URL
        let pool = SqlitePoolOptions::new().connect(database_url).await
            .map_err(sqlx_error_to_pyerr)?;
        insert_data(&pool, &id, &value).await
            .map_err(sqlx_error_to_pyerr)
    })
}

// Synchronous wrapper for getting value by ID
#[pyfunction]
fn get_value_by_id_sync(id: String) -> PyResult<String> {
    let rt = get_sync_runtime();
    rt.block_on(async {
        let database_url = "sqlite:femtorust.db"; // Fixed database URL
        let pool = SqlitePoolOptions::new().connect(database_url).await
            .map_err(sqlx_error_to_pyerr)?;
        get_value_by_id(&pool, &id).await
            .map_err(sqlx_error_to_pyerr)
    })
}

// PyO3 module definition
#[pymodule]
fn pyo3impl(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_database_sync, m)?)?;
    m.add_function(wrap_pyfunction!(insert_data_sync, m)?)?;
    m.add_function(wrap_pyfunction!(get_value_by_id_sync, m)?)?;
    Ok(())
}
