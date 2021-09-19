use std::string::String;
use std::str::CharIndices;

use crate::model::Column;

pub enum Operation{
    Eq,
    Gt,
    Lw,
    Exact,
    IExact,
    Contains,
    IsNull
}

pub struct Query{
    sql: String
}

pub type Condition = String;
// pub struct Condition{
//     sql: String
// }

pub trait Protection{
    fn protected(self)->Self;
}

impl Protection for String{
    fn protected(self)->String{
        String::from(format!("{s}", s=&self).to_string())
    }
}


pub trait Compare{
    fn custom_compare(column: String, value: String, symbol: &'static str)->Condition;
    fn eq_str(column: String, value: String)->Condition;
    fn eq<T:Clone + Protection + Into<String>>(column: Column<T>, value: T)->Condition;
    fn gt_str(column: String, value: String)->Condition;
    fn gt<T:Clone + Protection + Into<String>>(column: Column<T>, value: T)->Condition;
    fn lt_str(column: String, value: String)->Condition;
    fn lt<T:Clone + Protection + Into<String>>(column: Column<T>, value: T)->Condition;
}

impl Compare for Condition{
    fn custom_compare(column: String, value: String, symbol: &'static str)->Condition{
        Condition::from(format!("{column} {symbol} {value}", column = column, symbol=symbol.trim(), value = value ).to_string())
    }
    fn eq_str(column: String, value: String)->Condition{
        Condition::custom_compare(column, value, "=")
    }
    fn eq<T:Clone + Protection + Into<String>>(column: Column<T>, value: T)->Condition{
        Condition::eq_str(column.name, value.into())
    }
    fn gt_str(column: String, value: String)->Condition{
        Condition::custom_compare(column, value, "<")
    }
    fn gt<T:Clone + Protection + Into<String>>(column: Column<T>, value: T)->Condition{
        Condition::gt_str(column.name, value.into())
    }
    fn lt_str(column: String, value: String) ->Condition {
        Condition::custom_compare(column, value, "<")
    }
    fn lt<T:Clone + Protection + Into<String>>(column: Column<T>, value: T) ->Condition {
        Condition::lt_str(column.name, value.into())
    }
}

impl Query{
    fn new()->Self{
        Query{ sql: String::new()}
    }
    fn custom_add(&mut self, cond: Condition, symbol: &'static str)->&Self{
        if !self.sql.is_empty() {
            self.sql.push_str(format!(" {symbol} ", symbol=&symbol.trim()).as_str());
        };
        self.sql.push_str(&cond);
        self
    }
    fn or(&mut self, cond: Condition)->&Self{
        self.custom_add(cond, "OR")
    }
    fn and(&mut self, cond: Condition)->&Self{
        self.custom_add(cond, "AND")
    }
    fn custom_add_query(&mut self, query: Query, symbol: &'static str)->&Self{
        if !self.sql.is_empty() {
            self.sql.push_str(format!(" {symbol} ", symbol=symbol.trim()).as_str());
        };
        self.sql.push_str(query.sql.as_str());
        self
    }
    fn and_query(&mut self, query: Query)->&Self{
        self.custom_add_query(query, "AND")
    }
    fn or_query(&mut self, query: Query)->&Self{
        self.custom_add_query(query, "OR")
    }
}
