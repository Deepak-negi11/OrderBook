use std::sync::{Arc,Mutex};
use actix_web::{web::Data,App,HttpServer};

mod input;
mod output;
mod orderbook;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let orderbook = Arc::new(Mutex::new(orderbook::Orderbook::new()));
    println!("Server is running at port 3000");
    HttpServer::new(move|| {
        App::new()
        .app_data(Data::new(orderbook.clone()))
        .service(routes::create_order)
        .service(routes::delete_order)
        .service(routes::get_depth)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}