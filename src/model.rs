use std::collections::HashMap;
use std::string::String;
use std::collections::hash_set::Union;
use crate::{query, types};
use postgres::Client;

pub trait DbModel<T>{
    fn new(conn:Client)->Self;
    // fn filter(&self, query: HashMap<String, String>) -> T;
    fn sql(&self)->&String;
    // fn execute(&self) -> Vec<Self>;
    fn select(&self) -> &Self;
}

pub struct Column<T>{
    pub name: String,
    pub rtype: String,
    pub default: Option<T>,
    // pub var: Option<T>,
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
    ($arg:expr)=>{Some($arg)};
}

#[macro_export]
macro_rules! model{
    (
        $table_name:ident,
        $table_access: ident
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
        use std::clone;
        use std::string::String;
        use crate::DbModel;
        use crate::Column;
        use postgres::{Client};

        //Структура для хранения одной записи
        pub struct $table_name{
            columns: HashSet<String>,
            $($column_name: Option<$column_type>,)+
        }

        //Структура для работы со всей таблицей
        pub struct  $table_access{
            filter: String,
            raw_sql: String,
            columns: HashSet<String>,
            $($column_name: Column<$column_type>,)+
        }
        impl DbModel<$table_name> for $table_access{
            fn new(mut connection: Client) -> $table_access{ //FIXME: Переделать на универсальный SQL клиент 
                let mut model = $table_access{
                    filter: String::new(),
                    raw_sql: String::new(),
                    columns: HashSet::new(),
                    $($column_name: Column{
                        name: String::from(stringify!($column_name)),
                        rtype: String::from(stringify!($column_type)),
                        default: arg_or_none!($($column_default)?),
                        pk: def_or_none!($($column_pk)?, false),
                        null: def_or_none!($($column_null)?, false),
                        unique: def_or_none!($($column_unique)?, false),
                        readonly: def_or_none!($($column_readonly)?, false)
                    },)+
                };
                $(model.columns.insert(String::from(stringify!($column_name)));)+
                connection.batch_execute("");
                model
            }
            // fn filter(&self, column: String, value: DBType) -> $table_name{
            //     let mut re = $table_name{
            //         columns: self.columns.clone(),
            //         $($column_name: arg_or_none!($($column_default)?),)+
            //     };
            //     re
            // }
            fn select(&self) -> &Self{
                self
            }
            fn sql(&self) -> &String{
                &self.raw_sql
            }
        }
    }
}
