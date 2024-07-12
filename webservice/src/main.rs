use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::io;
use sqlx::Row;
#[derive(Debug)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub description: String,
    pub time: Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    let sql_pool = MySqlPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();
    let course_rows = sqlx::query("SELECT id, teacher_id, name, description, time FROM course WHERE id = ?")
    .bind(1)
    .fetch_all(&sql_pool)
    .await
    .unwrap();

    let mut courses_list = vec![];
    for row in course_rows {
        let id: i32 = row.get("id");
        let teacher_id: i32 = row.get("teacher_id");
        let name: String = row.get("name");
        let description: String = row.get("description");
        let time: Option<NaiveDateTime> = row.get("time");

        courses_list.push(Course {
            id,
            teacher_id,
            name,
            description,
            time,
        });
    }
    println!("Courses = {:?}", courses_list);

    Ok(())
}