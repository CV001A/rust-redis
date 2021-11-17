# logger 日志框架

## 提供能力

提供日志异步写入能力，目前只支持std::out和本地文件输出.

## 设计实现

采用mpsc 模式，日志异步写入到channel，由日志内部的单实例线程进行消费写出到日志文件。
日志级别如下：
1. debugger
2. info
3. warn
4. error

## 使用方式

```rust
use logger;

pub fn main() {
    logger::debugger("process begin start");
    logger::info("hello world");
    logger::warn("warning");
    logger::error("error happend");
}
```

## todo 优化