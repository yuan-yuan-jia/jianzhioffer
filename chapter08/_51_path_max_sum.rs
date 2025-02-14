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


fn path_max_sum_dfs(root: Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
    if root.is_none() {
        return 0;
    }

    let root_ref = root.as_ref().unwrap();
    // 求左子树的最大的和
    let mut left_v = 0;
    let left = path_max_sum_dfs(root_ref.borrow().left.clone(), &mut left_v);
    // 求右子树最大的和
    let mut right_v = 0;
    let right = path_max_sum_dfs(root_ref.borrow().right.clone(), &mut right_v);
    // 比较三者出中的最大值，（左子树、右子树、当前节点 + 左子树 + 右子树 ）
    let max = left_v.max(right_v);
    let max = max.max(left_v + right_v + root_ref.borrow().val);
    *max_sum = (*max_sum).max(max);

    // 返回当前节点和最大子树的和，组成一个新的路径
    return root_ref.borrow().val + left.max(right);
}

fn path_max_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut r = 0;
    path_max_sum_dfs(root, &mut r);
    r
}


/*Driver Code */
fn main() {

    // 构建二叉树
    let root = TreeNode::new(-9);

    {
        // 构建左子树
        let left_node = TreeNode::new(2);
        root.borrow_mut().left = Some(left_node);
    }
    
    {
        // 构建右子树
        let right_node = TreeNode::new(20);
        right_node.borrow_mut().right = Some(
            TreeNode::new(7)
        );
        
        {
            let left_node = TreeNode::new(15);
            let left_left_node = TreeNode::new(-3);
            left_node.borrow_mut().left = Some(left_left_node);
            right_node.borrow_mut().left = Some(left_node);
        }
        
        root.borrow_mut().right = Some(right_node);
    }

    let r = path_max_sum(Some(root.clone()));    

    println!("{}", r);
    assert_eq!(r, 42);
}