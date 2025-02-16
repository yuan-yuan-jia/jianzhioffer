use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}



struct BinaryTree{
    root: Option<Rc<RefCell<TreeNode>>>
}

struct BinaryTreeIter{
    stack: std::collections::VecDeque<Rc<RefCell<TreeNode>>>,
    cur: Option<Rc<RefCell<TreeNode>>>,
}

struct BinaryTreeIterRev{
    stack: std::collections::VecDeque<Rc<RefCell<TreeNode>>>,
    cur: Option<Rc<RefCell<TreeNode>>>,
}

impl BinaryTreeIter {
    fn new(tree: &BinaryTree) -> Self {
        Self {
            stack: std::collections::VecDeque::new(),
            cur: tree.root.clone(),
        }
    }
}

impl Iterator for BinaryTreeIter {
    type Item =  Rc<RefCell<TreeNode>>;
    
    fn next(&mut self) -> Option<Self::Item> {
        while self.cur.is_some() {
            self.stack.push_back(
                self.cur.as_ref().unwrap().clone()
            );
            self.cur = self.cur.as_ref().unwrap().clone().borrow().left.clone();
        }

        
        let c = self.stack.pop_back();
        if let Some(a) = c {
            self.cur = a.borrow().right.clone();
            return Some(a);
        }else {
            None
        }
    }

}


impl BinaryTreeIterRev {
    fn new(tree: &BinaryTree) -> Self {
        Self {
            stack: std::collections::VecDeque::new(),
            cur: tree.root.clone(),
        }
    }
}

impl Iterator for BinaryTreeIterRev  {
    type Item = Rc<RefCell<TreeNode>>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cur.is_some() {
            self.stack.push_back(
                self.cur.as_ref().unwrap().clone()
            );
            self.cur = self.cur.as_ref().unwrap().clone().borrow().right.clone();
        }

        let c = self.stack.pop_back();
        if let Some(a) = c {
            self.cur = a.borrow().left.clone();
            return Some(a);
        }else {
            None
        }
    }

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


/*双指针发：两个迭代器，还有其他方法比如使用哈希表 */
fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    if root.is_none() {
        return false;
    }

    let tree = BinaryTree {
        root
    };

    let mut iter_next = BinaryTreeIter::new(&tree);
    let mut iter_prev = BinaryTreeIterRev::new(&tree);

    let mut next = iter_next.next();
    let mut pre = iter_prev.next(); 

    while next.is_some() && pre.is_some() {
        let sum = next.as_ref().unwrap().borrow().val + pre.as_ref().unwrap().borrow().val;
        
        if sum == k {
            return true;
        }

        if sum < k {
            next = iter_next.next();
        }else {
            pre = iter_prev.next();
        }
    }

    return false;
}

fn print_tree(node: Option<Rc<RefCell<TreeNode>>>) {
    if node.is_some() {
        let n = node.as_ref().unwrap().borrow().val;
        print!("{},", n);
        print_tree(node.as_ref().unwrap().borrow().left.clone());
        print_tree(node.as_ref().unwrap().borrow().right.clone());
    }
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
    println!("==========");
    
    let r = find_target(Some(root), 12);
    println!("r:{}", r);
    assert_eq!(r, true);
}