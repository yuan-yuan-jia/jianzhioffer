/*
  和大于或等于k的最短子数组。
  输入一个正整数组成的数组和一个正整数k，
  请问数组中和大于或等于k的连续子数组的最短长度是多少？
  如果不存在所有数字之和大于或等于k的子数组，则返回0。
  例如，输入数组[5，1，4，3]，k的值为7，
  和大于或等于7的最短连续子数组是[4，3]，
  因此输出它的长度2。
*/

fn min_subarray_len(nums: Vec<i32>, k: i32) -> i32 {    
    let mut min_len = i32::MAX;

    let mut end = 0 as usize;
    let mut start = 0 as usize;
    let len = nums.len();
    let mut sum = 0;
    while end < len {
        
        sum += nums.get(end).unwrap();
        
        // 寻找最小的满足条件的长度
        // 不满足跳出循环，寻找新的子数组
        while start <= end && sum >= k {
            
            min_len = std::cmp::min(min_len, (end - start + 1) as i32 );
            
            sum -= nums.get(start).unwrap();
            
            start+=1;    
        }

        // 不满足条件，直接向右移动右边界
        end +=1;
    }

    if min_len == i32::MAX {0} else {min_len}
}


/*Test Code */
fn main() {

    let v = vec![5,1,4,3];

    let r = min_subarray_len(v.clone(), 7);
    println!("r:{}",r);
    assert_eq!(r, 2);
}