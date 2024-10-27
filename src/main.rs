use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Result};
use std::path::PathBuf;
use qrcode::QrCode;
use image::Luma;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    url: String
}

#[get("/gen")]
async fn gen_qrcode(query: web::Query<Info>) -> Result<NamedFile> {
    let code = QrCode::new(query.url.as_bytes()).unwrap();

    println!("Creating QR Code for URL: {}", query.url);

    let image = code.render::<Luma<u8>>().build();
    image.save("tmp/qrcode.jpg").unwrap();

    let file_path = PathBuf::from("tmp/qrcode.jpg");

    NamedFile::open(file_path).map_err(|e| actix_web::error::ErrorInternalServerError(e))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on 0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .service(gen_qrcode)
    })
    .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
