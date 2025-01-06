use std::cell::RefCell;

/**
 * issue three: 前n个数字二进制形式中1的个数
 * 输入一个非负数n，请计算0到n之间每个数字的二进制形式中1的个数，
 * 并输出一个数组。例如，输入的n为4，
 * 由于0、1、2、3、4的二进制形式中1的个数分别为0、1、1、2、1，
 * 因此输出数组[0，1，1，2，1]
 */

fn count_bits_first(n: i32) -> Vec<i32> {
    let mut v = Vec::with_capacity((n + 1) as usize);
    for i in 0..=n {
        let mut j = i;
        let mut one_cnt = 0;
        while j != 0 {
            j = j & (j - 1);
            one_cnt += 1;
        }
        v.push(one_cnt);

    }
    v
}

fn count_bits_second(n: i32) -> Vec<i32> {
    let mut v = Vec::with_capacity((n + 1) as usize);
    v.push(0);
    for i in 1..=n {
        let  j = i;
        //let mut one_cnt = 0;
        // j 比 (j & (j - 1)) 多一个1
        let value = v[(j & (j - 1)) as usize] + 1;
        v.push(value);

    }
    v
}


fn count_bits_third(n: i32) -> Vec<i32> {
    let mut v = Vec::with_capacity((n + 1) as usize);
    v.push(0);
    for i in 1..=n {
        let  j = i;
        // 偶数和偶数/2的1的个数是一样的。
        // 偶数相当于偶数/2左移一位
        // 偶数右移一位，就可拿到结果
        // 奇数是奇数/2 + 1
        let mut value= 0;
        // if i % 2 == 0 {
        //     value = v[(j >> 1) as usize];
        // }else {
        //     value = v[(j >> 1) as usize] + 1;
        // }
        value = v[(j >> 1) as usize] + (i & 1);
        
        v.push(value);

    }
    v
}

fn main() {
    println!("{:?}", count_bits_first(4));
    println!("{:?}", count_bits_second(4));
    println!("{:?}", count_bits_third(4));
}

