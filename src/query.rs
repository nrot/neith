use std::string::String;
use std::str::CharIndices;

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

pub struct Condition<T>{
    pub column: String,
    pub value: T,
    operator: Operation,
    sql: String
}

trait Protection{
    fn protected(self)->Self;
}

impl Protection for i128{
    fn protected(self)->Self{
        self
    }
}

impl Protection for String{
    fn protected(self)->String{
        self
    }
}

impl <T: Clone + Protection + Into<String>> Condition<T>{
    fn eq(column: String, value: T)->Condition<T>{
        Condition {
            column: column.clone(),
            value: value.clone().protected(),
            operator: Operation::Eq,
            sql: format!("{column} = {value}", column = column, value = value.into()).to_string()
        }
    }
}

impl Query{
    fn new()->Self{
        Query{ sql: String::new()}
    }
    fn or(&mut self, cond: Condition<i128>)->&Self{

        self
    }
}
