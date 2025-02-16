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


fn do_convert_bst(root: Option<Rc<RefCell<TreeNode>>>, sum :&mut i32) {
    if root.is_none() {
        return;
    }
    let root_ref = root.as_ref().unwrap();
    let root_val = root_ref.borrow().val;
    do_convert_bst(root_ref.borrow().right.clone(), sum);
    *sum = *sum + root_val;
    // 当前节点
    root_ref.borrow_mut().val = *sum;
    // 左子树
    do_convert_bst(root_ref.borrow().left.clone(), sum);
}

fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) {
    let mut i = 0;
    do_convert_bst(root,&mut i);
}





/*Driver Code */
fn main() {

    // 构建二叉树
    let root = TreeNode::new(4);

    {
        // 构建左子树
        let left_node = TreeNode::new_node(2,1,3);
        root.borrow_mut().left = Some(left_node);
    }
    
    {
        // 构建右子树
        let right_node = TreeNode::new_node(6,5,7);
        root.borrow_mut().right = Some(right_node);
    }

    print_tree(Some(root.clone()));
    println!("==========");
    convert_bst(Some(root.clone()));
    print_tree(Some(root.clone()));
    println!("==========");
}