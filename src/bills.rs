#[derive(Debug)]
pub struct Bill{
    pub name: String,
    pub amount: f64,
}

// 하나만 있을때 inner 종종 씀
pub struct Bills{
    inner: Vec<Bill>,
}

impl Bills{
    pub fn new() -> Self{
        Self {inner: vec![]}
    }

    pub fn add(&mut self, bill: Bill){
        self.inner.push(bill);
    }

    pub fn getAll(&self) -> &Vec<Bill>{
         &self.inner
    }
}