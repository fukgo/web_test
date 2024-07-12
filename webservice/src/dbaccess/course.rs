// 修复代码中的问题

use std::{f32::consts::E, result::Result};

use chrono::NaiveDateTime;
use sqlx::mysql::MySqlPool;
use uuid::Uuid;
use crate::models::course::{Course, CreatCourse, UpdateCourse};
use sqlx::Row;
use crate::error::MyError;
// 获取某位教师的所有课程
pub async fn get_courses_for_teacher_db(pool: &MySqlPool, teacher_id: i32) -> Result<Vec<Course>,MyError> {
    let course_rows = sqlx::query_as::<_,Course>("SELECT * FROM course WHERE teacher_id = ?")
        .bind(teacher_id)
        .fetch_all(pool)
        .await?;
    // 如果没有课程，返回一个错误
    match course_rows.len() {
        0 => Err(MyError::NotFound("Course not found".to_string())),
        _ =>Ok(course_rows)
    }

}

// 获取某位教师的某门课程的详细信息
pub async fn get_course_details_db(pool: &MySqlPool, teacher_id: i32, course_id: i32) -> Result<Vec<Course>,MyError> {
    let course_rows = sqlx::query_as::<_,Course>("SELECT * FROM course WHERE teacher_id = ? AND id = ?")
        .bind(teacher_id)
        .bind(course_id)
        .fetch_optional(pool)
        .await
        ?;
    // 如果没有课程，返回一个错误
    if let Some(course) = course_rows {
        Ok(vec![course])
    } else {
        Err(MyError::NotFound("Course not found".to_string()))
    }

}

// 向数据库中添加新课程
pub async fn post_new_course_db(pool: &MySqlPool, new_course: CreatCourse) -> Result<Vec<Course>,MyError> {
    let uuid = Uuid::new_v4();
    // 首先执行插入操作
    sqlx::query("INSERT INTO course (uuid,teacher_id, name, description) VALUES (?,?, ?, ?)")
        .bind(uuid.to_string())
        .bind(new_course.teacher_id)
        .bind(new_course.name)
        .bind(new_course.description)
        .execute(pool)
        .await
        ?;

    // 假设你想要获取所有课程作为示例，这里使用SELECT查询
    let course_rows = sqlx::query_as::<_, Course>("SELECT * FROM course")
        .fetch_all(pool)
        .await
        ?;

    // course_rows已经是Course类型的Vec了，直接返回
    Ok(course_rows)
}
pub async fn delete_course_db(pool: &MySqlPool, teacher_id: i32, course_id: i32) -> Result<String,MyError> {
    // 首先执行删除操作
    let delete_count = sqlx::query("DELETE FROM course WHERE teacher_id = ? AND id = ?")
        .bind(teacher_id)
        .bind(course_id)
        .execute(pool)
        .await
        ?;
    // 如果没有删除任何课程，返回一个错误
    //delete_count.rows_affected() 将返回被删除的行数
    match delete_count.rows_affected() {
        1 => Ok(format!("Deleted {:?} record", delete_count)),
        _ =>Err(MyError::NotFound("Course not found".to_string()))

    }


}

pub async fn update_course_details_db(pool: &MySqlPool,teacher_id: i32,id:i32,update_course:UpdateCourse)->Result<String,MyError>{
    let current_course_row = sqlx::query_as::<_,Course>("SELECT * FROM course WHERE teacher_id = ? AND id = ?")
        .bind(teacher_id)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|_err|MyError::NotFound("Course not found".to_string()))?;
    //如果 update_course.name 有值，就使用这个值作为 name，否则使用 current_course_row.name 作为 name
    let name  = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };
    let description = if let Some(description) = update_course.description {
        description
    } else {
        current_course_row.description
    };
    let rows_affected = sqlx::query("UPDATE course SET name = ?, description = ? WHERE teacher_id = ? AND id = ?")
        .bind(name)
        .bind(description)
        .bind(teacher_id)
        .bind(id)
        .execute(pool)
        .await?;
    match rows_affected.rows_affected() {
        1 => Ok(format!("Updated {:?} record", rows_affected)),
        _ => Err(MyError::NotFound("Teacher not found".to_string()))
    }
}