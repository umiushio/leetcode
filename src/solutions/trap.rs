use super::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n <= 2 { return 0; }

        let max_height = *height.iter().max().unwrap();
        let mut total = 0;
        let (mut first_max_index, mut candidate)= (0, 0);
        for i in 0..n {
            if height[i] == max_height {
                first_max_index = i;
                break;
            }
            if height[i] >= height[candidate] {
                candidate = i;
            } else {
                total += height[candidate] - height[i];
            }
            
        }

        candidate = n - 1;
        for i in (first_max_index..n-1).rev() {
            if height[i] >= height[candidate] {
                candidate = i;
            } else {
                total += height[candidate] - height[i];
            }
        }
        total
    }
}