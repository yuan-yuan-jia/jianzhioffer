


struct KthLargets {
    size: u32,
    heap: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
}

impl KthLargets {
    fn new(k: u32, nums: &[i32]) -> Self {
        
        let heap  = std::collections::BinaryHeap::with_capacity(nums.len());
        let mut kth = Self { size: k, heap };
        
        for i in nums {
            kth.add(*i);
        }

        kth
    }

    fn add(&mut self, val: i32) -> Option<i32> {
        
        if self.heap.len() < self.size as usize {
            self.heap.push(std::cmp::Reverse(val));
        } else {
            let peek_val = self.heap.peek().unwrap();
            if val > peek_val.0 {
                self.heap.pop();
                self.heap.push(std::cmp::Reverse(val));
            }
        }
        
        
        match self.heap.peek() {
            Some(e) => { Some(e.0)},
            None => None,
        }
    }
}


/*Driver Code */
fn main() {
    let mut kth = KthLargets::new(3, &[4,5,8,2]);
    let r = kth.add(3);
    assert_eq!(r, Some(4));

    let r = kth.add(5);
    assert_eq!(r, Some(5));

}