


#[derive(Eq,Debug)]
struct KeyVal(i32, i32);


impl KeyVal {
    fn sum(&self) -> i32 {
        self.0 + self.1
    }
}

impl core::cmp::PartialEq for KeyVal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl core::cmp::PartialOrd for KeyVal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.1.partial_cmp(&other.1)
    }
}



impl core::cmp::Ord for KeyVal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        
        if self.sum() < other.sum() {
            return std::cmp::Ordering::Less;
        }else if self.sum() == other.sum() {
            return std::cmp::Ordering::Greater;
        }else {
            return std::cmp::Ordering::Equal;
        }        
    
    }
}


fn k_smallest_pair(nums1: &[i32], nums2: &[i32], k: usize) -> Vec<KeyVal> {
    let mut v = Vec::new();
    let mut heap = std::collections::BinaryHeap::new();
    for i in nums1 {
        for j in nums2 {
            let kv = KeyVal(*i,*j);
            if heap.len() < k {
                heap.push(kv);
            }else {
                let peek = heap.peek().unwrap();
                if kv.cmp(peek) == std::cmp::Ordering::Less {
                    heap.pop();
                    heap.push(kv);
                }
            }
        }
    }
    while !heap.is_empty() {
        v.push(heap.pop().unwrap());
    }
    v
}










/*Driver Code */
fn main() {
    let arr = [1,5,13,21];
    let arr1 = [2,4,9,15];

    let v = k_smallest_pair(&arr, &arr1, 3);
    println!("{:?}", v);
}