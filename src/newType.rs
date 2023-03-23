#[derive(Debug, Copy, Clone)]
pub struct NeverZero(i32);

impl NeverZero{
    pub fn new(i: i32) -> Result<Self, String> {
        if i==0 {
            Err("Can not be zero".to_owned())
        }else {
            Ok(Self(i))
        }
    }
}

pub fn divide(a: i32, b: NeverZero) -> i32{
    let b = b.0;
    a / b
}