
struct Solution {}

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        
        let mut resp = vec![false; candies.len()];
        let max_candies = candies.iter().max().unwrap();

        for i in 0..candies.len() {
            if candies[i] + extra_candies >= *max_candies {
                resp[i] = true
            }
        }

        resp
    }
}

fn main() {

}

#[cfg(test)]

    mod tests{
        use super::*;


        #[test]
        fn test_1() {
            assert_eq!(
                Vec::<bool>::from([true,true,true,false,true]),
                Solution::kids_with_candies(Vec::<i32>::from([2,3,5,1,3]), 3)
            )
        }

    }