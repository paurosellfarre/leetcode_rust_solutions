

struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = vec!['a','e','i','o','u'];
        
        let mut string_rev_vowels = s.chars().filter(|ch|{
            vowels.iter().find(|&v| &ch.to_ascii_lowercase() == v).is_some()
        }).rev();

        s.chars().map(|ch| {
            match vowels.iter().find(|&v| &ch.to_ascii_lowercase() == v) {
                Some(_) => string_rev_vowels.next().unwrap(),
                None => ch,
            }
        }).collect()
    }
}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(String::from("race car"),Solution::reverse_vowels(String::from("race car")))
    }

    #[test]
    fn test_2() {
        assert_eq!(String::from("AceCreIm"),Solution::reverse_vowels(String::from("IceCreAm")))
    }

    #[test]
    fn test_3() {
        assert_eq!(String::from("leotcede"),Solution::reverse_vowels(String::from("leetcode")))
    }

    #[test]
    fn test_4() {
        assert_eq!(String::from("leotcede"),Solution::reverse_vowels(String::from("leetcode")))
    }
}