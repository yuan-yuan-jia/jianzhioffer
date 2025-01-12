use core::net;

include!("../src/include/list.rs");




fn remove_nth_from_end<T>(head: Option<Arc<RefCell<ListNode<T>>>>, n: i32) -> Option<Arc<RefCell<ListNode<T>>>> 
where T: Clone + Display + Debug + Default
{
    if head.is_none() || n < 1 {
        return None;
    }

    let dummy_head = ListNode::new(T::default());
    let dummy_head = dummy_head.unwrap();
    dummy_head.borrow_mut().next = head.clone();
    let mut pre = Some(dummy_head.clone());
    let mut vanguard = Some(dummy_head.clone());

    let mut step = 0;
    while vanguard.is_some() && step < n {
        vanguard =  vanguard.unwrap().borrow().next.clone();
        
        step += 1;
    }
    
    if vanguard.is_none() {
        // 不够长度n，删除失败，直接返回None。
        return None;
    }

    while vanguard.as_ref().unwrap().borrow().next.is_some() {
        vanguard =  vanguard.unwrap().borrow().next.clone();
        pre = pre.unwrap().borrow().next.clone();
    }

    let pre = pre.unwrap();
    let removed_node = pre.borrow().next.clone();
    let cloned_removed_node = removed_node.clone();
    let removed_node = removed_node.unwrap();
    pre.borrow_mut().next = removed_node.borrow().next.clone();
    removed_node.borrow_mut().next = None;
    // 释放头节点防止内存泄漏
    // 好像没必要？
    //dummy_head drop时会自动释放？ 
    dummy_head.borrow_mut().next = None;
    cloned_removed_node
}

fn main() {
    let n1 = ListNode::new(1);
    let n2 = ListNode::new(2);
    n1.as_ref().unwrap().borrow_mut().next = n2.clone();
    let n3 = ListNode::new(3);
    n2.as_ref().unwrap().borrow_mut().next = n3.clone();
    let n4 = ListNode::new(4);
    n3.as_ref().unwrap().borrow_mut().next = n4.clone();
    let n5 = ListNode::new(5);
    n4.as_ref().unwrap().borrow_mut().next = n5.clone();
    let n6 = ListNode::new(6);
    n5.as_ref().unwrap().borrow_mut().next = n6.clone();


    println!("==========before delete 2th node==========");
    print_list(n1.clone());
    println!("==========before delete 2th node==========");

    println!("==========after delete 2th node==========");
    let removed_node = remove_nth_from_end(n1.clone(),2);
    if removed_node.is_none() {
        println!("删除节点为None");
    }else {
        println!("删除节点为{}", removed_node.unwrap().borrow().val);
    }
    print_list(n1.clone());
    println!("==========after delete 2th node==========");



}