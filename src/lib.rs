use std::string::String;

#[macro_use]
mod model;

use model::DbModel;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn model_macro(){
        let z: u128;
        let us: u64;
        model!(
            User[
                id, u128, null=true;
                username, u64;
            ]
        );
    }
    #[test]
    fn some_testing(){
        let s = String::new();
        let mut v: Vec<String> = Vec::new();
        v.push(String::from("Some test"));
        struct Test{
            var: String
        }
        let mut tst = Test{var: String::new()};
        tst.var.push_str("dawd");
    }
}
