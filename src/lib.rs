#[macro_use]
mod model;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn model_macro(){
        let z: u128;
        let us: u64;
        model!(
            User[
                id, u128, null=true;
                username, u64;
            ]
        )
    }
}
