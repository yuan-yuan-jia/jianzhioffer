use std::{cell::RefCell, rc::Rc, str::Chars};

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


/*
    将二叉树序列为字符串
*/
fn do_serialization_book_version(node: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut r = String::new();
    if node.is_none() {
        r.push('#');
        return r;
    }
    let node_t = node.as_ref().unwrap();
    let val = node_t.borrow().val.to_string();
    let left = do_serialization_book_version(node_t.borrow().left.clone());
    let right = do_serialization_book_version(node_t.borrow().right.clone());
    
    r.push_str(&val);
    r.push(',');
    r.push_str(&left);
    r.push(',');
    r.push_str(&right);
    
    r

}
fn do_serialization(node: Option<Rc<RefCell<TreeNode>>>, r: &mut String) {
    if node.is_some() {
        let refe = node.as_ref().unwrap();
        let s = refe.borrow().val.to_string();   
        r.push_str(&s);
        r.push(',');
        // 处理左子树
        if refe.borrow().left.is_some() {
            do_serialization(refe.borrow().left.clone(), r);
        }else {
            r.push('#');
            r.push(',');
        }
        // 处理右子树
        if refe.borrow().right.is_some() {
            do_serialization(refe.borrow().right.clone(), r);
        }else {
            r.push('#');
            r.push(',');
        }  
    }else {
        r.push('#');
        r.push(',');
    }
}
fn serial(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut r = String::new();
    do_serialization(root, &mut r);
    // 去除最后一个逗号
    r.pop();
    r
}


fn do_deserialization(str: &mut Chars) -> Option<Rc<RefCell<TreeNode>>> {
    let c = str.nth(0);
    if c.is_some() {
        let c = c.unwrap();
        if c == '#' {
            return None;
        }
        if c == ',' {
            
            return do_deserialization(str);
        }
        let val: i32 = c as i32 - '0' as i32;
        let node = TreeNode::new(val);
        // 构建当前node的左子树
        
        node.borrow_mut().left = do_deserialization(str);
        // 构建当前node的右子树
        
        node.borrow_mut().right = do_deserialization(str);
        return Some(node);
    }
    None
}

fn deserial(s: &str) -> Option< Rc<RefCell<TreeNode>>> {
    let mut c = s.chars();
    //let mut i = 0;
    do_deserialization(&mut c)
}

/*
    将字符串反序列化为二叉树
*/

/*Deriver Code */
fn main() {
    // 构建二叉树
    let root = TreeNode::new(6);
    
    {
        // 构建左子树
        root.borrow_mut().left = Some(
            TreeNode::new_node(6, 6, 6)
        );
    }

    {
        // 构建右子树
        root.borrow_mut().right = Some(
            TreeNode::new(6)
        );
    }

    //序列化
    let serilization_r = serial(Some(root.clone()));
    println!("{}", serilization_r);

    // 反序列化
    let new_root = deserial(&serilization_r);
    println!("{}", serial(new_root));

}