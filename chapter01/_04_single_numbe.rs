/**
 * 只出现一次的数字
 * 输入一个整数数组，数组中只有一个数字出现了一次，
 * 而其他数字都出现了3次。请找出那个只出现一次的数字。
 * 例如，如果输入的数组为[0，1，0，1，0，1，100]，
 * 则只出现一次的数字是100。
 * 
 */
 fn single_number(nums: Vec<i32>) -> i32 {
    let mut temp_result = vec![0;32];
    let mut result = 0;
    for n in &nums {
        for i in 0..32 {
            temp_result[i] = temp_result[i]  + (((1 << i) & *n) >> i); 
        }
    }
    for i in 0..32 {
        result = result | (temp_result[i] % 3)  <<i;
    }
    result

 }

fn main() {
    let v = vec![0,1,0,1,0,1,100];
    let result = single_number(v);
    println!("{}", result);
}