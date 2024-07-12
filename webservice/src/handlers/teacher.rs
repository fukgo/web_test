use actix_web::{web, HttpResponse};
use crate::dbaccess::course::update_course_details_db;
use crate::dbaccess::teacher::*;
use crate::models::teacher::{Teacher,CreatTeacher,UpdateTeacher};
use crate::state::AppState;
use crate::error::MyError;

pub async fn post_new_teacher(new_teacher: web::Json<CreatTeacher>, app_state: web::Data<AppState>) -> Result<HttpResponse,MyError> {
    let teachers = post_new_teacher_db(&app_state.db, new_teacher.into_inner()).await?; 
    Ok(HttpResponse::Ok().json(teachers))
}

pub async fn delete_teacher(app_state: web::Data<AppState>,params: web::Path<usize>) -> Result<HttpResponse,MyError> {
    let teacher_id = params.into_inner();
    let res = delete_teacher_db(&app_state.db, teacher_id as i32).await?;
    Ok(HttpResponse::Ok().json(res))
}

pub async fn update_teacher_details(app_state: web::Data<AppState>,params: web::Path<usize>,update_teacher: web::Json<UpdateTeacher>) -> Result<HttpResponse,MyError> {
    let teacher_id = params.into_inner();
    let res = update_teacher_db(&app_state.db, teacher_id as i32, update_teacher.into_inner()).await?;
    Ok(HttpResponse::Ok().json(res))

}
pub async fn get_teacher_detail(app_state: web::Data<AppState>,params: web::Path<usize>) -> Result<HttpResponse,MyError> {
    let teacher_id = params.into_inner();
    let teacher = get_teacher_detail_db(&app_state.db, teacher_id as i32).await?;
    Ok(HttpResponse::Ok().json(teacher))
}
pub async fn get_all_teacher_detail(app_state: web::Data<AppState>) -> Result<HttpResponse,MyError> {
    let teachers = get_all_teacher_detail_db(&app_state.db).await?;
    Ok(HttpResponse::Ok().json(teachers))
}

#[cfg(test)]
mod tests{
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::mysql::MySqlPoolOptions;
    use std::env;
    use std::sync::Mutex;
    use uuid::Uuid;
    #[actix_rt::test]
    async fn post_teacher_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not defined in .env file");
        let db_pool = MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visitor_count: Mutex::new(0),
            db: db_pool,
        });
        let new_teacher = web::Json(CreatTeacher{
            name: "test".to_string(),
            picture_url: "test".to_string(),
            profile: "test".to_string(),
        });
        let resp = post_new_teacher(new_teacher, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[ignore = "delete ignored for now"]
    #[actix_rt::test]
    async fn delete_teacher_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not defined in .env file");
        let db_pool = MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visitor_count: Mutex::new(0),
            db: db_pool,
        });
        let resp = delete_teacher(app_state, web::Path::from(1)).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn update_teacher_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not defined in .env file");
        let db_pool = MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visitor_count: Mutex::new(0),
            db: db_pool,
        });
        let update_teacher = web::Json(UpdateTeacher{
            name: Some("test".to_string()),
            picture_url: Some("test".to_string()),
            profile: Some("test".to_string()),
        });
        let resp = update_teacher_details(app_state, web::Path::from(1), update_teacher).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_teacher_detail_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not defined in .env file");
        let db_pool = MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visitor_count: Mutex::new(0),
            db: db_pool,
        });
        let resp = get_teacher_detail(app_state, web::Path::from(1)).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_teacher_detail_test(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not defined in .env file");
        let db_pool = MySqlPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visitor_count: Mutex::new(0),
            db: db_pool,
        });
        let resp = get_all_teacher_detail(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}