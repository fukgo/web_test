use std::{f32::consts::E, result::Result};
use crate::models::teacher::{Teacher,CreatTeacher,UpdateTeacher};
use chrono::NaiveDateTime;
use sqlx::mysql::MySqlPool;
use uuid::Uuid;
use sqlx::Row;
use crate::error::MyError;
//创建老师数据
pub async fn post_new_teacher_db(pool: &MySqlPool,new_teacher:CreatTeacher)->Result<Vec<Teacher>,MyError>{
    let uuid = Uuid::new_v4();
    sqlx::query("insert into teacher (uuid,name,picture_url,profile) values (?,?,?,?)")
        .bind(uuid.to_string())
        .bind(new_teacher.name)
        .bind(new_teacher.picture_url)
        .bind(new_teacher.profile)
        .execute(pool)
        .await?;
    //返回该老师的所有数据
    let teacher_rows = sqlx::query_as::<_,Teacher>("select * from teacher where uuid =?")
        .bind(uuid.to_string())
        .fetch_one(pool)
        .await?;
    Ok(vec![teacher_rows])
}
//删除老师数据
pub async fn delete_teacher_db(pool: &MySqlPool,teacher_id:i32)->Result<String,MyError>{
    let delete_count = sqlx::query("delete from teacher where id = ?")
        .bind(teacher_id)
        .execute(pool)
        .await?;
    match delete_count.rows_affected() {
        1 => Ok(format!("Deleted {:?} record", delete_count)),
        _ =>Err(MyError::NotFound("Course not found".to_string()))
}
}
//修改老师数据
pub async fn update_teacher_db(pool: &MySqlPool,teacher_id:i32,update_teacher:UpdateTeacher)->Result<String,MyError>{
    let current_course_row = sqlx::query_as::<_,Teacher>("select * from teacher where id = ?")
        .bind(teacher_id)
        .fetch_one(pool)
        .await
        .map_err(|_| MyError::NotFound("Teacher not found".to_string()))?;
    let name = if let Some(name) = update_teacher.name {
        name
    } else {
        current_course_row.name
    };
    let picture_url = if let Some(picture_url) = update_teacher.picture_url {
        picture_url
    } else {
        current_course_row.picture_url
    };
    let profile = if let Some(profile) = update_teacher.profile {
        profile
    } else {
        current_course_row.profile
    };
    let rows_affected = sqlx::query("update teacher set name = ?,picture_url = ?,profile = ? where id = ?")
    .bind(name)
    .bind(picture_url)
    .bind(profile)
    .bind(teacher_id)
    .execute(pool)
    .await?;
    match rows_affected.rows_affected() {
        1 => Ok(format!("Updated {:?} record", rows_affected)),
        _ => Err(MyError::NotFound("Teacher not found".to_string()))
    }
}
//获取所有老师数据
pub async fn get_all_teacher_detail_db(pool: &MySqlPool,)->Result<Vec<Teacher>,MyError>{
    let teacher_rows = sqlx::query_as::<_,Teacher>("select * from teacher")
        .fetch_all(pool)
        .await?;
    Ok(teacher_rows)
}
//获取某个老师所有数据
pub async fn get_teacher_detail_db(pool: &MySqlPool,teacher_id:i32)->Result<Vec<Teacher>,MyError>{
    let teacher_rows = sqlx::query_as::<_,Teacher>("select * from teacher where id = ?")
        .bind(teacher_id)
        .fetch_all(pool)
        .await?;
    Ok(teacher_rows)
}

// 获取某位教师的所有课程
pub async fn get_teacher_course_db(pool: &MySqlPool,teacher_id:i32)->Result<Vec<Teacher>,MyError>{
    let teacher_rows = sqlx::query_as::<_,Teacher>("select * from course where teacher_id = ?")
        .bind(teacher_id)
        .fetch_all(pool)
        .await?;
    Ok(teacher_rows)
}
