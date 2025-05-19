use super::Solution;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        assert!(!points.is_empty());

        let mut sorted_points = points;
        sorted_points.sort_unstable_by_key(|p| p[1] );
        let mut shots = 1;
        let mut end = sorted_points[0][1];
        for point in sorted_points.iter().skip(1) {
            if point[0] > end {
                shots += 1;
                end = point[1];
            } 
        }
        shots
    }
}