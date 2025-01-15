/*
  链表中的数字相加
  给定两个表示非负整数的单向链表，
  请问如何实现这两个整数的相加并且把它们的和仍然用单向链表表示？
  链表中的每个节点表示整数十进制的一位，
  并且头节点对应整数的最高位数而尾节点对应整数的个位数。

  链表a: 表示整数123,读作一百二十三
  1 --> 2 --> 3
  链表b: 表示整数531,读作五百三十一
  5 --> 3 --> 1

  两个链表相加：得到新的链表：
  6 --> 5 --> 4，读作六百五十四
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

fn add_two_numbers(
    a: Option<Arc<RefCell<ListNode<i32>>>>,
    b: Option<Arc<RefCell<ListNode<i32>>>>,
) -> Option<Arc<RefCell<ListNode<i32>>>> {


    let dummy_head = ListNode::new(-1);

    // 先反转链表
    let mut a = reverse_list(a);
    let mut b = reverse_list(b);
    
    
    let mut carry = 0;
    
    
    while a.is_some() || b.is_some() {
        let mut add_result = 
        if a.is_some() {a.clone().unwrap().borrow().val} else {0} 
           + 
        if b.is_some() {b.clone().unwrap().borrow().val} else {0} 
           + 
        carry;
        
        
        if add_result >= 10 {
            carry = 1;
            add_result -= 10;
        }else {
            carry = 0;
        }
        let new_node = ListNode::new(add_result);
        // 头插法
        new_node.clone().unwrap().borrow_mut().next = dummy_head.clone().unwrap().borrow().next.clone();
        dummy_head.clone().unwrap().borrow_mut().next = new_node;
    
        // 向下移动两个指针
        a = if a.is_some() {a.unwrap().borrow().next.clone()} else {None};
        b = if b.is_some() {b.unwrap().borrow().next.clone()} else {None};
    }

    if carry == 1 {
        let new_node = ListNode::new(1);
        // 头插法
        new_node.clone().unwrap().borrow_mut().next = dummy_head.clone().unwrap().borrow().next.clone();
        dummy_head.clone().unwrap().borrow_mut().next = new_node;
    }

    // 返回真正的头节点
    let h = dummy_head.clone().unwrap().borrow().next.clone();
    // 释放节点
    dummy_head.unwrap().borrow_mut().next = None;    
    h
}



/*test code */
fn main() {
/*
  链表a: 表示整数123
  1 --> 2 --> 3
  链表b: 表示整数531
  5 --> 3 --> 1

  两个链表相加：得到新的链表：
  6 --> 5 --> 4
*/

    let n11 = ListNode::new(1);
    let n12 = ListNode::new(2);
    n11.clone().unwrap().borrow_mut().next = n12.clone();
    let n13 = ListNode::new(3);
    n12.clone().unwrap().borrow_mut().next = n13.clone();

    println!("=======first linked list=======");
    print_list(n11.clone());
    println!("=======first linked list=======");

    let n21 = ListNode::new(5);
    let n22 = ListNode::new(3);
    n21.clone().unwrap().borrow_mut().next = n22.clone();
    let n23 = ListNode::new(1);
    n22.clone().unwrap().borrow_mut().next = n23.clone();

    println!("=======second linked list=======");
    print_list(n21.clone());
    println!("=======second linked list=======");

    let new_liked_list = add_two_numbers(n11.clone(), n21.clone());
    println!("====两链表相加====");
    print_list(new_liked_list);
    println!("====两链表相加====");
}