use actix_web::{http, web, App, HttpServer};
use dbaccess::teacher;
use error::MyError;
use std::io;
use std::sync::Mutex;
use sqlx::mysql::MySqlPoolOptions;
use actix_cors::Cors;
#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../models/mod.rs"]
mod models;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

#[path = "../error.rs"]
mod error;
use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main()-> io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    let sql_pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visitor_count: Mutex::new(0),
        //courses: Mutex::new(Vec::new()),
        db:sql_pool,
    });
    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().starts_with(b"http://localhost"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::InvalidInput("Invalid path".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .wrap(cors)
            .configure(teacher_routes)
    };
    println!("Server running at http://127.0.0.1:3000");
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await

}

/*
curl -X POST http://127.0.0.1:3000/courses/ \
-H "Content-Type: application/json" \
-d '{"id":"1","teacher_id":"1","name":"test","description":"none","time":"2020-01-03 00:00:00"}' 



*/