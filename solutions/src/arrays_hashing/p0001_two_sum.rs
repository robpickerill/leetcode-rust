//! LeetCode 1 - Two Sum.  https://leetcode.com/problems/two-sum/
//! Pattern: HashMap one-pass.
//! Time O(n), Space O(n).

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: &[i32], target: i32) -> (i32, i32) {
        let mut seen = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(j) = seen.get(&complement) {
                return (*j as i32, i as i32);
            }

            seen.insert(num, i);
        }

        panic!("No solution found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let nums = vec![1, 2, 3, 4, 5];
        let target = 3;

        assert!(Solution::two_sum(&nums, target) == (0, 1));
    }

    use proptest::prelude::*;

    fn proptest_input_with_solution() -> impl Strategy<Value = (Vec<i32>, i32)> {
        prop::collection::vec(-10_000i32..10_000, 2..50)
            .prop_flat_map(|v| {
                let len = v.len();
                (Just(v), 0..len, 0..len)
            })
            .prop_filter("indices must be distinct", |(_, i, j)| i != j)
            .prop_map(|(v, i, j)| {
                let target = v[i] + v[j];
                (v, target)
            })
    }

    proptest! {
        #[test]
        fn returned_pair_sums_to_target(
            (nums, target) in proptest_input_with_solution()
        ) {
            let (i, j) = Solution::two_sum(&nums, target);
            let (i, j) = (i as usize, j as usize);

            prop_assert!(i < nums.len());
            prop_assert!(j < nums.len());
            prop_assert_ne!(i, j);
            prop_assert_eq!(nums[i] + nums[j], target);
        }
    }
}
