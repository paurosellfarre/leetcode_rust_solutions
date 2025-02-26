
struct Solution {}

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut i = 0;
        let mut n = n;
        if n == 0 { return true };
        while i < flowerbed.len() {
            if flowerbed[i] == 1 {
                i += 2;
                continue
            }

            if i < flowerbed.len() - 1 && flowerbed[i+1] == 0 || i == flowerbed.len() - 1 {
                if n > 1 { 
                    n -= 1;
                    i += 1;
                } else { return true }
            }
            
            i += 1;
        }
        false      
    }
}


fn main() {

}

#[cfg(test)]

 mod tests {

    use super::*;

    #[test]
    fn test_1() {
        let flowerbed = vec![1,0,0,0,1];
        assert_eq!(true, Solution::can_place_flowers(flowerbed, 1))
    }

    #[test]
    fn test_2() {
        let flowerbed = vec![1,0,0,0,0,0,1];
        assert_eq!(true, Solution::can_place_flowers(flowerbed, 2))
    }

    #[test]
    fn test_3() {
        let flowerbed = vec![1,0,0,0,1,0,0];
        assert_eq!(true, Solution::can_place_flowers(flowerbed, 2))
    }

    #[test]
    fn test_4() {
        let flowerbed = vec![1,0,0,0,0,1];
        assert_eq!(false, Solution::can_place_flowers(flowerbed, 2))
    }

 }