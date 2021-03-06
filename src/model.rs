use std::string::String;
use crate::{query};
use r2d2::{ManageConnection, PooledConnection};

pub trait DbModel<'a, T, M: ManageConnection>{
    fn new(conn:&'a mut PooledConnection<M>)->Self;
    fn table_create(&self)-> bool;
    // fn execute(&self) -> Vec<T>;
    fn select(&self) -> query::Query;
    fn fetch(&mut self, q: query::Query)->Vec<T>;
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
        $table_access: ident:
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
        use std::collections::HashSet;
        use std::string::String;
        use crate::DbModel;
        use crate::Column;
        use crate::query::{Query};
        use r2d2::{ManageConnection, PooledConnection};

        //Структура для хранения одной записи
        pub struct $table_name{
            pub columns: HashSet<String>,
            $(pub $column_name: Option<$column_type>,)+
        }

        //Структура для работы со всей таблицей
        pub struct  $table_access<'a, M>{
            columns: HashSet<String>,
            columns_list: String,
            connection: &'a mut PooledConnection<M>,
            $($column_name: Column<$column_type>,)+
        }
        impl<'a, M: ManageConnection> DbModel<'a, $table_name, M> for $table_access<'a, M>{
            fn new(connection: &'a mut PooledConnection<M>) -> $table_access{ //FIXME: Переделать на универсальный SQL клиент 
                let mut model = $table_access{
                    columns: HashSet::new(),
                    columns_list: String::new(),
                    connection: connection,
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
                let mut tmp_vec: Vec<String> = Vec::new();
                for column in model.columns.iter(){
                    tmp_vec.push(column.to_string())
                }
                model.columns_list.push_str(&tmp_vec.join(", ").to_owned());
                model
            }
            fn table_create(&self)-> bool{
                true
            }
            fn select(&self) -> Query{
                let mut query = Query::new();
                query.suffix_sql(format!("SELECT {lst} FROM {tbl} WHERE", lst=self.columns_list, tbl=stringify!($table_name)).to_string());
                query
            }
            fn fetch(&mut self, q: Query)->Vec<$table_name>{
                let mut result: Vec<$table_name> = Vec::new();
                let params = q.get_params();
                let res = self.connection.query(q.get_query().as_str(), &params[..]);
                match res{
                    Err(_)=>{},
                    Ok(rows)=>{
                        for row in rows{
                            result.push($table_name{
                                columns: self.columns.clone(),
                                $($column_name:Some(row.try_get(stringify!($column_name)).unwrap()),)+
                            })
                        }
                    }
                }
                result
            }
        }
    }
}
