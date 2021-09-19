pub trait DBType{
    fn to_db(&self) -> String;
}

impl DBType for i64 {
    fn to_db(&self) -> String {
        let mut s = self.to_string();
        return s
    }
}

