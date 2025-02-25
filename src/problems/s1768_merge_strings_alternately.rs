
struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        
        let mut result = String::with_capacity(word1.len() + word2.len());
        for (c1, c2) in word1.chars().zip(word2.chars()) {
            result.push(c1);
            result.push(c2);
        }
        result.extend(word1.chars().skip(word2.len()));
        result.extend(word2.chars().skip(word1.len()));
        result
    }
}

fn main() {

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("apbqrs", Solution::merge_alternately(String::from("ab"), String::from("pqrs")));
    }
}