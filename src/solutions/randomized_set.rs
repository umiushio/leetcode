use rand::prelude::*;
use std::collections::HashMap;

pub struct RandomizedSet {
    nums: Vec<i32>,
    index_map: HashMap<i32, usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    pub fn new() -> Self {
        Self {
            nums: Vec::new(),
            index_map: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, val: i32) -> bool {
        if self.index_map.contains_key(&val) { 
            return false;
        }
        self.index_map.insert(val, self.nums.len());
        self.nums.push(val);
        true
    }
    
    pub fn remove(&mut self, val: i32) -> bool {
        if let Some(&index) = self.index_map.get(&val) {
            let last = *self.nums.last().unwrap();
            self.nums[index] = last;
            self.index_map.insert(last, index);
            self.nums.pop();
            self.index_map.remove(&val);
            true
        } else {
            false
        }

        // if !self.index_map.contains_key(&val) {
        //     return false;
        // }
        // let i = self.index_map[&val];
        // self.nums[i] = *self.nums.last().unwrap();
        // self.index_map.insert(self.nums[i], i);
        // self.nums.pop();
        // self.index_map.remove(&val);
        // true
    }
    
    pub fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        *self.nums.choose(&mut rng).unwrap()
    }
}
