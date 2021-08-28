#[macro_use]
mod model;
use model::DbModel;
use model::Column;

#[cfg(test)]
mod tests {
    use std::io::empty;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn model_macro(){
        model!(
            User[
                id, u128, null=true;
                username, u64, null=false;
            ]
        );
    }
    #[test]
    fn some_testing(){
        let s = String::new();
        let mut v: Vec<String> = Vec::new();
        v.push(String::from("Some test"));
        struct Test{
            var: String,
            o: Option<i64>
        }
        let mut tst = Test{var: String::new(), o: arg_or_none!()};
        tst.var.push_str("dawd");
    }
}
