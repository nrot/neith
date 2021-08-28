use std::collections::HashMap;
use std::string::String;
use std::collections::hash_set::Union;

fn to_where<I: i8 + i16 + i32 + i64 + i128>(value: I,   ) -> &String{
    let mut re = String::new();
    &re
}

pub trait DbModel{
    fn new()->Self;
    fn filter(&self, query: HashMap<String, String>) -> &Self;
    fn sql(&self)->&String;
    // fn execute(&self) -> Vec<Self>;
    fn select(&self) -> &Self;
}

pub struct Column<T>{
    pub name: String,
    pub rtype: String,
    pub default: Option<T>,
    pub var: Option<T>,
    pub pk: bool,
    pub null: bool,
    pub unique: bool,
    pub readonly: bool
}

macro_rules! attribute {
    ($data:ident, $field:ident) => {
        $data.$field
    };
}

macro_rules! def_or_none {
    ($def:expr) => {$def};
    (, $def:expr) => {$def};
    ($arg:expr, $def:expr) => {$arg};
}

macro_rules! arg_or_none{
    ()=>{None};
    ($arg:expr)=>{$arg};
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
        use std::collections::HashSet;
        use std::string::String;
        use crate::DbModel;
        use crate::Column;
        pub struct $table_name{
            filter: String,
            raw_sql: String,
            columns: HashSet<String>,
            $($column_name: Column<$column_type>,)+
        }
        impl DbModel for $table_name{
            fn new() -> $table_name{
                let mut model = $table_name{
                    filter: String::new(),
                    raw_sql: String::new(),
                    columns: HashSet::new(),
                    $($column_name: Column{
                        name: String::from(stringify!($column_name)),
                        rtype: String::from(stringify!($column_type)),
                        default: arg_or_none!($($column_default)?),
                        var: None,
                        pk: def_or_none!($($column_pk)?, false),
                        null: def_or_none!($($column_null)?, false),
                        unique: def_or_none!($($column_unique)?, false),
                        readonly: def_or_none!($($column_readonly)?, false)
                    },)+
                };
                $(model.columns.insert(String::from(stringify!($column_name)));)+
                model
            }
            fn filter(&self, query: HashMap<String, String>) -> &$table_name{
                for key in query.keys(){

                    if !self.columns.contains(key){
                        continue;
                    }
                }
                self
            }
            fn select(&self) -> &Self{
                self
            }
            fn sql(&self) -> &String{
                &self.raw_sql
            }
        }
    }
}
