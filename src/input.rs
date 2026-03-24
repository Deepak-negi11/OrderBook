use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateOrderInput{
    pub price:u64,
    pub quantity:u64,
    pub user_id : String,
    pub side : Side

}

#[derive(Serialize , Deserialize)]
pub enum Side{
    Buy , 
    Sell
}

#[derive(Serialize , Deserialize)]
pub struct DeleteOrderInput{
    pub order_id:String,
    pub price : u64,
    pub side : Side
}