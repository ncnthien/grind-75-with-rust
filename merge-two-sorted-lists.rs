#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

struct Solution;

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(list1), None) => Some(list1),
            (None, Some(list2)) => Some(list2),
            (Some(list1), Some(list2)) => match list1.val < list2.val {
                true => Some(Box::new(ListNode {
                    val: list1.val,
                    next: Self::merge_two_lists(list1.next, Some(list2))
                })),
                false => Some(Box::new(ListNode {
                    val: list2.val,
                    next: Self::merge_two_lists(Some(list1), list2.next)
                }))
            }
        }
    }
}

fn main() {

}
