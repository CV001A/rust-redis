/// 命中缓存
pub struct DataResult {
    /// 执行结果编码，0为success
    pub code: i32,
    /// success时返回『success』，异常时返回原因描述
    pub message: String,
    /// 结果数据
    pub data: String,
}

/// 异常枚举
pub const Err_Code_Args_Validate: i32 = 400;