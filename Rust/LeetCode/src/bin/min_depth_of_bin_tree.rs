// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
			Some(node) => {
				let node = node.as_ref().borrow();
				let left = Self::min_depth(node.left.clone());
				let right = Self::min_depth(node.right.clone());
				if left == 0 || right == 0 {
					1 + left + right
				} else {
					1 + left.min(right)
				}
			}
			None => 0,
		}
    }
}

fn main() {
	let root = Some(Rc::new(RefCell::new(TreeNode {
		val: 3,
		left: Some(Rc::new(RefCell::new(TreeNode {
			val: 9,
			left: None,
			right: None,
		}))),
		right: Some(Rc::new(RefCell::new(TreeNode {
			val: 20,
			left: Some(Rc::new(RefCell::new(TreeNode {
				val: 15,
				left: None,
				right: None,
			}))),
			right: Some(Rc::new(RefCell::new(TreeNode {
				val: 7,
				left: None,
				right: None,
			}))),
		}))),
	})));
	let result = Solution::min_depth(root);
    println!("min_depth: {:?}", result);
}
