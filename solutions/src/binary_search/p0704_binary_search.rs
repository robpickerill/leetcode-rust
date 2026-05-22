//! LeetCode 704 - Binary Search.  https://leetcode.com/problems/binary-search/
//! Pattern: Binary Search
//! Time: O(n log n), Space: O(1)

use cmp::Ordering;
use std::cmp;

pub struct Solution;

impl Solution {
    pub fn binary_search<T: Ord>(arr: &[T], target: T) -> Option<usize> {
        if arr.is_empty() {
            return None;
        }

        let mut low = 0;
        let mut high = arr.len();

        while low < high {
            let mid = low + (high - low) / 2;
            // compiles to: low + ((high - low) >> 1);

            match arr[mid].cmp(&target) {
                Ordering::Greater => high = mid,
                Ordering::Equal => return Some(mid),
                Ordering::Less => low = mid + 1,
            }
        }

        return None;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn basic() {
        let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(Solution::binary_search(&nums, 3), Some(2));
    }

    proptest! {
        #![proptest_config(ProptestConfig { max_shrink_iters: 100_000, ..ProptestConfig::default() })]

        #[test]
        fn oracle_agreement_no_dups(
            xs in prop::collection::vec(any::<i32>(), 0..4000),
            target in any::<i32>(),
        ) {
            let mut xs = xs;
            xs.sort();
            xs.dedup();

            let ours = Solution::binary_search(&xs, target);
            let theirs = xs.binary_search(&target).ok();

            prop_assert_eq!(ours, theirs);
        }

        #[test]
        fn returned_index_is_genuine(
            xs in prop::collection::vec(any::<i32>(), 0..200),
            target in any::<i32>(),
        ) {
            let mut xs = xs;
            xs.sort();

            if let Some(i) = Solution::binary_search(&xs, target) {
                prop_assert!(i < xs.len());
                prop_assert_eq!(xs[i], target);
            }
        }

        #[test]
        fn none_means_absent(
            xs in prop::collection::vec(any::<i32>(), 0..200),
            target in any::<i32>(),
        ) {
            let mut xs = xs;
            xs.sort();
            if Solution::binary_search(&xs, target).is_none() {
                prop_assert!(!xs.contains(&target));
            }
        }

        #[test]
        fn finds_existing_element(
            xs in prop::collection::vec(any::<i32>(), 1..200),
            idx in any::<prop::sample::Index>(),
        ) {
            let mut xs = xs;
            xs.sort();
            let i = idx.index(xs.len());
            let target = xs[i];
            prop_assert!(Solution::binary_search(&xs, target).is_some());
        }

        #[test]
        fn empty_is_always_none(target in any::<i32>()) {
            let xs: Vec<i32> = vec![];

            prop_assert_eq!(Solution::binary_search(&xs, target), None);
        }
    }
}
