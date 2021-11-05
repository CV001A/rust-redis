use std::error::Error;
use std::net::TcpListener;

use net2::TcpBuilder;

use database::Database;

/// the redis server
pub struct Server {
    /// 内存数据库，数据存储结构体
    database: Database,

}

impl Server {
    /// 生成server实例，初始化内存
    pub fn new() -> Server {
        Server {
            database: Database::new()
        }
    }

    /// 启动监听
    pub fn run(&self) {
        let result = self.listen();
        match result {
            Ok(_) => println!("listen success"),
            Err(err) => panic!("lister fail, err:"),
        }

        println!("begin running!")
    }

    /// 网络监听
    fn listen(&self) -> std::io::Result<()> {
        let tcp = TcpBuilder::new_v4()?;
        tcp.reuse_address(true)?;
        let listener = tcp.bind("127.0.0.1:8080")?.listen(511)?;
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}