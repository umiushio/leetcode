use std::sync::Once;
use crate::utils::logging::init_logger;

static INIT: Once = Once::new();

/// 初始化调试日志
pub fn setup_logging() {
    INIT.call_once(|| {
        init_logger("debug.log").expect("Failed to initialize logger");
    });
}

/// 单输入单输出测试宏
#[macro_export]
macro_rules! test_cases {
    ($mod:path, $func:path, [ $( ($input:expr, $expected:expr) ),* $(,)?]) => {
        paste::paste! {
            #[test]
            fn [<test_ $func>]() {
                $(
                    let result = $mod::$func($input);
                    assert_eq!(result, $expected);
                )*
            }
        }
    };
}

/// 多输入单输出测试宏
#[macro_export]
macro_rules! test_cases_multi {
    ($mod:path, $func:ident, [ $( ( ($($input:expr),*), $expected:expr ) ),* $(,)? ]) => {
        paste::paste! {
            #[test]
            fn [<test_ $func>]() {
                $(
                    let result = $mod::$func($($input),*);
                    assert_eq!(result, $expected);
                )*
            }
        }
    };
}