use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen; 

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Ressource {
    name: String, 
    yearly_rent: u128, // in 1/10^6
    currency: String, // tbd: the id of the fungible token contract  
    profit_margin: u32, // in 1/10^6
    min_duration: u64, // in seconds
    // usage_factors: Vec  // not right now, too complicated

    // tbd demands
//    demand_per_second: f64, 
//    month_demands: [u16; 12], // 1/10^4
//    weekday_demands: [u16; 7], 
//    hour_demands: [u16; 24]
}

#[near_bindgen]
impl Ressource {
    #[init]
    pub fn new(
        _name: String, 
        _yearly_rent: u128, 
        _currency: String, 
        _profit_margin: u32, 
        _min_duration: u64
        
    ) -> Self {
        Self {
            name: _name, 
            yearly_rent: _yearly_rent, 
            currency: _currency, 
            profit_margin: _profit_margin, 
            min_duration: _min_duration
        }
    }

    pub fn make_booking(
   //     _from: Date
   //     _until: Date
    ){
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn set_name(&mut self, _name: String) {
        self.name = _name;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_something() {
        assert!(true); 
    }
}
