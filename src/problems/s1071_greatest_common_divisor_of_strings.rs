struct Solution {}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if &(str1.clone() + &str2) != &(str2.clone() + &str1) {
            return "".to_string();
        }
        let length = gcd(str1.len(), str2.len());
        str1[0..length].to_string()
    }
}
pub fn gcd(n1: usize, n2: usize) -> usize{
    if n2 == 0 {
        return n1;
    }
    gcd(n2, n1 % n2)
}

fn main() {

}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(String::from("ABC"), Solution::gcd_of_strings(String::from("ABCABC"), String::from("ABC")));
        assert_eq!(String::from("AB"), Solution::gcd_of_strings(String::from("ABABAB"), String::from("ABAB")));
    }
}