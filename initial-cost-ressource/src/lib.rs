use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen; 

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Ressource {
    name: String, 
}

#[near_bindgen]
impl Ressource {
    #[init]
    pub fn new(
        _name: String, 
    ) -> Self {
        Self {
            name: _name, 
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
