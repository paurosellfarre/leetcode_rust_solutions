struct Solution {}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut first, mut second) = (i32::MAX, i32::MAX);
        nums.into_iter().any(|num| {
            if num <= first {
                first = num;
                false
            } else if num <= second {
                second = num;
                false
            } else {
                true
            }
        })
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(true, Solution::increasing_triplet(nums));
    }

    #[test]
    fn test_2() {
        let nums = vec![5, 4, 3, 2, 1];
        assert_eq!(false, Solution::increasing_triplet(nums));
    }

    #[test]
    fn test_3() {
        let nums = vec![2, 1, 5, 0, 4, 6];
        assert_eq!(true, Solution::increasing_triplet(nums));
    }
}
