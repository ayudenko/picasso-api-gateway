use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use dotenv::dotenv;

struct Config {
    ssl_key_path: String,
    ssl_cert_path: String,
    workers: usize,
    host: String,
    port: u16,
}

impl Config {
    fn new() -> Result<Self, String> {
        dotenv().ok();

        let ssl_key_path = dotenv::var("SSL_KEY_PATH").unwrap();
        let ssl_cert_path = dotenv::var("SSL_CERT_PATH").unwrap();
        let workers = dotenv::var("WORKERS").unwrap().parse::<usize>().unwrap();
        let host = dotenv::var("HOST").unwrap();
        let port = dotenv::var("PORT").unwrap().parse::<u16>().unwrap();

        Ok(Config {
            ssl_key_path,
            ssl_cert_path,
            workers,
            host,
            port,
        })
    }
}

pub struct APIGateway {
    config: Config,
}

impl APIGateway {
    pub fn new() -> Self {
        let config = Config::new();
        APIGateway { config: config.unwrap() }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        println!("{}", format!(
            "Starging http server: {}:{} with {} workers.",
            self.config.host,
            self.config.port,
            self.config.workers,
        ));

        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(&self.config.ssl_key_path, SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(&self.config.ssl_cert_path).unwrap();

        HttpServer::new(|| {
            App::new()
                .service(hello)
                .service(echo)
                .route("/hey", web::get().to(manual_hello))
        })
        .bind_openssl(format!("{}:{}", self.config.host, self.config.port), builder)?
        .workers(self.config.workers)
        .run()
        .await
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

