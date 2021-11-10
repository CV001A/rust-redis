#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// 内存数据存储结构
pub struct Database {}

impl Clone for Database {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Copy for Database {}

/// 命令
pub struct Command {}

/// 命中缓存
pub struct DataResult {}

impl Database {
    pub fn new() -> Database {
        Database {}
    }

    pub fn handle_command(&self, command: Command) -> DataResult {
        todo!()
    }
}
