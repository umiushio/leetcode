use super::Solution;
// use once_cell::sync::Lazy;
// use std::sync::Mutex;

// // 利用外部crate实现线程安全的缓存优化
// static CACHE: Lazy<Mutex<Vec<Vec<String>>>> = Lazy::new(|| {
//     let mut array: Vec<Vec<String>> = Vec::new();
//     array.push(vec![String::from("")]);
//     Mutex::new(array)
// });

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        assert!(n >= 1 && n <= 8);

        // 线程不安全的缓存加速方法
        static mut CACHE : Option<Vec<Vec<String>>> = None;

        unsafe {
            // 直接用原始指针避免引用
            let cache_ptr = &raw mut CACHE as *mut Option<Vec<Vec<String>>>;
            let mut cache = match (*cache_ptr).take() {
                Some(c) => c,
                None => vec![vec![String::from("")]],
            };

            // 动态扩容到n
            for i in cache.len()..=n as usize {
                let mut current = Vec::new();
                for j in 0..i {
                    for s in &cache[j] {
                        for t in &cache[i - j - 1] {
                            current.push(format!("({}){}", t, s));
                        }
                    }
                }
                cache.push(current);
            }
            let result = cache[n as usize].clone();
            // 放回缓存
            *cache_ptr = Some(cache);
            result
        }

        // // crates实现缓存加速的写法
        // let mut cache = CACHE.lock().unwrap();
        // while cache.len() <= n as usize {
        //     let i = cache.len();
        //     let mut current = Vec::new();
        //     for j in 0..i {
        //         for s in &cache[j] {
        //             for t in &cache[i - j - 1] {
        //                 current.push(format!("({}){}", t, s));
        //             }
        //         }
        //     }
        //     cache.push(current);
        // }
        // cache[n as usize].clone()

        // // 没有缓存优化的写法
        // let mut array: Vec<Vec<String>> = Vec::new();
        // array.push(vec![String::from("")]);
        // for i in 1..=n as usize {
        //     let mut current = Vec::new();
        //     for j in 0..i {
        //         for s in &array[j] {
        //             for t in &array[i - j - 1] {
        //                 current.push(format!("({}){}", t, s));
        //             }
        //         }
        //     }
        //     array.push(current);
        // }
        // array[n as usize].clone()
    }
}