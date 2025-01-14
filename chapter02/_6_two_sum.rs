/**
 * 排序数组中的两个数字之和
 * 输入一个递增排序的数组和一个值k，请问如何在数组中找出两个和为k的数字并返回它们的下标？
 * 假设数组中存在且只存在一对符合条件的数字，同时一个数字不能使用两次。
 * 例如，输入数组[1，2，4，6，10]，k的值为8，数组中的数字2与6的和为8，它们的下标分别为1与3。
 * 
 */
//时间复杂O(n)，空间O(n)
fn two_sum_first_soluton(numbers: Vec<i32>, target: i32) 
-> Box<[i32;2]> 
{
    use std::collections::HashMap;
    let mut result_arr = [-1;2];
    let mut hash_map = HashMap::new();
    for (index, item) in numbers.iter().enumerate() {
        hash_map.insert(*item, index);
    }

    for elem in &numbers {
        let other_elem = target - elem;
        if hash_map.contains_key(&other_elem) {
            result_arr[0] = hash_map.get(&elem).unwrap().clone() as i32;
            result_arr[1] = hash_map.get(&other_elem).unwrap().clone() as i32;
            return Box::new(result_arr);
        }
    }

    Box::new(result_arr)
}

//时间O(n)，空间O(1)
fn two_sum_second_soluton(numbers: Vec<i32>, target: i32) 
-> Box<[i32;2]> 
{
    let mut end = numbers.len() - 1;
    let mut start = 0 as usize;
    let mut arr = [-1;2];
    while start <= end {
        let first = numbers.get(start).unwrap();
        let second = numbers.get(end).unwrap();
        if first + second == target {
            arr[0] = start as i32;
            arr[1] = end as i32;
            return Box::new(arr);
        } else if  first + second < target { 
            start = start + 1;
        }else {
            end = end - 1;
        }

    }

    Box::new(arr)
}


fn main() {
    let arr = [1,2,4,6,10];
    let v = Vec::from(arr);
    let r = two_sum_first_soluton(v.clone(), 8);
    println!("{:?}", r);
    let r = two_sum_second_soluton(v, 8);
    println!("{:?}", r);
}