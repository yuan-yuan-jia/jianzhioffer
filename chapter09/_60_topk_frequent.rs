


#[derive(Eq)]
struct KeyVal(i32, i32);


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
        if self.1 < other.1 {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Equal;
    }
}

fn topk_frequent(nums: &[i32], k: i32) -> Vec<i32> {
    let mut hash = std::collections::HashMap::new();
    let mut v = vec![];
    let mut heap = std::collections::BinaryHeap::new();
    for i in nums {
        match hash.get_mut(i) {
            Some(v1) => {
                *v1 = *v1 + 1
            },
            None => {hash.insert(*i, 1);} 
        }
    }

    for i in &hash {
        let kv = KeyVal(*i.0,*i.1);
        heap.push(kv);
    }
    
    while heap.len() > k as usize {
        heap.pop();
    }

    while !heap.is_empty() {
        let kv = heap.pop().unwrap();
        v.push(kv.0);
    }
    v
}






/*Driver Code */
fn main() {
    let arr = [1,2,2,1,3,1];
    let v = topk_frequent(&arr, 2);
    println!("{:?}", v);
}