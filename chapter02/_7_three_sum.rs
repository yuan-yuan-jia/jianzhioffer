/*
  数组中和为0的3个数字
  输入一个数组，如何找出数组中所有和为0的3个数字的三元组？
  需要注意的是，返回值中不得包含重复的三元组。
  例如，在数组[-1，0，1，2，-1，-4]中
  有两个三元组的和为0，
  它们分别是[-1，0，1]和[-1，-1，2]。
*/


fn is_exists(num: &Vec<i32>, start: usize, target: i32) -> bool { 
    // 可以修改为二分查找
    let mut start = start;
    while start < num.len() {
        if *num.get(start).unwrap() == target {
            return true;
        }
        start+=1;
    }
    false
}

fn tow_sum(num: &Vec<i32>, start: usize, target: i32,first_ele: i32) -> Vec<Vec<i32>>   {
    let mut result = vec![];
    let mut start = start;
    while start < num.len() {
        let mut v = vec![];
        let second_num = *num.get(start).unwrap();
        let thired_num_should_be = target - second_num;
        if is_exists(num, start + 1, thired_num_should_be) {
            v.push(first_ele);
            v.push(second_num);
            v.push(thired_num_should_be);
            result.push(v);
        }

        while start < num.len() && *num.get(start).unwrap() == second_num {
            start+=1;
        }

    }
    result
}

fn three_sum(num: Vec<i32>) -> Vec<Vec<i32>> {
    let mut num = num;
    num.sort();
    let mut result:Vec<Vec<i32>> = Vec::new();
    let len = num.len();
    let mut i: usize = 0;
    while i < len {
        let first_num = num.get(i).unwrap();
        let r = tow_sum(&num, i + 1, 0 - (*first_num),*first_num);
        result.extend(r);

        while i < len && *num.get(i).unwrap() == *first_num {
            i+=1;
        }
    }
    result
}


fn main() {
    let v = [-1,0,1,2,-1,-4];
    let mut v1 = Vec::new();
    v1.extend_from_slice(&v);

    println!("{:?}", v1);
    let result = three_sum(v1.clone());
    println!("{:?}", result);
    // [-1，0，1]和[-1，-1，2]
}