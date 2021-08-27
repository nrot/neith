use std::collections::HashMap;
use std::string::String;

macro_rules! column_struct{
    (
        ($column_name:ident),
        ($column_type:ty)
    )=>{
        $column_name: $column_type
    }
}

pub trait DbModel{
    fn new()->Self;
    fn filter(&self, query: HashMap<String, String>) -> &Self;
    fn sql(&self)->String;
    // fn execute(&self) -> Vec<Self>;
    fn select(&self) -> &Self;
}

#[macro_export]
macro_rules! model{
    (
        $table_name:ident
        [$(
            $column_name:ident,
            $column_type:ty $(,)?
            $(, pk=$column_pk:expr)?
            $(, null=$column_null:expr)?
            $(, default=$column_default:expr)?
            $(, unique=$column_unique:expr)?
            $(, readonly=$column_readonly:expr)? $(,)?
        ;)+]
    ) => {
        use std::collections::HashMap;
        use std::string::String;
        use crate::DbModel;
        pub struct $table_name{
            filter: String,
            raw_sql: String,
            columns: Vec<String>,
            $($column_name: Option<$column_type>,)+
        }
        impl DbModel for $table_name{
            fn new() -> $table_name{
                let mut model = $table_name{
                    filter: String::new(),
                    raw_sql: String::new(),
                    columns: Vec::new(),
                    $($column_name: None,)+
                };
                $(model.columns.push(String::from(stringify!($column_name)));)+
                model
            }
            fn filter(&self, query: HashMap<String, String>) -> &$table_name{
                for key in query.keys(){
                }
                self
            }
            fn select(&self) -> &Self{
                self
            }
            fn sql(&self) -> String{
                self.raw_sql
            }
        }
    }
}
