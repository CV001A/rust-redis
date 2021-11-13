use std::error::Error;
use std::io::BufReader;
use std::net::{
    TcpListener,
    TcpStream,
};

use net2::TcpBuilder;

use database::{Database, DataResult};
use logger;

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
            Ok(_) => logger::info("listen success"),
            Err(err) => panic!("lister fail, err:"),
        }

        logger::info("begin running!");
    }

    /// 网络监听
    fn listen(&self) -> std::io::Result<()> {
        let tcp = TcpBuilder::new_v4()?;
        tcp.reuse_address(true)?;
        let listener = tcp.bind("127.0.0.1:8080")?.listen(511)?;

        let database2 = self.database;
        let join = std::thread::spawn(move || {
            logger::info("begin to handle incoming stream");
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                logger::info("receive stream, begin to handle");
                let command = parse_command(&mut stream);    // todo
                let data_result = database2.handle_command(command);   // todo
                handle_response(stream, data_result); // todo
            }
        });
        join.join();

        return Ok(());
    }
}

/// 将缓存命中结果，转换为二进制流
fn handle_response(stream: TcpStream, p0: database::DataResult) {
    todo!()
}

/// 将输入流转换为command 实例
fn parse_command(stream: &mut TcpStream) -> database::Command {
    // let mut stream = BufReader::new(stream);
    todo!()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}