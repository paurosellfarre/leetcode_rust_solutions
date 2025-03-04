
struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = String::from("the sky is blue");
        assert_eq!(String::from("blue is sky the"), Solution::reverse_words(s));
    }

    #[test]
    fn test_2() {
        let s = String::from("  hello world  ");
        assert_eq!(String::from("world hello"), Solution::reverse_words(s));
    }

    #[test]
    fn test_3() {
        let s = String::from("a good   example");
        assert_eq!(String::from("example good a"), Solution::reverse_words(s));
    }
}