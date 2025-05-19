use super::*;
use paste::paste;

/// 通用断言宏，适合单输入单输出的测试
macro_rules! test_cases {
    ($func:ident, [ $( ($input:expr, $expected:expr) ),* $(,)?]) => {
        paste! {
            #[test]
            fn [<test_ $func>]() {
                $(
                    let result = Solution::$func($input);
                    assert_eq!(result, $expected);
                )*
            }
        }
    };
}

/// 通用断言宏，适合多输入单输出的测试
macro_rules! test_cases_multi {
    ($func:ident, [ $( ( ($($input:expr),*), $expected:expr ) ),* $(,)? ]) => {
        paste! {
            #[test]
            fn [<test_ $func>]() {
                $(
                    let result = Solution::$func($($input),*);
                    assert_eq!(result, $expected);
                )*
            }
        }
    };
}

#[test]
fn test_merge() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
}

#[test]
fn test_remove_element() {
    let cases = [
        // (输入数组, val, 期望长度, 期望结果)
        (vec![3, 2, 2, 3], 3, 2, vec![2, 2]),
        (vec![0, 1, 2, 2, 3, 0, 4, 2], 2, 5, vec![0, 0, 1, 3, 4]),
        (vec![], 1, 0, vec![]),
        (vec![1, 1, 1], 1, 0, vec![]),
    ];

    for (mut nums, val, expected_len, expected_nums) in cases {
        let len = Solution::remove_element(&mut nums, val);
        assert_eq!(len, expected_len);
        let mut result = nums[..len as usize].to_vec();
        result.sort_unstable();
        assert_eq!(result, expected_nums);
    }
}

#[test]
fn test_remove_duplicates() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let len = Solution::remove_duplicates(&mut nums);
    assert_eq!(len, 5);
    assert_eq!(nums[..len as usize], [0, 1, 2, 3, 4])
}

#[test]
fn test_remove_duplicates2() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 3, 3, 4];
    let len = Solution::remove_duplicates2(&mut nums);
    assert_eq!(len, 8);
    assert_eq!(nums[..len as usize], [0, 0, 1, 1, 2, 3, 3, 4]);
}

#[test]
fn test_rotate() {
    let cases = [
        (vec![1, 2, 3, 4, 5, 6, 7], 3, vec![5, 6, 7, 1, 2, 3, 4]),
        (vec![-1, -100, 3, 99], 2, vec![3, 99, -1, -100]),
        (vec![1], 0, vec![1]),
        (vec![1], 1, vec![1]),
    ];

    for (mut nums, k, expected) in cases {
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, expected);
    }
}

test_cases!(max_profit, [
    (vec![7, 1, 5, 3, 6, 4], 5),
    (vec![7, 6, 4, 3, 1], 0),
    (vec![2, 1], 0),
    (vec![1], 0),
]);

test_cases!(max_profit2, [
    (vec![7, 1, 5, 3, 6, 4], 7),
    (vec![1, 2, 3, 4, 5], 4),
    (vec![7, 6, 4, 3, 1], 0),
    (vec![2, 1], 0),
    (vec![1], 0),
]);

test_cases!(majority_element, [
    (vec![3, 2, 3], 3),
    (vec![2, 2, 1, 1, 1, 2, 2], 2),
    (vec![1], 1),
    (vec![0], 0),
]);

test_cases!(cap_jump, [
    (vec![2, 3, 1, 1, 4], true),
]);

test_cases!(cas_jump2, [
    (vec![2, 3, 1, 1, 4], 2),
    (vec![2, 3, 0, 1, 4], 2),
]);

test_cases!(h_index, [
    (vec![3, 0, 6, 1, 5], 3),
    (vec![1, 5, 1], 1),
    (vec![0], 0),
]);

#[test]
fn test_randomized_set() {
    use super::randomized_set::RandomizedSet;
    let mut set = RandomizedSet::new();
    assert_eq!(set.insert(1), true);
    assert_eq!(set.remove(2), false);
    assert_eq!(set.insert(2), true);
    let val = set.get_random();
    assert!(val == 1 || val == 2);
    assert_eq!(set.remove(1), true);
    assert_eq!(set.insert(2), false);
    assert!(set.get_random() == 2);
}

