use actix_files::{Files, NamedFile};
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{App, Error, HttpServer};

use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::{HeaderName, HeaderValue};
use std::io::Read;

async fn fallback(req: ServiceRequest) -> Result<ServiceResponse, Error> {
    // Not possible to pass via actix data so retrive the base path again
    let args: Vec<_> = std::env::args().collect();
    let root_dir = args.get(1).unwrap_or(&".".to_string()).clone();

    // Remove first slash
    let path = req.path().replacen('/', "", 1);
    let fullpath = format!("{root_dir}/{path}.html");

    let data = NamedFile::open(&fullpath)
        .map(|mut x| {
            let mut str = String::new();
            x.read_to_string(&mut str)
                .unwrap_or_else(|_| panic!("Failed to read file {fullpath}"));
            str
        })
        .unwrap_or("<body><h1>404 file not found</h1></body>".to_string());

    let mut res = req.into_response(data);
    res.headers_mut().remove("content-type");
    res.headers_mut().append(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/html"),
    );
    Ok(res.map_into_boxed_body())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let args: Vec<_> = std::env::args().collect();
    let root_dir = args.get(1).unwrap_or(&".".to_string()).clone();

    println!("Serving {root_dir} on 0.0.0.0:8080");

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
