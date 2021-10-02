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
    use std::{any::Any, io::empty};
    use std::collections::HashSet;
    use r2d2::{Pool, ManageConnection};
    use r2d2_postgres::PostgresConnectionManager;
    use r2d2_postgres::postgres::NoTls;
    use postgres::types::{ToSql, Type, private::BytesMut};

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
        let manager = r2d2_postgres::PostgresConnectionManager::new("".parse().unwrap(), NoTls);
        let pool = r2d2::Pool::new(manager).unwrap();
        let mut conn = pool.get().unwrap();

        model!(
            User, TableUser: [
                id, i64, null=true, default=64;
                username, u32, null=false;
            ]
        );
        let mut table_user: TableUser<PostgresConnectionManager<NoTls>> = TableUser::new(&conn);
    }
    #[test]
    fn test_r2d2(){

        let manager = r2d2_postgres::PostgresConnectionManager::new("".parse().unwrap(), NoTls);

        let pool = r2d2::Pool::new(manager).unwrap();

        let mut conn = pool.get().unwrap();
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let tmp = String::from("Test").clone();
        let a: i16 = 13;
        let quer = String::from("Select * from test;");
        params.push(&tmp);
        params.push(&a);
        conn.execute(quer.as_str(), &params[..]).unwrap();
        // conn.batch_execute("").unwrap();
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
