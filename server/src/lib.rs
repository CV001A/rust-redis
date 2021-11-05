#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// the redis server
pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }
    pub fn run(&self) {
        println!("begin running!")
    }
}