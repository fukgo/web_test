use dotenv::dotenv;

fn main(){
    dotenv().ok();
    //判断存在.env文件并且声明DATABASE_URL
    match std::env::var("DATABASE_URL") {
        Ok(val) => println!("DATABASE_URL: {}", val),
        Err(_e) => println!("DATABASE_URL not defined in .env file"),
    }
}