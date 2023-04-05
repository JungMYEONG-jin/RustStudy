pub enum Hero{
    Fast,
    Info {name: String, secret: String},
}

impl Hero{
    pub fn get_info(&self){
        match self{
            Hero::Fast => println!("i am fast"),
            Hero::Info {name, secret} => {
                println!("name is: {0}, secret is : {1}", name, secret);
            },
        }
    }
}

#[cfg(test)]
mod test{
    use crate::sample_enum::Hero;

    #[test]
    fn get_info_test(){
        let f = Hero::Fast;
        let info = Hero::Info {name: "aa".to_owned(), secret: "bb".to_owned()};

        f.get_info();
        info.get_info();
    }
}