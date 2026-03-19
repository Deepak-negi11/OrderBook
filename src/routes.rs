use crate::{
    input::{CreateOrderInput, DeleteOrderInput},
    orderbook::{self, Orderbook},
    outputs::{CreateOrderOutput, DeleteOrderOutput, Depth},
};
use actix_web::{
    HttpResponse, Responder, delete, get, post,
    web::{self, Data, Json},
};
use std::sync::{Arc, Mutex};

#[get("/depth")]
pub async fn get_depth(orderbook: Data<Arc<Mutex<Orderbook>>>) -> impl Responder {
    let orderbook = orderbook.lock().unwrap();
    let depth = orderbook.get_depth();
    HttpResponse::Ok().json(depth)
}

#[post("/order")]
pub async fn create_order(orderbook:Data<Arc<Mutex<Orderbook>>>, Json(body):Json<CreateOrderInput>) -> impl Responder{
    let mut ob = orderbook.lock().unwrap();
    let result = ob.create_order(
        body.price,
       
    );
    HttpResponse.Ok().json(CreateOrderOutput{
        order_id: result.order_id
    })
}


#[delete("/order")]
pub async fn delete_order(orderbook: Data<Arc<Mutex<Orderbook>>>, order: Json<DeleteOrder>) -> impl Responder {
    let mut orderbook = orderbook.lock().unwrap();
    let orderbook = orderbook.delete_order(order.0);
    HttpResponse::Ok().json(orderbook)
}