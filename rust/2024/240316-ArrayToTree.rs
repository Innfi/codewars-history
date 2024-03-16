// https://www.codewars.com/kata/57e5a6a67fbcc9ba900021cd

/**
 * fn array_to_tree(xs: &[i32]) -> Option<TreeNode> {
    to_tree_from(xs, 0)
}

fn to_tree_from(xs: &[i32], i: usize) -> Option<TreeNode> {
    xs.get(i).map(|&x| TreeNode {
        value: x,
        left: to_tree_from(xs, 2 * i + 1).map(Box::new),
        right: to_tree_from(xs, 2 * i + 2).map(Box::new),
    })
}
 */

#[derive(Debug, PartialEq)]
pub struct TreeNode {
  pub value: i32,
  pub left: Option<Box<TreeNode>>,
  pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
  // This is just convenience so you don't have to `Box` everywhere.
  pub fn new(value: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> Self {
    Self {
      value,
      left: left.map(|n| Box::new(n)),
      right: right.map(|n| Box::new(n)),
    }        
  }
}

fn array_to_tree(array: &[i32]) -> Option<TreeNode> {
  let mut root: Option<TreeNode> = None;

  for i in 0..array.len() {
    let new_node: TreeNode = TreeNode::new(array[i], None, None);

    if root == None {
      root = Some(new_node);
      continue;
    }

    let mut current = &root.unwrap();
    if array[i] < root.unwrap().value {

    }


    // let mut root_ref = root.as_mut();
    // let mut current: Option<Box<TreeNode>> = None;
    // if array[i] < root_ref.unwrap().value {
    //   if root_ref.unwrap().left == None {
    //     root_ref.unwrap().left = Some(Box::new(new_node));
    //     continue;
    //   }
    // }
    

    // loop {

    //   break;
    // }
  }

  None
}

#[test]
fn test_initial_state() {
  let instance = TreeNode::new(1, None, None);

  assert_eq!(instance.value, 1);
  assert_eq!(instance.left, None);
  assert_eq!(instance.right, None);
}

