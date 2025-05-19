// use log::{debug, info};

use std::collections::HashMap;

pub struct LRUCache {
    cache_map: HashMap<i32, (i32, usize)>,
    cache_order: Vec<(usize, usize, i32)>,  //[before, next, data]
    cache_capacity: usize,
    cache_len: usize,
    cache_begin: usize,
    cache_end: usize,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        assert!(capacity > 0);

        Self {
            cache_map: HashMap::new(),
            cache_order: vec![(0, 0, 0); capacity as usize],
            cache_capacity: capacity as usize,
            cache_len: 0,
            cache_begin: 0,
            cache_end: 0
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        // debug!("get key: {}", key);
        if let Some(&(value, order)) = self.cache_map.get(&key) {
            self.touch(order);
            
            value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        // debug!("put key: {}, value: {}", key, value);
        let mut update_order;
        if let Some((val, order)) = self.cache_map.get_mut(&key) {
            *val = value;
            update_order = *order;
        } else {
            update_order = self.cache_len;
            if update_order == self.cache_capacity {
                update_order = self.cache_begin;
                self.cache_map.remove(&self.cache_order[update_order].2);   
            }
            self.cache_map.insert(key, (value, update_order));
            self.cache_order[update_order].2 = key;
        }
        // debug!("cache_map: {:?}", self.cache_map);
        self.touch(update_order);
    }

    fn delete(&mut self, order: usize) {
        if order == self.cache_begin {
            self.cache_begin = self.cache_order[order].1;
        } else {
            let (front, back, _) = self.cache_order[order];
            self.cache_order[front].1 = back;
            self.cache_order[back].0 = front;
        }
    }

    fn push(&mut self, order: usize) {
        self.cache_order[self.cache_end].1 = order;
        self.cache_order[order].0 = self.cache_end;
        self.cache_end = order;
    }

    fn touch(&mut self, order: usize) {
        if order == self.cache_len {
            self.cache_len += 1;
            self.push(order);
        } else if order != self.cache_end {
            self.delete(order);
            self.push(order);
        }
        // debug!("cache_order: {:?}", self.cache_order);
        // debug!("cache_begin: {}, cache_end: {}", self.cache_begin, self.cache_end);
    }
}