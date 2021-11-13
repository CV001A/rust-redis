/// 命令相关

/// 命令
pub struct Command {
    /// 操作符
    pub op: Op,
    /// 参数列表
    pub args: Vec<String>,
}

/// op 枚举值
pub enum Op {
    /// get操作
    Get,
    /// set操作
    Set,
}