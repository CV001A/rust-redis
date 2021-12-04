use std::collections::HashMap;

use crate::command;
use crate::DataResult;

/// 内存数据存储结构
pub struct Database {
    mapping: HashMap<String, String>,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database{
            mapping: self.mapping.clone()
        }
    }
}

impl Database {
    pub fn new() -> Database {
        Database {
            mapping: HashMap::new(),
        }
    }

    pub fn handle_command(&mut self, command: command::Command) -> DataResult {
        command.op.handle(self,command.args)
    }

    pub(crate) fn get(&self, key: String) -> Option<String> {
        let result = self.mapping.get(&key);
        match result {
            None => None,
            Some(T) => Some(T.clone()),
        }
    }

    pub(crate) fn set(&mut self, key: String, value: String) {
        self.mapping.insert(key, value);
    }
}
