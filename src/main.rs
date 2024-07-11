use actix_files::{Files, NamedFile};
use actix_web::middleware::{Compress, Logger, NormalizePath, TrailingSlash};
use actix_web::{App, Error, HttpServer};

use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::{HeaderName, HeaderValue};
use clap::Parser;
use std::io::Read;
use tracing::{info, Level};

async fn fallback(req: ServiceRequest) -> Result<ServiceResponse, Error> {
    // Not possible to pass via actix data so retrieve the base path again
    let args: Vec<_> = std::env::args().collect();
    let root_dir = args.get(1).unwrap_or(&".".to_string()).clone();

    // Remove first slash
    let path = req.path().replacen('/', "", 1);
    let full_path = format!("{root_dir}/{path}.html");

    let data = NamedFile::open(&full_path)
        .map(|mut x| {
            let mut str = String::new();
            x.read_to_string(&mut str)
                .unwrap_or_else(|_| panic!("Failed to read file {full_path}"));
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

/// Serve a directory
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port to run on
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Directory to host
    #[arg(default_value = ".")]
    path: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let args = Args::parse();

    let root_dir = args.path;
    let port = args.port;

    info!("Serving {root_dir} on 0.0.0.0:{port}");

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
            .wrap(NormalizePath::new(TrailingSlash::Always))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
