/*
 乘积小于k的子数组
 输入一个由正整数组成的数组和一个正整数k，
 请问数组中有多少个数字乘积小于k的连续子数组？
 例如，输入数组[10，5，2，6]，
 k的值为100，有8个子数组的所有数字的乘积小于100，
 它们分别是[10]、[5]、[2]、[6]、[10，5]、[5，2]、[2，6]和[5，2，6]。
*/
fn num_subarray_product_less_thank(
    nums: Vec<i32>,
    k: i32
) -> i32 {
    let mut result = 0;
    let mut start = 0 as usize;
    let mut end = 0 as usize;
    let mut product = 1;
    while end < nums.len() {
        product = product * nums.get(end).unwrap();

        while start <= end && product >= k {
            product = product / nums.get(start).unwrap();
            start+=1;
        }
        if start <= end {
            result+= (end - start + 1) as i32;
        }
        end+=1;
    }
    
    result
}

fn main() {
    let v = vec![10,5,2,6];
    let r = num_subarray_product_less_thank(
        v.clone(),
        100
    );
    println!("r: {}", r);
    assert_eq!(r, 8);
}