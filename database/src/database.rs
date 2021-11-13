use std::collections::HashMap;

use crate::command;
use crate::DataResult;

/// 内存数据存储结构
pub struct Database {
    mapping: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            mapping: HashMap::new(),
        }
    }

    pub fn handle_command(&self, command: command::Command) -> DataResult {
        todo!()
    }
}