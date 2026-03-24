use crate::input::{CreateOrderInput, DeleteOrderInput, Side};
use crate::output::{CreateOrderResponse, DeleteOrderResponse, DepthResponse, PriceLevel};
use std::collections::{BTreeMap, VecDeque};



#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub price: u64,
    pub quantity: u64,
    pub user_id: String,
}

#[derive(Debug)]
pub struct Orderbook {
    pub bids: BTreeMap<u64, VecDeque<Order>>,
    pub asks: BTreeMap<u64, VecDeque<Order>>,
    pub last_update_id: u64,
    next_order_id: u64,
}
impl Orderbook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_update_id: 0,
            next_order_id: 1,
        }
    }

    pub fn create_order(&mut self, input: CreateOrderInput) -> CreateOrderResponse {
        let order_id = self.next_order_id.to_string();
        self.next_order_id += 1;
        self.last_update_id += 1;

        let mut new_order = Order {
            id: order_id.clone(),
            price: input.price,
            quantity: input.quantity,
            user_id: input.user_id,
        };

        let mut filled_qty: u64 =0;
        let mut total_cost: u64 =0;

        match input.side{
            Side::Buy =>{
                let matching_price: Vec<u64> = self.asks
                    .keys()
                    .copied()
                    .take_while(|&ask_price| ask_price <= new_order.price)
                    .collect();

                for ask_price in matching_price {
                    if new_order.quantity == 0 {break;}

                    if let Some(queue) = self.asks.get_mut(&ask_price){
                        while let Some(maker) = queue.front_mut(){
                            if new_order.quantity == 0 {break;}
                            
                            let fill = new_order.quantity.min(maker.quantity);
                            filled_qty += fill;
                            total_cost += fill * ask_price;

                            new_order.quantity -= fill;
                            maker.quantity -= fill;

                            if maker.quantity == 0{
                                queue.pop_front();
                            }
                        }
                        if queue.is_empty(){
                            self.asks.remove(&ask_price);
                        }
                    }
                }
                if new_order.quantity > 0{
                    self.bids
                        .entry(new_order.price)
                        .or_insert_with(VecDeque::new)
                        .push_back(new_order);
                }
            }

            Side::Sell =>{
                let matching_prices : Vec<u64> = self.bids
                    .keys()
                    .copied()
                    .rev()
                    .take_while(|&bid_price | bid_price >= new_order.price)
                    .collect();

                for bid_price in matching_prices {
                    if new_order.quantity == 0 {break;}
                    if let Some(queue) = self.bids.get_mut(&bid_price){

                        while let Some(maker) = queue.front_mut(){
                            if new_order.quantity == 0 {break;}

                            let fill = new_order.quantity.min(maker.quantity);
                            filled_qty += fill;
                            total_cost += fill * bid_price;
                            new_order.quantity -= fill;
                            maker.quantity -= fill;

                            if maker.quantity == 0{
                                queue.pop_front();
                            }
                        }
                        if queue.is_empty(){
                            self.bids.remove(&bid_price);
                        }
                    }
                }
                if new_order.quantity > 0 {
                    self.asks
                        .entry(new_order.price)
                        .or_insert_with(VecDeque::new)
                        .push_back(new_order);
                }
            }

        }

        let average_price = 
        if filled_qty > 0 {
            total_cost as f64 / filled_qty as f64
        }else {
            0.0
        };

        let status = if filled_qty == 0 {
            "resting"
        }else if filled_qty < input.quantity {
            "partial"
        }else{
            "filled"
        };

        CreateOrderResponse {
            order_id,
            filled_qty,
            average_price,
            status: status.to_string(),
        }
    }

    pub fn delete_order(&mut self , input: DeleteOrderInput) -> DeleteOrderResponse {
        let map = match input.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };
        let order_id = input.order_id.clone();

        if let Some(queue) = map.get_mut(&input.price) {
            queue.retain(|o| o.id != input.order_id);
            if queue.is_empty() {
                map.remove(&input.price);
            }
            self.last_update_id += 1;
            return DeleteOrderResponse { order_id, success: true };
        }
        DeleteOrderResponse { order_id, success: false }
    }

    pub fn get_depth(&self) -> DepthResponse{
        let bids = self.bids
        .iter()
        .rev()
        .map(|(price,queue)| PriceLevel{
            price:*price,
            quantity:queue.iter().map(|o|o.quantity).sum(),
            order_count:queue.len()
        })
        .collect();
    let asks = self.asks
    .iter()
    .map(|(price, queue)| PriceLevel{
        price: *price,
        quantity : queue.iter().map(|o| o.quantity).sum(),
        order_count:queue.len(),
    })
    .collect();

    DepthResponse { 
        bids,
         asks,
          lastUpdatedId: self.last_update_id }
    }

}
