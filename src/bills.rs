use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Bill{
    pub name: String,
    pub amount: f64,
}

// 하나만 있을때 inner 종종 씀
pub struct Bills{
    pub(crate) inner: HashMap<String, Bill>,
}

impl Bills{
    pub fn new() -> Self{
        Self {inner: HashMap::new()}
    }

    pub fn add(&mut self, bill: Bill){
        self.inner.insert(bill.name.clone(), bill);
    }


    pub fn remove(&mut self, str: &str) -> bool{
        self.inner.remove(str).is_some()
    }

    pub fn getAll(&self) -> Vec<Bill>{
         let mut bills = vec![];
        for bill in  self.inner.values(){
            bills.push(bill.clone());
        }
        bills
    }

    pub fn edit(&mut self, str: &str, amount: f64) -> bool{
        match self.inner.get_mut(str){
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}