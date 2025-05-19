use super::Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        // 本题存在解的判定是gas和>=cost和
        if gas.iter().sum::<i32>() < cost.iter().sum::<i32>() {
            return -1;
        }
        let mut total = 0;
        let mut start = 0;
        for (i, (g, c)) in gas.iter().zip(cost.iter()).enumerate() {
            total += g - c;
            if total < 0 {
                start = i + 1;
                total = 0;
            }
        }
        start as i32

        // assert_eq!(gas.len(), cost.len());
        // let (mut l, mut r) = (0, gas.len());
        // let mut s = 0;
        // while l < r {
        //     if s >= 0 {
        //         s += gas[l] - cost[l];
        //         l += 1;
        //     } else {
        //         s += gas[r - 1] - cost[r - 1];
        //         r -= 1
        //     }
        // }
        // if s >= 0 {
        //     (r % gas.len()) as i32
        // } else {
        //     -1
        // }
    }
}