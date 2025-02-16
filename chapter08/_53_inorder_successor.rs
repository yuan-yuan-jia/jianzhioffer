use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
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



fn inorder_successor(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let root_ref = root.as_ref().unwrap();
    if root_ref.borrow().left.is_some() {
        let left = inorder_successor(root_ref.borrow().left.clone(), val);
        if left.is_some() && left.as_ref().unwrap().borrow().val > val {
            return left;
        }
    }
    
    if root_ref.borrow().val > val {
        return root.clone();
    }
    if root_ref.borrow().right.is_some() {
        let right = inorder_successor(root_ref.borrow().right.clone(), val);
        if right.is_some() && right.as_ref().unwrap().borrow().val > val {
            return right;
        }
    }
    None
}




/*Driver Code */
fn main() {

    // 构建二叉树
    let root = TreeNode::new(8);

    {
        // 构建左子树
        let left_node = TreeNode::new_node(6,5,7);
        root.borrow_mut().left = Some(left_node);
    }
    
    {
        // 构建右子树
        let right_node = TreeNode::new_node(10,9,11);
        root.borrow_mut().right = Some(right_node);
    }

    print_tree(Some(root.clone()));
    println!();
    let target = inorder_successor(Some(root.clone()), 11);
    println!("{:?}", target);
    println!("====================================");
    match target {
        Some( ref e) => {
            println!("{:?}", e.borrow().val);
        },
        None => {println!("None")}
    }
}