//https://www.codewars.com/kata/52bef5e3588c56132c0003bc

mod preloaded;
use preloaded::Node;
use std::collections::VecDeque;

fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::new();
    let mut queue: VecDeque<&Node> = VecDeque::new();
    
    queue.push_back(root);
    
    while queue.is_empty() == false {
        let current: &Node = queue.pop_front().unwrap();
        
        output.push(current.value);
        
        if current.left.is_none() == false {
            queue.push_back(&current.left.as_ref().unwrap());
        }
        
        if current.right.is_none() == false {
            queue.push_back(&current.right.as_ref().unwrap());
        }
    }
    
    output
}