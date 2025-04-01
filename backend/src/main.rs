use std::fs;
use std::path::Path;

use actix_web::{App, HttpResponse, HttpServer, web};

async fn list_directories() -> HttpResponse {
    let base_folder = Path::new("/home/jonesgc/Repos/Rust/rover/integration/workdir/");
    let subdirectories: Vec<String> = fs::read_dir(base_folder)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_dir() {
                Some(path.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();
    HttpResponse::Ok().json(subdirectories)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting backend server. . .");
    HttpServer::new(|| App::new().route("/list", web::get().to(list_directories)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
