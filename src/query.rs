use std::string::String;
use std::str::CharIndices;

use crate::model::Column;
use postgres::types::{ToSql, Type};

pub enum Operation{
    Eq,
    Gt,
    Lw,
    Exact,
    IExact,
    Contains,
    IsNull
}

pub struct Query<'a>{
    counts: i8,
    sql: String,
    values: Vec<&'a(ToSql + Sync)>
}

pub struct Condition<T: ToSql + Sync>{
    sql: String,
    value: T
}

pub trait Protection{
    fn protected(self)->Self;
}

impl Protection for String{
    fn protected(self)->String{
        String::from(format!("{s}", s=&self).to_string())
    }
}


impl<T: ToSql + Sync> Condition<T>{
    
    fn custom_compare(column: String, value: T, symbol: &'static str)->Condition<T>{
        Condition{
            sql: format!("{column} {symbol} $1", column = column, symbol=symbol.trim()).to_string(),
            value: value
        }
    }
    fn eq(column: Column<T>, value: T)->Condition<T>{
        Condition::custom_compare(column.name, value, "=")
    }
    fn gt(column: Column<T>, value: T)->Condition<T>{
        Condition::custom_compare(column.name, value, "<")
    }
    fn lt(column: Column<T>, value: T) ->Condition<T>{
        Condition::custom_compare(column.name, value, ">")
    }
}

impl<'a> Query<'a>{
    pub fn new()->Self{
        Query{
            counts: 0, 
            sql: String::new(),
            values: Vec::new()
        }
    }
    fn custom_add<T: ToSql + Sync>(&mut self, cond: &'a Condition<T>, symbol: &'static str)->&Self{
        if self.counts != 0 {
            self.sql.push_str(format!(" {symbol} ", symbol=&symbol.trim()).as_str());
        };
        self.counts += 1;
        self.sql.push_str(&cond.sql.replace("$1", &format!("${c}", c=self.counts)));
        self.values.push(&cond.value);
        self
    }
    pub fn or<T: ToSql + Sync>(&mut self, cond: &'a Condition<T>)->&Self{
        self.custom_add(cond, "OR")
    }
    pub fn and<T: ToSql + Sync>(&mut self, cond: &'a Condition<T>)->&Self{
        self.custom_add(cond, "AND")
    }
    fn custom_add_query(&mut self, query: Query, symbol: &'static str)->&Self{
        if query.sql.is_empty(){
            return self;
        }
        if self.counts != 0 {
            self.sql.push_str(format!(" {symbol} ", symbol=symbol.trim()).as_str());
        };
        let mut tmp_query = query.sql.clone();
        for c in std::iter::repeat(query.counts) {
            tmp_query = tmp_query.replace(&format!("${c}", c=c), &format!("${c}", c=c+self.counts));
        }
        self.sql.push_str(&tmp_query);
        self.counts += query.counts;
        self
    }
    pub fn and_query(&mut self, query: Query)->&Self{
        self.custom_add_query(query, "AND")
    }
    pub fn or_query(&mut self, query: Query)->&Self{
        self.custom_add_query(query, "OR")
    }
    pub fn suffix_sql(&mut self, sql: String)->&Self{
        self.sql.insert_str(0, &format!("{s} ", s=&sql.trim()));
        self
    }
    pub fn get_query(&self)->String{
        self.sql.clone()
    }
    pub fn get_params(&self)->Vec<&'a(ToSql + Sync)>{
        self.values.clone()
    }
}
