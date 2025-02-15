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


fn do_increasing_bst_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let root_ref = root.as_ref().unwrap();
    // 叶节点
    if root_ref.borrow().left.is_none() && root_ref.borrow().right.is_none() {
        // 直接返回
        return root;
    }
    let left = do_increasing_bst_dfs(root_ref.borrow().left.clone());
    let right = do_increasing_bst_dfs(root_ref.borrow().right.clone());

    root_ref.borrow_mut().right = right;    
    
    if left.is_some() {
        // 左子树存在
        root_ref.borrow_mut().left = None;
        let mut left_pre = left.as_ref().unwrap().clone();
        // 找左子树中的右子树的右子树为空的节点
        // 将当前节点作为该节点的右子树
        while left_pre.borrow().right.is_some() {
            let next = left_pre.borrow().right.clone().unwrap();
            left_pre = next;
        }  
        left_pre.borrow_mut().right = root;
        return left;    
    }

    root
}

fn incresing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    do_increasing_bst_dfs(root)    
}


fn result_check(root: Option<Rc<RefCell<TreeNode>>>) {
    if root.is_none() {
        return;
    }
    let root_ref = root.as_ref().unwrap();
    if root_ref.borrow().left.is_some() {
        let s = format!("the node {} should not has left child",  root_ref.borrow().val);       
        panic!("the result is not correct: {}", s);
    }
    result_check(root_ref.borrow().right.clone());
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
        let right_node = TreeNode::new(5);
        right_node.borrow_mut().right = Some(
            TreeNode::new(6)
        );
        root.borrow_mut().right = Some(right_node);
    }

    print_tree(Some(root.clone()));
    println!();
    let root = incresing_bst(Some(root));
    result_check(root.clone());
    print_tree(root.clone());
    println!();
}