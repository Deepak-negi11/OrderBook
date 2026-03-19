use serde::{Deserialize, Serialize};

#[derive(Deserialize , Debug)]
pub struct CreateOrderInput{
    pub price:u32,
    pub quality:u32,
    pub user_id : u32,
    pub side : Side

}

#[derive(Deserialize , Debug , Partialize)]
pub enum Side{
    Buy , 
    Sell
}

#[derive(Serialize,Deserialize)]
pub struct DeleteOrderInput{
    pub order_id:String
}