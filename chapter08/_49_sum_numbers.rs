use std::{cell::RefCell, rc::Rc};

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        let node = TreeNode {
            val,
            left: None,
            right: None,
        };
        Rc::new(
            RefCell::new(node)
        )
    }

    fn new_node(val: i32,left: i32, right: i32) -> Rc<RefCell<TreeNode>> {
        let node  = Self::new(val);
        node.borrow_mut().left = Some(
            Self::new(left)
        );
        node.borrow_mut().right = Some(
            Self::new(right)
        );
        node  
    }
}

fn print_tree(node: Option<Rc<RefCell<TreeNode>>>) {
    if node.is_some() {
        let n = node.as_ref().unwrap().borrow().val;
        print!("{},", n);
        print_tree(node.as_ref().unwrap().borrow().left.clone());
        print_tree(node.as_ref().unwrap().borrow().right.clone());
    }
}



fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>, path: i32) -> i32 {
    if root.is_some() {
        let refer_root = root.as_ref().unwrap();
        let path = path * 10  + refer_root.borrow().val;
        if refer_root.borrow().left.is_none() && refer_root.borrow().right.is_none() {
            return path;
        }
        return sum_numbers(refer_root.borrow().left.clone(), path) + sum_numbers(refer_root.borrow().right.clone(), path);
    }
    0
}


/*Driver Code */
fn main() {

    // 构建二叉树
    let root = TreeNode::new(3);

    {
        // 构建左子树
        let left_node = TreeNode::new_node(9, 5, 1);
        root.borrow_mut().left = Some(left_node);
    }
    
    {
        // 构建右子树
        let right_node = TreeNode::new(0);
        right_node.borrow_mut().right = Some(
            TreeNode::new(2)
        );
        root.borrow_mut().right = Some(right_node);
    }

    let result = sum_numbers(Some(root.clone()), 0);

    println!("result:{}", result);
    assert_eq!(result, 1088);

}