/*
 重排链表
 给定一个链表，
 链表中节点的顺序是L0→L1→L2→…→Ln-1→Ln，
 请问如何重排链表使节点的顺序变成L0→Ln→L1→Ln-1→L2→Ln-2→…？
 1 --> 2 --> 3 --> 4 --> 5 --> 6
           |
           |
          \ / 
 2 --> 6 --> 2 --> 5 --> 3 --> 4           
  
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


fn reorder_list(h: Option<Arc<RefCell<ListNode<i32>>>>) -> Option<Arc<RefCell<ListNode<i32>>>> {
    // 利用快慢指针
    // 当快指针到达链表末尾时
    // 慢指针，正好在链表中间
    
    // 至少有三个节点，才能完成重排序
    if h.is_none() || 
       h.clone().unwrap().borrow().next.is_none() ||
       h.clone().unwrap().borrow().next.clone().unwrap().borrow().next.is_none()
       {
           return h.clone();
    }

    let mut slow = h.clone();
    let mut fast = h.clone();
    while fast.clone().unwrap().borrow().next.is_some() {
        fast = fast.clone().unwrap().borrow().next.clone();
        if fast.clone().unwrap().borrow().next.is_some() {
            fast = fast.clone().unwrap().borrow().next.clone();
        }

        slow = slow.clone().unwrap().borrow().next.clone();
    }

    let hh = slow.clone().unwrap().borrow().next.clone();
    slow.clone().unwrap().borrow_mut().next = None;
    let mut second_list = reverse_list(
        hh
    );
    let new_l = h.clone();
    let mut first_h = h.clone().unwrap().borrow().next.clone();
    h.clone().unwrap().borrow_mut().next = None;

    let mut pre = new_l.clone();
    while first_h.is_some() || second_list.is_some() {
        if second_list.is_some() {
            let ss = second_list.clone();
            second_list = second_list.clone().unwrap().borrow().next.clone();
            ss.clone().unwrap().borrow_mut().next = None;
            pre.clone().unwrap().borrow_mut().next = ss;
            pre = pre.clone().unwrap().borrow().next.clone();
        }

        if first_h.is_some() {
            let ss = first_h.clone();
            first_h = first_h.clone().unwrap().borrow().next.clone();
            ss.clone().unwrap().borrow_mut().next = None;
            pre.clone().unwrap().borrow_mut().next = ss;
            pre = pre.clone().unwrap().borrow().next.clone();
        }
    }
    new_l
}




/*Test Code */
fn main() {
    // 1 --> 2 --> 3 --> 4 --> 5 --> 6
    let n1 = ListNode::new(1);
    let n2 = ListNode::new(2);
    n1.clone().unwrap().borrow_mut().next = n2.clone();
    let n3 = ListNode::new(3);
    n2.clone().unwrap().borrow_mut().next = n3.clone();
    let n4 = ListNode::new(4);
    n3.clone().unwrap().borrow_mut().next = n4.clone();
    let n5 = ListNode::new(5);
    n4.clone().unwrap().borrow_mut().next = n5.clone();
    let n6 = ListNode::new(6);
    n5.clone().unwrap().borrow_mut().next = n6.clone();

    println!("===重排序前====");
    print_list(n1.clone());
    println!("===重排序前====");
    println!("");
    println!("===重排序后====");
    let h = reorder_list(n1.clone());
    print_list(h.clone());
    println!("===重排序后====");

}