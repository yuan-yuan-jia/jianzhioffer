/*
  反转链表
  定义一个函数，输入一个链表的头节点，
  反转该链表并输出反转后链表的头节点。
  1--> 2-->3-->4-->5
  5--> 4-->3-->2-->1
*/

include!("../src/include/list.rs");

// 三指针的方法
fn reverse_list(h: Option<Arc<RefCell<ListNode<i32>>>>) -> Option<Arc<RefCell<ListNode<i32>>>>{

    if h.is_none() {
        return None;
    }
    // 只有一个节点，也支持。所以这个判断完全没有必要 
    // if h.is_some() && h.clone().unwrap().borrow().next.is_none() {
    //     return h;
    // }

    let mut pre = None;
    let mut middle = h.clone();
    let mut post = h.clone().unwrap().borrow().next.clone();

    
    
    while  middle.is_some() {
        middle.clone().unwrap().borrow_mut().next = pre.clone();
        pre = middle.clone();
        middle = post.clone();
        
        if post.is_some() {
           // post已经先为None，但是反转过程还没有完成
           post = post.unwrap().borrow().next.clone();    
        }
          
    }

    let h = pre.clone();
    h
}

fn main() {

    let n1 = ListNode::new(1);
    let n2 = ListNode::new(2);
    n1.clone().unwrap().borrow_mut().next = n2.clone();
    let n3 = ListNode::new(3);
    n2.clone().unwrap().borrow_mut().next = n3.clone();
    let n4 = ListNode::new(4);
    n3.clone().unwrap().borrow_mut().next = n4.clone();
    let n5 = ListNode::new(5);
    n4.clone().unwrap().borrow_mut().next = n5.clone();

    println!("反转前==========");
    print_list(n1.clone());
    println!("反转前==========");
    let new_head = reverse_list(n1.clone());
    println!("反转后==========");
    print_list(new_head.clone());
    println!("反转后==========");

 
}