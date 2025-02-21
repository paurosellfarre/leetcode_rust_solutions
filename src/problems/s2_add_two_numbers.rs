// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut curr = &mut head;
    
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let sum = l1.as_ref().map_or(0, |x| x.val) + l2.as_ref().map_or(0, |x| x.val) + carry;
            println!("{}", sum);
            
            carry = sum / 10;
    
            let sum_node = curr.insert(Box::new(ListNode::new(sum % 10)));
            println!("{:?}", sum_node);

            curr = &mut sum_node.next;
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }
    
        head
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));

        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));

        let resp = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(8)))
            }))
        }));
        
        assert_eq!(resp, Solution::add_two_numbers(l1,l2));
    }
}