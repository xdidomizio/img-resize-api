#[macro_use]
extern crate serde;
use actix_web::{get, web, App, HttpServer, Responder};
use std::env;
use std::io;
mod processing;

#[derive(Deserialize)]
pub struct ResizeRequest {
    image_name: String,
    width: String,
    height: String,
}

async fn index(params: web::Query<ResizeRequest>) -> impl Responder {
    let image_size = processing::ImageSize {
        width: params.width.parse().unwrap(),
        height: params.height.parse().unwrap(),
    };

    let into: String = "result.png".to_string();
    let image_path = format!("img/{}", &params.image_name);
    let result_path = format!("img/{}", &into);
    processing::run_conversion(&image_path, &result_path, image_size);
    format!("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let (from, into, width, height) = if env::args_os().count() == 5 {
    //     (
    //         env::args_os().nth(1).unwrap().into_string().unwrap(),
    //         env::args_os().nth(2).unwrap().into_string().unwrap(),
    //         env::args_os().nth(3).unwrap().into_string().unwrap(),
    //         env::args_os().nth(4).unwrap().into_string().unwrap(),
    //     )
    // } else {
    //     println!("Please enter a from path, destination path, target width ad height");
    //     std::process::exit(1);
    // };

    // #[get("/img/resize?image_name={image_name}&width={width}&height={height}")]
    HttpServer::new(|| App::new().service(web::resource("/img/resize").route(web::get().to(index))))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
