use serde::{Deserialize, Serialize};

use crate::records;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Task,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskAttributes {
    pub title: String,
    pub description: String,
    pub priority: records::TaskPriority,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskData {
    pub id: String,
    #[serde(rename = "type")]
    _type: ResourceType,
    pub attributes: TaskAttributes,
}

impl TaskData {
    pub fn new(id: i64, title: String, desc: String, priority: records::TaskPriority) -> Self {
        Self {
            id: id.to_string(),
            _type: ResourceType::Task,
            attributes: TaskAttributes {
                title,
                priority,
                description: desc,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub data: TaskData,
}

impl Task {
    pub fn new(id: i64, title: String, desc: String, priority: records::TaskPriority) -> Self {
        Self {
            data: TaskData::new(id, title, desc, priority),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    data: Vec<TaskData>,
}

impl TaskList {
    pub fn new(tasks: Vec<records::Task>) -> Self {
        Self {
            data: tasks
                .into_iter()
                .map(|raw| TaskData::new(raw.id, raw.title, raw.description, raw.priority))
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskData {
    #[serde(rename = "type")]
    pub _type: ResourceType,
    pub attributes: TaskAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTask {
    pub data: CreateTaskData,
}
