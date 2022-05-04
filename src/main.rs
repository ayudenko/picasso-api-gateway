use picasso_api_gateway::APIGateway;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let app = APIGateway::new();
    app.run().await
}

