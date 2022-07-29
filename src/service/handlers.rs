use thiserror::Error;
use warp::hyper::StatusCode;
use warp::Reply;

use super::schemas;
use crate::db;
use crate::records::tasks::Task;

#[derive(Error, Debug)]
enum TaskError {
    #[error("db error")]
    DatabaseError(#[from] db::errors::Error),
}

impl warp::reject::Reject for TaskError {}

pub async fn create_task(
    req: schemas::CreateTask,
    db: db::Pool,
) -> Result<impl Reply, warp::Rejection> {
    let task = Task::create(
        req.data.attributes.title,
        req.data.attributes.description,
        req.data.attributes.priority,
        &db,
    )
    .await
    .map_err(TaskError::DatabaseError)?;

    Ok(warp::reply::json(&schemas::Task::new(
        task.id,
        task.title,
        task.description,
        task.priority
    )))
}

pub async fn get_task_by_id(id: i64, db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let task = Task::find(id, &db)
        .await
        .map_err(TaskError::DatabaseError)?;

    let task = match task {
        Some(task) => task,
        None => return Ok(StatusCode::NOT_FOUND.into_response()),
    };

    Ok(warp::reply::json(&schemas::Task::new(
        task.id,
        task.title,
        task.description,
        task.priority,
    ))
    .into_response())
}

pub async fn get_tasks_list(db: db::Pool) -> Result<impl Reply, warp::Rejection> {
    let tasks = Task::select(&db).await.map_err(TaskError::DatabaseError)?;

    Ok(warp::reply::json(&schemas::TaskList::new(tasks)).into_response())
}
