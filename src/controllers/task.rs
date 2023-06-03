use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use axum::{Extension, Json};
use serde_json::{json, Value};
use sqlx::MySqlPool;

use crate::{errors::CustomError, models::task};

pub async fn find_all(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let sql = "SELECT bin_to_uuid(id) as id, task FROM task ".to_string();

    let task = sqlx::query_as::<_, task::Task>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(task))
}

pub async fn find_task(
    Path(id): Path<i32>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<Json<task::Task>, CustomError> {
    let sql = "SELECT * FROM task where id=$1".to_string();

    let task: task::Task = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::TaskNotFound)?;

    Ok(Json(task))
}

pub async fn create_task(
    Extension(pool): Extension<MySqlPool>,
    Json(task): Json<task::NewTask>,
) -> Result<(StatusCode, Json<task::NewTask>), CustomError> {
    if task.task.is_empty() {
        return Err(CustomError::BadRequest);
    }
    let sql = "INSERT INTO task (task) values ($1)";

    let _ = sqlx::query(&sql)
        .bind(&task.task)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn update_task(
    Extension(pool): Extension<MySqlPool>,
    Path(id): Path<i32>,
    Json(task): Json<task::UpdateTask>,
) -> Result<(StatusCode, Json<task::UpdateTask>), CustomError> {
    let sql = "SELECT * FROM task where id=$1".to_string();

    let _find: task::Task = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::TaskNotFound)?;

    sqlx::query("UPDATE task SET task=$1 WHERE id=$2")
        .bind(&task.task)
        .bind(id)
        .execute(&pool)
        .await;

    Ok((StatusCode::OK, Json(task)))
}

pub async fn delete_task(
    Path(id): Path<i32>,
    Extension(pool): Extension<MySqlPool>,
) -> Result<(StatusCode, Json<Value>), CustomError> {
    let _find: task::Task = sqlx::query_as("SELECT * FROM task where id=$1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::TaskNotFound)?;

    sqlx::query("DELETE FROM task WHERE id=$1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::TaskNotFound)?;

    Ok((StatusCode::OK, Json(json!({"msg": "Task Deleted"}))))
}
