use std::fmt::{Display, Formatter};
use actix_web::http::KeepAlive;
use actix_web::{HttpServer, web};
use anyhow::bail;
use rand::{Rng, thread_rng};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("server started at 0.0.0.0:18080");

    HttpServer::new(|| {
        actix_web::App::new()
            .service(web::resource("/hello").route(web::get().to(hello)))
            .service(web::resource("/ping").route(web::get().to(ping)))
    })
        .keep_alive(KeepAlive::Os)
        .bind("0.0.0.0:18080")?
        .run()
        .await?;

    unreachable!()
}

fn is_ok() -> bool {
    thread_rng().gen_bool(0.6)
}

async fn hello() -> anyhow_to_actix_error::Result<&'static str> {
    return Ok(may_generate_error("hello, world!").await?);
}

async fn ping() -> anyhow_to_actix_error::Result<&'static str> {
    return Ok(may_generate_error("pong").await?);
}

async fn may_generate_error(val: &'static str) -> anyhow::Result<&'static str> {
    if is_ok() {
        return Ok(val);
    }

    bail!("custom error generated");

    // or use a custom struct (impl std::error::Error, which can be converted into anyhow::Error with From<T>)
    Err(MyError("custom error generated".to_string()).into())
}

#[derive(Debug)]
struct MyError(String);

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }    
}

impl std::error::Error for MyError {}
