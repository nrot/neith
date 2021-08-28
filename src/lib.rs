#[macro_use]
mod model;
mod query;

use model::DbModel;
use model::Column;



#[cfg(test)]
mod tests {
    use std::io::empty;
    use std::collections::HashSet;

    fn print_type_of<T>(_: &T) {
        println!("Type: {}", std::any::type_name::<T>())
    }

    fn test_types(val: i128){
        print_type_of(&val);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn model_macro(){
        model!(
            User, Table_User[
                id, u128, null=true, default=64;
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
        let mut hs = HashSet::new();
        hs.insert(1234);
        let var: i16 = 32;
        test_types(var as i128);
    }
}
