use actix_web::{web, HttpResponse};
use crate::dbaccess::course::{get_courses_for_teacher_db,post_new_course_db,get_course_details_db,delete_course_db,update_course_details_db};
use crate::models::course::{Course,CreatCourse,UpdateCourse};
use crate::state::AppState;
use crate::error::MyError;



pub async fn post_new_course(new_course: web::Json<CreatCourse>, app_state: web::Data<AppState>) -> Result<HttpResponse,MyError> {
    let courses = post_new_course_db(&app_state.db, new_course.into_inner()).await?; 
    Ok(HttpResponse::Ok().json(courses))
}
pub async fn update_course(update_course: web::Json<UpdateCourse>,app_state: web::Data<AppState>,params: web::Path<(usize, usize)>)->Result<HttpResponse,MyError>{
    let (teacher_id, course_id) = params.into_inner();
    let res = update_course_details_db(&app_state.db, teacher_id as i32, course_id as i32, update_course.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))

}
pub async fn delete_course(app_state: web::Data<AppState>,params: web::Path<(usize, usize)>) -> Result<HttpResponse,MyError> {
    let (teacher_id, course_id) = params.into_inner();
    let res = delete_course_db(&app_state.db, teacher_id as i32, course_id as i32).await?;
    Ok(HttpResponse::Ok().json(res))
}
pub async fn get_courses_for_teacher(app_state: web::Data<AppState>,params: web::Path<usize>) -> Result<HttpResponse,MyError> {
    let teacher_id = params.into_inner();
    let courses = get_courses_for_teacher_db(&app_state.db, teacher_id as i32).await?;
    Ok(HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(app_state: web::Data<AppState>,params: web::Path<(usize, usize)>) -> Result<HttpResponse,MyError> {
    let (teacher_id, course_id) = params.into_inner();
    let course = get_course_details_db(&app_state.db, teacher_id as i32, course_id as i32).await?;
    Ok(HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    // use chrono::NaiveDateTime;
    use dotenv::dotenv;
    use sqlx::mysql::MySqlPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visitor_count: Mutex::new(0),
            db: db_pool,
        });

        let course = web::Json(CreatCourse {
            teacher_id: 1,
            name: "Test course".into(),
            description: "Test course description".into(),
        });

        let resp = post_new_course(course, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visitor_count: Mutex::new(0),
            db: db_pool,
        });

        let teacher_id: web::Path<usize> = web::Path::from(1);
        let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_one_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visitor_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(usize, usize)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visitor_count: Mutex::new(0),
            db: db_pool,
        });

        let update_course_ = web::Json(UpdateCourse {
            name: Some("Test course".into()),
            description: Some("Test course description".into()),
        });

        let params: web::Path<(usize, usize)> = web::Path::from((1, 1));
        let resp = update_course(update_course_, app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}