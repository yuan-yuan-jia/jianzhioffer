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



/*Driver Code */
fn main() {

    // 构建二叉树
    let root = TreeNode::new(2);

    {
        // 构建左子树
        let left_node = TreeNode::new(1);
        root.borrow_mut().left = Some(left_node);
    }
    
    {
        // 构建右子树
        let right_node = TreeNode::new(3);
        root.borrow_mut().right = Some(right_node);
    }

    print_tree(Some(root.clone()));
    println!("==========");
    let tree = BinaryTree {
        root: Some(root.clone())
    };
    let mut binary_iter = BinaryTreeIter::new(&tree);

    while let Some(a)  = binary_iter.next() {
        println!("{}", a.borrow().val);
    }
}