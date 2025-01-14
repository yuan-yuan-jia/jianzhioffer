#[allow(unused_doc_comments)]
/**
 * 两个链表的第1个重合节点
 * 
 * 输入两个单向链表，请问如何找出它们的第1个重合节点。
 * 下面两个链表的第1个重合节点的值是4。
 * 1 --> 2 --> 3 
 *              \
 *               \
 *                - > 4 --> 5 --> 6
 *               /
 *       7 --> 8 
 * 
 */

include!("../src/include/list.rs");

fn count_node(h: Option<Arc<RefCell<ListNode<i32>>>>) -> i32 {
    let mut cnt = 0;
    let mut h_temp = h.clone();
    
    while h_temp.is_some() {
        h_temp = h_temp.unwrap().borrow().next.clone();
        cnt += 1;
    }
    cnt
}

fn get_insersection_node(
    a: Option<Arc<RefCell<ListNode<i32>>>>, 
    b: Option<Arc<RefCell<ListNode<i32>>>>) -> Option<Arc<RefCell<ListNode<i32>>>>  {
    if a.is_none() || b.is_none() {
        return None;
    }

    let cnt_a = count_node(a.clone());
    let cnt_b = count_node(b.clone());

    let mut a_temp = a.clone();
    let mut b_temp = b.clone();
    
    // 长的节点，先走 abs(cnt_a - cnt_b)
    if cnt_a > cnt_b {
        let mut cnt = 0;
        let diff = cnt_a - cnt_b;
        while a_temp.is_some() && cnt < diff  {
            a_temp = a_temp.unwrap().borrow().next.clone();
            cnt +=1;
        }    
    }else if  cnt_a < cnt_b {
        let mut cnt = 0;
        let diff = cnt_b - cnt_a;
        while b_temp.is_some() && cnt < diff  {
            b_temp = b_temp.unwrap().borrow().next.clone();
            cnt +=1;
        }
    }

    // 两者同时走
    if a_temp.is_none() || b_temp.is_none() {
        return None;
    }

    while a_temp.is_some() && b_temp.is_some() {
        let aa = a_temp.clone().unwrap();
        let bb = b_temp.clone().unwrap();

        if aa.as_ptr() == bb.as_ptr() {
            return Some(aa);
        }
        a_temp = a_temp.unwrap().borrow().next.clone();
        b_temp = b_temp.unwrap().borrow().next.clone();
    }

    None

}

fn main() {

    /**
     
    1 --> 2 --> 3 
               \
                \
                 - > 4 --> 5 --> 6
                /
        7 --> 8 
     */
    
    // h1===========
    let mut h1 = ListNode::new(1);
    let mut h12 = ListNode::new(2);
    h1.clone().unwrap().borrow_mut().next = h12.clone();
    let mut h13 = ListNode::new(3);
    h12.clone().unwrap().borrow_mut().next = h13.clone();
    
    // h1===========
    // common ========
    let mut h_common1 = ListNode::new(4);
    h13.clone().unwrap().borrow_mut().next = h_common1.clone();
    let mut h_common2 = ListNode::new(5);
    h_common1.clone().unwrap().borrow_mut().next = h_common2.clone();
    let mut h_common3 = ListNode::new(6);
    h_common2.clone().unwrap().borrow_mut().next = h_common3.clone();
    // common ========

    // h2=========
    let mut h2 = ListNode::new(7);
    let mut h21 = ListNode::new(8);
    h2.clone().unwrap().borrow_mut().next = h21.clone();
    h21.clone().unwrap().borrow_mut().next = h_common1.clone();
    // h2=========

    // print h1
    print_list(h1.clone());
    // print h2
    print_list(h2.clone());

    // find the common linked list head
    let r =  get_insersection_node(
        h1.clone(),
        h2.clone()
    );

    assert!(r.is_some() && r.unwrap().borrow().val == 4)

}