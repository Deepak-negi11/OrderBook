use std::{collections::HashMap, env::current_exe};


pub struct BalanceManager {
    pub balance : HashMap<String, HashMap<String ,u64>>,
    
}



impl BalanceManager {
    fn new()-> Self{
        Self {
            balance:HashMap::new()
        }
    }

    fn get_balance(&self , user_id: &str , currency:&str )->u64 {
        self.balance
            .get(user_id)
            .and_then(|currenies| currenies.get(currency))
            .copied()
            .unwrap()

    } 

    pub fn has_enough(&self , user_id: &str , currency:&str , amount:u64)->bool{
        self.get_balance(user_id, currency) >= amount
    }

    pub fn add_balance(&mut self , user_id: &str, currency:&str,amount:u64){
        self.balance
            .entry(user_id.to_string())
            .or_insert_with(HashMap::new)
            .entry(currency.to_string())
            .or_insert(0);

        *self.balance
            .get_mut(user_id).unwrap()
            .get_mut(currency).unwrap() += amount ;
    }

    pub fn deduct_balance(&mut self, user_id: &str, currency: &str, amount: u64) -> Result<(), String> {
        if !self.has_enough(user_id, currency, amount) {
            return Err(format!("{} has insufficient {}", user_id, currency));
            
        }
        *self.balances
            .get_mut(user_id).unwrap()
            .get_mut(currency).unwrap() -= amount;

        Ok(())  
    }
}
