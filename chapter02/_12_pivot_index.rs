
fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut res = -1;
    let mut dp: Vec<i32> = vec![0;nums.len()];
    let mut sum = 0;
    
    for i in 0..nums.len() {
        sum += nums.get(i).unwrap();
        let c= dp.get_mut(i).unwrap();
        *c = sum;
    }

    for i in 0..nums.len() {
        let item = *nums.get(i).unwrap();
        let item_sum = *dp.get(i).unwrap();
        if item_sum - item == sum - item_sum {
            res = i as i32;
            break;
        } 
    }
    
    res
}



fn main() {

    let v = vec![1,7,3,6,2,9];
    let r = pivot_index(v);
    println!("r: {}",r);
    assert_eq!(r, 3);
}