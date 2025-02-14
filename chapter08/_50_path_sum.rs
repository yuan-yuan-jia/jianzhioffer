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


fn do_path_sum(root: Option<Rc<RefCell<TreeNode>>>, path_sum: i32, map: &mut std::collections::HashMap<i32,i32>, sum: i32) -> i32 {
    if root.is_none() {
        return 0;
    }
    let root_ref  = root.as_ref().unwrap();
    let path_sum = path_sum + root_ref.borrow().val;
    let count = match map.get(&(path_sum - sum)) {
        Some(e) => *e,
        None => 0,
    };

    let path_cnt = match map.get(&path_sum) {
        Some(e) => *e + 1,
        None => 1,
    };
    map.insert(path_sum, path_cnt);

    let count = count + do_path_sum(root_ref.borrow().left.clone(), path_sum, map, sum);
    let count = count + do_path_sum(root_ref.borrow().right.clone(), path_sum, map, sum);
    map.remove(&path_sum);
    
    count

}


fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    let mut map = std::collections::HashMap::new();
    // 刚好到根节点的特殊情况
    map.insert(0, 1);
    do_path_sum(root, 0, &mut map, sum)
}


/*Driver Code */
fn main() {

    // 构建二叉树
    let root = TreeNode::new(5);

    {
        // 构建左子树
        let left_node = TreeNode::new_node(2, 1, 6);
        root.borrow_mut().left = Some(left_node);
    }
    
    {
        // 构建右子树
        let right_node = TreeNode::new_node(4, 3, 7);
        right_node.borrow_mut().right = Some(
            TreeNode::new(2)
        );
        root.borrow_mut().right = Some(right_node);
    }

    let r = path_sum(Some(root.clone()), 8);    

    println!("{}", r);
    assert_eq!(r, 2);
}