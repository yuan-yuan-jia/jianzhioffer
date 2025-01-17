use std::{cell::RefCell, collections::BinaryHeap, sync::Arc};



struct ListNode {
    key: i32,
    value: i32,
    next: Option<Arc<RefCell<ListNode>>>,
    prev: Option<Arc<RefCell<ListNode>>>,
}

struct LRUCache {
    head: Option<Arc<RefCell<ListNode>>>,
    tail: Option<Arc<RefCell<ListNode>>>,
    map: std::collections::HashMap<i32, Option<Arc<RefCell<ListNode>>>>,
    capacity: i32
}

impl LRUCache {
    fn new(cpa: i32) -> Self {
        let head = ListNode::new(-1, -1);
        let tail = ListNode::new(-1, -1); 
        
        head.clone().unwrap().borrow_mut().next = tail.clone();
        tail.clone().unwrap().borrow_mut().prev = head.clone();
        Self {
            map: std::collections::HashMap::new(),
            head: head,
            tail: tail,
            capacity: cpa
        }
    }

    fn get(&self, key: i32) -> i32 {
        let a = self.map.get(&key);
        if a.is_none() {
            return -1;
        }

        let a = a.unwrap().as_ref().unwrap();
        let c = a.clone();
        self.move_to_tail(a.clone(), a.clone().borrow().value);

        return a.borrow().value;
    }

    fn move_to_tail(&self, node: Arc<RefCell<ListNode>>, new_value: i32) {
        // 先删除
        self.delete_node(node.clone());
        node.borrow_mut().value = new_value;
        // 插入到尾部
        self.insert_to_tail(node);
    }

    fn delete_node(&self,node: Arc<RefCell<ListNode>>) {
        node.borrow_mut().prev.clone().unwrap().borrow_mut().next = node.borrow().next.clone();
        node.borrow_mut().next.clone().unwrap().borrow_mut().prev = node.borrow().prev.clone();
        node.borrow_mut().next = None;
        node.borrow_mut().prev = None;
    }

    fn insert_to_tail(&self,node: Arc<RefCell<ListNode>>) {
        self.tail.clone().unwrap().borrow_mut().prev.clone().unwrap().borrow_mut().next = Some(node.clone());
        node.borrow_mut().prev = self.tail.clone().unwrap().borrow().prev.clone();
        node.borrow_mut().next = self.tail.clone();
        self.tail.clone().unwrap().borrow_mut().prev = Some(node.clone());
    }

    fn put(&mut self,key: i32, value: i32) {
        if self.map.contains_key(&key) {
            let node = self.map.get(&key).unwrap().as_ref().unwrap().clone();
            self.move_to_tail(node, value);
        }else {
            if self.map.len() == self.capacity as usize {
                // 删除头节点
                let to_be_deleted = self.head.clone().unwrap().borrow().next.clone();
                self.delete_node(to_be_deleted.clone().unwrap());
                self.map.remove(&to_be_deleted.unwrap().borrow().key);
            }

            // 插入新节点
            let new_node = ListNode::new(key, value);
            self.insert_to_tail(new_node.clone().unwrap());
            self.map.insert(key, new_node);
        }
    }

}


impl ListNode {
    fn new(
        k: i32,
        v: i32
    )  -> Option<Arc<RefCell<ListNode>>> {
        Some(
            Arc::new(
                RefCell::new(
                    Self {
                        key: k,
                        value: v,
                        next: None,
                        prev: None,
                    }
                )
            )
        )
    }
}





fn main() {
 let mut lru = LRUCache::new(4);
 lru.put(1, 1);
 lru.put(2, 2);
 lru.put(3, 3);
 lru.put(4, 4);
 
}