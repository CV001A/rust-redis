use crate::{Database, DataResult};
use crate::dataresult;
use crate::Op::Get;

/// 命令相关

/// 命令
pub struct Command {
    /// 操作符
    pub op: Op,
    /// 参数列表
    pub args: Vec<String>,
}

impl Command {
    /// 将文本协议转换为命令对象
    /// 参考文档：http://redisdoc.com/topic/protocol.html
    /// 协议的一般模式为
    /// ```txt
    ///*<参数数量> CR LF
    /// $<参数 1 的字节数量> CR LF
    /// <参数 1 的数据> CR LF
    /// ...
    /// $<参数 N 的字节数量> CR LF
    /// <参数 N 的数据> CR LF
    /// ```
    /// demo如下：
    /// ```txt
    /// *3
    /// $3
    /// SET
    /// $5
    /// mykey
    /// $7
    /// myvalue
    /// ```
    pub fn parse(content: String) -> Result<Command, ()> {
        let commands: Vec<&str> = content.split("\r\n").collect();
        if commands.is_empty() {
            logger::error("receive commands is empty");
            return Err(());
        }
        let param_count_line = *commands.get(0).unwrap();
        if !param_count_line.matches("^\\*\\d+$") {
            logger::error("param count line is not match ^\\*\\d+$ ");
            return Err(())
        }

        let param_count_str=param_count_line.split("*").next().unwrap();

    }
}

impl Command {}

/// op 枚举值
pub enum Op {
    /// get操作
    Get,
    /// set操作
    Set,
}

impl Op {
    pub(crate) fn handle(&self, database: &mut Database, args: Vec<String>) -> DataResult {
        match self {
            Get => {
                return self.handle_get(database, args);
            }
            Set => {
                return self.handle_set(database, args);
            }
        }
    }


    fn handle_get(&self, database: &mut Database, args: Vec<String>) -> DataResult {
        if args.len() != 1 {
            logger::error("args is not right, expect len size 1");
            return DataResult {
                code: dataresult::Err_Code_Args_Validate,
                message: "args not right".to_string(),
                data: "".to_string(),
            };
        }

        let result: String = "".to_string();
        let key = args.get(0).unwrap();
        let result = match database.get(key.clone()) {
            Some(val) => val,
            None => "".to_string(),
        };
        DataResult {
            code: 0,
            message: "success".to_string(),
            data: result,
        }
    }

    fn handle_set(&self, database: &mut Database, args: Vec<String>) -> DataResult {
        if args.len() != 2 {
            logger::error("args is not right, expect len size 2");
            return DataResult {
                code: dataresult::Err_Code_Args_Validate,
                message: "args not right".to_string(),
                data: "".to_string(),
            };
        }

        let key = args.get(0).unwrap();
        let val = args.get(1).unwrap();
        database.set(key.clone(), val.clone());

        DataResult {
            code: 0,
            message: "success".to_string(),
            data: "".to_string(),
        }
    }
}
