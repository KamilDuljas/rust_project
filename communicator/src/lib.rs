pub mod client;
pub mod network;

pub fn abc() {
    client::connect()
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::client;
    use super::add;
    

    #[test]
    fn it_works() {
        let result = add(2, 2);
        client::connect();
        assert_eq!(result, 4);
    }
}
