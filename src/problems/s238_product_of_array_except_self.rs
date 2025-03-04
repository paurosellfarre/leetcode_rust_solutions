struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prefix = vec![0; n];
        let mut suffix = vec![0; n];
        let mut ans = vec![0; n];
        prefix[0] = 1;
        suffix[n - 1] = 1;

        for i in 1..n {
            prefix[i] = prefix[i - 1] * nums[i - 1];
        }
        for i in (0..=n - 2).rev() {
            suffix[i] = suffix[i + 1] * nums[i + 1];
        }
        println!("{:?}", suffix);
        println!("{:?}", prefix);
        for i in 0..n {
            ans[i] = prefix[i] * suffix[i];
        }
        ans
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(vec![24, 12, 8, 6], Solution::product_except_self(nums));
    }

    #[test]
    fn test_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        assert_eq!(vec![0, 0, 9, 0, 0], Solution::product_except_self(nums));
    }
}