test_cases!(product_except_self, [
    (vec![1, 2, 3, 4], vec![24, 12, 8, 6]),
]);

test_cases_multi!(can_complete_circuit, [
    ((vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3),
    ((vec![2, 3, 4], vec![3, 4, 3]), -1),
    ((vec![5, 8, 2, 8], vec![6, 5, 6, 6]), 3),
]);

test_cases!(candy, [
    (vec![1, 0, 2], 5),
    (vec![1, 2, 2], 4),
    (vec![1, 3, 2, 2, 1], 7),
    (vec![1, 2, 3, 1, 0], 9),
]);

test_cases!(trap, [
    (vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6),
    (vec![4, 2, 0, 3, 2, 5], 9),
]);

test_cases!(roman_to_int, [
    (String::from("III"), 3),
    (String::from("IV"), 4),
    (String::from("LVIII"), 58),
    (String::from("MCMXCIV"), 1994),
]);

test_cases!(int_to_roman, [
    (3, String::from("III")),
    (4, String::from("IV")),
    (58, String::from("LVIII")),
    (1994, String::from("MCMXCIV")),
    (3999, String::from("MMMCMXCIX")),
]);

test_cases_multi!(is_anagram, [
    ((String::from("anagram"), String::from("nagaram")), true),
    ((String::from("rat"), String::from("car")), false),
    ((String::from(""), String::from("")), true),
    ((String::from("にほん"), String::from("ほんに")), true),
]);

#[test]
fn test_group_anagrams() {
    let cases = [
        (vec![String::from("eat"), String::from("tea"), String::from("tan"), String::from("ate"), String::from("nat"), String::from("bat")],
         vec![vec![String::from("ate"), String::from("eat"), String::from("tea")],
              vec![String::from("bat")],
              vec![String::from("nat"), String::from("tan")]]),
        (vec![String::from("")], vec![vec![String::from("")]]),
        (vec![], vec![]),
    ];

    for (strs, expected_groups) in cases {
        let mut groups = Solution::group_anagrams(strs);
        // Sort the groups and their contents for comparison
        for group in &mut groups {
            group.sort();
        }
        groups.sort_by(|a, b| a.get(0).cmp(&b.get(0)));
        assert_eq!(groups, expected_groups);
    }
}

test_cases!(trailing_zeros, [
    (5, 1),
    (3, 0),
    (0, 0),
    (25, 6),
]);

test_cases!(my_sqrt, [
    (0, 0),
    (1, 1),
    (2, 1),
    (3, 1),
    (4, 2),
    (25, 5),
    (2147483647, 46340),
]);

test_cases!(max_points, [
    (vec![vec![1,1], vec![2,2], vec![3,3]], 3),
    (vec![vec![1,1], vec![3,2], vec![5,3], vec![4,1], vec![2,3], vec![1,4]], 4),
]);

test_cases!(find_min_arrow_shots, [
    (vec![vec![10,16], vec![2,8], vec![1,6], vec![7,12]], 2),
    (vec![vec![1,2], vec![2,3], vec![3,4], vec![4,5]], 2),
]);

test_cases!(generate_parenthesis, [
    (3, vec!["((()))","(()())","(())()","()(())","()()()"]),
    (1, vec!["()"]),
]);

test_cases_multi!(min_sub_array_len, [
    ((7, vec![2, 3, 1, 2, 4, 3]), 2),
    ((4, vec![1, 4, 4]), 1),
    ((11, vec![1, 1, 1, 2, 1, 1, 1, 1]), 0),
]);

test_cases!(length_of_longest_substring, [
    (String::from("abcabcbb"), 3),
    (String::from("bbbbb"), 1),
    (String::from("pwwkew"), 3),
]);

test_cases_multi!(find_substring, [
    ((String::from("barfoothefoobarman"), 
    vec![String::from("foo"), String::from("bar")]), 
    vec![0, 9]),
    ((String::from("barfoofoobarthefoobarman"), 
    vec![String::from("foo"), String::from("bar"), String::from("the")]), 
    vec![6, 9, 12]),
    ((String::from("wordgoodgoodgoodbestword"), 
    vec![String::from("word"), String::from("good"), String::from("best"), String::from("good")]), 
    vec![8]),
]);