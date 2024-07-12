use actix_web::{web, HttpResponse};
use crate::state::AppState;
pub async fn health_check_handler(app_state:web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visitor_count = app_state.visitor_count.lock().unwrap();
    let response = format!("{}! Visitor count: {}", health_check_response, *visitor_count);
    *visitor_count += 1;
    HttpResponse::Ok().json(&response)
}