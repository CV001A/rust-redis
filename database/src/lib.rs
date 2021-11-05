#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct Database{

}

impl Database {
    pub fn new()->Database{
        Database{}
    }
}