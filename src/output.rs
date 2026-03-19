use serde::{Deserialize , Serialize};


#[derive(Deserialize , Serialize, Debug)]
pub struct CreateOrderResponse{
    pub order_id:String,
    pub filled_qty:u64,
    pub average_price: f64,
    pub status :String

}

#[derive(Serialize,Deserialize)]
pub struct DelteOrderResponse{
    pub order_id : String,
    pub success : bool
}

#[derive(Debug , Serialize)]
pub struct PriceLevel{
    pub price :u64,
    pub quantity: u64,
    pub order_count: usize,
}

#[derive(Serialize,Deserialize)]
pub struct DepthResponse{
    pub bids:Vec<PriceLevel>,
    pub asks:Vec<PriceLevel>,
    pub lastUpdatedId:u64
}