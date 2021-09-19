#[macro_use]
mod model;
mod query;
mod types;

use model::DbModel;
use model::Column;
use r2d2::Pool;
use r2d2::ManageConnection;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::postgres::NoTls;



#[cfg(test)]
mod tests {
    use std::io::empty;
    use std::collections::HashSet;
    use r2d2::{Pool, ManageConnection};
    use r2d2_postgres::postgres::NoTls;

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
    fn test_r2d2(){

        let manager = r2d2_postgres::PostgresConnectionManager::new("".parse().unwrap(), NoTls);

        let pool = r2d2::Pool::new(manager).unwrap();

        let mut conn = pool.get().unwrap();
        let a: i16 = 13;
        conn.execute("", &[&a]).unwrap();
        conn.batch_execute("").unwrap();
    }

    trait TestTrait{
        fn to_val(&self)->String;
    }

    impl TestTrait for i64{
        fn to_val(&self) -> String {
            let mut s: String = self.to_string();
            return s
        }
    }

    #[test]
    fn test_trait(){
        let a: i64 = 32;
        println!("{}", a.to_val());
    }

}
