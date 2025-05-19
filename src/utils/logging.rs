use log::{LevelFilter, info};
use fern::Dispatch;
use chrono::Local;
use std::fs::OpenOptions;
use std::path::Path;

pub fn init_logger(log_path: &str) -> Result<(), fern::InitError> {
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(Path::new(log_path))?;

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{:?}][{}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S:%3f"),
                record.level(),
                std::thread::current().id(),          // 线程 ID
                record.target(),                      // 模块路径
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(LevelFilter::Debug) // 全局日志级别
        .chain(log_file)           // 输出到文件
        .chain(std::io::stdout())  // 同时输出到控制台（可选）
        .apply()?;

    info!("Logger initialized for {}", log_path);
    Ok(())
}