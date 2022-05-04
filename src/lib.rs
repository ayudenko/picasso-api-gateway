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

        let ssl_key_path = dotenv::var("SSL_KEY_PATH").unwrap_or_else(|_| {
            panic!("Can not read SSL_KEY_PATH variable from .env file!");
        });
        let ssl_cert_path = dotenv::var("SSL_CERT_PATH").unwrap_or_else(|_| {
            panic!("Can not read SSL_CERT_PATH variable from .env file!")
        });
        let workers = dotenv::var("WORKERS")
            .unwrap_or_else(|_| {
                panic!("Can not read WORKERS variable from .env file!")
            })
            .parse::<usize>().unwrap_or_else(|_| {
                panic!("WORKERS variable value should be of usize type!")
            });
        let host = dotenv::var("HOST").unwrap_or_else(|_| {
            panic!("Can not read HOST variable from .env file!")
        });
        let port = dotenv::var("PORT")
            .unwrap_or_else(|_| {
                panic!("Can not read PORT variable from .env file!")
            })
            .parse::<u16>().unwrap_or_else(|_| {
                panic!("PORT variable value should be of u16 type!")
            });

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
            .unwrap_or_else(|_| {
                panic!("Can not load SSL private key file!")
            });
        builder.set_certificate_chain_file(&self.config.ssl_cert_path)
            .unwrap_or_else(|_| {
                panic!("Can not load SSL certificate chain file!")
            });

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

