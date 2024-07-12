
use crate::handlers::course::*;
use crate::handlers::teacher::*;
use crate::handlers::general::health_check_handler;
use actix_web::web::{self, route};
use webservice::dbaccess::course::update_course_details_db;
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/courses")
    .route("/", web::post().to(post_new_course))
    .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
    .route("/{teacher_id}/{course_id}", web::get().to(get_course_detail))
    .route("/{teacher_id}/{course_id}", web::put().to(update_course))
    .route("/{teacher_id}/{course_id}",web::delete().to(delete_course))
);

}
pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/teachers")
    .route("/", web::post().to(post_new_teacher))
    .route("/{teacher_id}", web::get().to(get_teacher_detail))
    .route("/{teacher_id}", web::put().to(update_teacher_details))
    .route("/{teacher_id}",web::delete().to(delete_teacher))
    .route("/", web::get().to(get_all_teacher_detail))

);

}