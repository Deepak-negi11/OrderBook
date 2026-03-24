use crate::{
    input::{CreateOrderInput, DeleteOrderInput},
    orderbook::{ Orderbook},
    output::{CreateOrderResponse, DeleteOrderResponse, DepthResponse},
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
        body
       
    );
    HttpResponse::Ok().json(CreateOrderResponse{
        order_id: result.order_id,
        filled_qty: result.filled_qty,
        average_price: result.average_price,
        status:result.status
    })
}


#[delete("/order")]
pub async fn delete_order(orderbook: Data<Arc<Mutex<Orderbook>>>, order: Json<DeleteOrderInput>) -> impl Responder {
    let mut orderbook = orderbook.lock().unwrap();
    let orderbook = orderbook.delete_order(order.0);
    HttpResponse::Ok().json(orderbook)
}