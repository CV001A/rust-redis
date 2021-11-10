#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// debug log
pub fn debugger(log: &str) {
    println!("debugger: {}", log)
}

/// info log
pub fn info(log: &str) {
    println!("info: {}", log)
}

/// warn log
pub fn warn(log: &str) {
    println!("warn: {}", log)
}

/// error log
pub fn error(log: &str) {
    println!("err: {}", log)
}