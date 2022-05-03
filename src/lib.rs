use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub struct APIGateway {
    port: u16,
    workers: usize,
}

impl APIGateway {
    pub fn new(port: u16, workers: usize) -> Self {
        APIGateway { port, workers }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{} with {} workers.", self.port, self.workers);

        HttpServer::new(|| {
            App::new()
                .service(hello)
                .service(echo)
                .route("/hey", web::get().to(manual_hello))
        })
        .bind(("127.0.0.1", self.port))?
        .workers(self.workers)
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

