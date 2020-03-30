use actix_files::{Files, NamedFile};
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{App, Error, HttpServer};

use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::{HeaderName, HeaderValue};
use std::io::Read;

async fn fallback(req: ServiceRequest) -> Result<ServiceResponse, Error> {
    // Not possible to pass via actix data so retrive the base path again
    let args: Vec<_> = std::env::args().collect();
    let root_dir = args.get(1).unwrap_or(&".".to_string()).clone();

    // Remove first slash
    let path = req.path().replacen("/", "", 1);
    let fullpath = format!("{}/{}.html", root_dir, path);

    let data = NamedFile::open(&fullpath)
        .map(|mut x| {
            let mut str = String::new();
            x.read_to_string(&mut str)
                .expect(format!("Failed to read file {}", fullpath).as_str());
            str
        })
        .unwrap_or("<body><h1>404 file not found</h1></body>".to_string());

    let mut res = req.into_response(data);
    res.headers_mut().remove("content-type");
    res.headers_mut().append(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/html"),
    );
    Ok(res)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let root_dir = args.get(1).unwrap_or(&".".to_string()).clone();

    println!("Serving {} on 0.0.0.0:8080", root_dir);

    HttpServer::new(move || {
        App::new()
            .service(
                Files::new("/", &root_dir)
                    .index_file("index.html")
                    .show_files_listing()
                    .redirect_to_slash_directory()
                    .use_etag(true)
                    .default_handler(fallback),
            )
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
