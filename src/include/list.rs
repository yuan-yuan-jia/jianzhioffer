use std::{cell::RefCell, fmt::{Debug, Display}, sync::Arc};

pub struct ListNode<T>  where T: Clone + Display + Debug + Default {
    val: T,
    next: Option<Arc<RefCell<ListNode<T>>>>
}

impl <T> ListNode<T> where T: Clone + Display + Debug + Default {
    fn new(val: T) -> Option<Arc<RefCell<ListNode<T>>>>{
        Some(
            Arc::new(
                RefCell::new(Self { val, next: None })
            )
        )
    }
}


fn print_list_recursively<T>(head: Option<Arc<RefCell<ListNode<T>>>>) where T: Clone + Display + Debug + Default  {
    if head.is_none() {
        println!("None");
        return;
    }
    let head = head.unwrap();
    let next = head.as_ref().borrow().next.clone();
    print!("{:?}", head.borrow().val);
    print!("---->");
    print_list_recursively(next);
}

pub fn print_list<T>(head: Option<Arc<RefCell<ListNode<T>>>>) where T: Clone + Display + Debug + Default {
    if head.is_none() {
        return;
    }
    print_list_recursively(head);
}