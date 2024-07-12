use chrono::NaiveDateTime;
use actix_web::web;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
#[derive(Debug, Serialize,Clone,FromRow)]
pub struct  Course{
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub uuid: String,
    pub description: String,
    pub time: Option<NaiveDateTime>,

}
#[derive(Deserialize,Serialize, Debug, Clone, sqlx::FromRow)]
pub struct  CreatCourse{
    pub teacher_id: i32,
    pub name: String,
    pub description: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse{
    pub name: Option<String>,
    pub description: Option<String>,
}

//允许 web::Json<Course> 类型转换为 CreatCourse 类型
impl From<web::Json<Course>> for CreatCourse{
    fn from(course: web::Json<Course>) -> Self {
        CreatCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            description: course.description.clone(),
        }
    }
}