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


fn prune_tree_do(cur: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>{
    if cur.is_some() {
        let cur_node = cur.clone();
        if cur_node.as_ref().unwrap().borrow().left.is_none() && cur_node.as_ref().unwrap().borrow().right.is_none() {
            if cur_node.as_ref().unwrap().borrow().val == 0 {
                // 叶节点且值为0，直接干掉
                return None;
            }else {
                return cur_node;
            }
        }
        {
            let left = cur_node.as_ref().unwrap().borrow().left.clone();
            cur_node.as_ref().unwrap().borrow_mut().left =   prune_tree_do( left);
        }
        {
            let right = cur_node.as_ref().unwrap().borrow().right.clone();
            cur_node.as_ref().unwrap().borrow_mut().right =   prune_tree_do(right);
        }
        if cur_node.as_ref().unwrap().borrow().left.is_none() && 
           cur_node.as_ref().unwrap().borrow().right.is_none() &&
           cur_node.as_ref().unwrap().borrow().val == 0 {
                // 当前节点成为叶节点，且值为0
                // 删除当前节点
                return None;
           }
        return cur_node;
    }
    None
}

fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>{
    if root.is_some() {
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();
        
        
        root.as_ref().unwrap().borrow_mut().left = prune_tree_do(left);
          
        root.as_ref().unwrap().borrow_mut().right = prune_tree_do( right);
        
        let c=  root.as_ref().unwrap().borrow();
        if c.left.is_none() && c.right.is_none() && c.val == 0 {
            return None;
        }
        return root.clone();
    }
    None
}



fn print_tree(node: Option<Rc<RefCell<TreeNode>>>) {
    if node.is_some() {
        let n = node.as_ref().unwrap().borrow().val;
        print!("{},", n);
        print_tree(node.as_ref().unwrap().borrow().left.clone());
        print_tree(node.as_ref().unwrap().borrow().right.clone());
    }
}


fn main() {
    let  root = TreeNode::new(1);
    {

        // 构建左子树
        root.borrow_mut().left = Some(TreeNode::new_node(0, 0, 0));

    }

    {
        // 构建右子树
        root.borrow_mut().right = Some(
            TreeNode::new_node(0, 0, 1)
        );
    }

    print_tree(Some(root.clone()));
    println!();
    println!("========after============");

    let root = prune_tree(Some(root));
    print_tree(root);
    println!()
}