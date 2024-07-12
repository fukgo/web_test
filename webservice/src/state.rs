/*实现健康检查功能*/
use std::sync::Mutex;
//Mutex确保了在任何时刻，只有一个线程可以访问被保护的数据，从而避免了数据竞争和不一致的问题。
pub struct AppState {
    pub health_check_response: String,
    pub visitor_count: Mutex<u64>,
    //pub courses: Mutex<Vec<Course>>,
    pub db: sqlx::MySqlPool,
}