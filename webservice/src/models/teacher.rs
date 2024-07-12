use serde::{Deserialize,Serialize};
use sqlx::FromRow;
use uuid::Uuid;
/*
    id int primary key auto_increment,
    name varchar(255) not null,
    uuid varchar(255) not null,
    picture_url varchar(255) not null,
    profile text,

*/
#[derive(Debug, Serialize,Clone,FromRow)]
pub struct  Teacher{
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}
#[derive(Deserialize,Serialize, Debug, Clone, sqlx::FromRow)]
pub struct CreatTeacher{
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher{
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}