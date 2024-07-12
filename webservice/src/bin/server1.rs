use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::io;
//配置路由
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}
//配置handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web Server is running!")
}
//实例化HTTP服务器
#[actix_web::main]
async fn main() -> io::Result<()> {
    //构建app,配置路由
    let app = move || {
        App::new()
            .configure(general_routes)
    };
    //运行HTTP服务器
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await

}