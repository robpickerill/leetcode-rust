//! LeetCode 33 - Search in Rotated Sorted Array.  https://leetcode.com/problems/search-in-rotated-sorted-array/
//! Pattern: Binary Search
//! Time: O(log n), Space: O(1)

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search<T: Ord>(nums: &[T], target: T) -> Result<usize, Box<dyn std::error::Error>> {
        if nums.is_empty() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotSeekable,
                "empty array",
            )));
        }

        let n = nums.len();
        let last = match nums.last() {
            Some(last) => last,
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "target not found in array",
                )));
            }
        };

        // [4, 5, 6, ,7, 0, 1, 2] -> rot = 4
        let rot = nums.partition_point(|x| x > last);

        let (mut lo, mut hi) = (0usize, n);

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            // mid = 3 -> midval = nums[(3 + 4) % 8] = nums[7] = 2
            let midval = &nums[(mid + rot) % n];

            match midval.cmp(&target) {
                Ordering::Greater => hi = mid,
                Ordering::Equal => return Ok((mid + rot) % n),
                Ordering::Less => lo = mid + 1,
            }
        }

        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "target not found in array",
        )))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn basic() {
        let nums = [3, 4, 5, 6, 7, 8, 9, 1, 2];
        assert_eq!(Solution::search(&nums, 4).unwrap(), 1);
    }
}
