use std::borrow::BorrowMut;

/**
 * 
 * 输入一个字符串，求该字符串中不含重复字符的最长子字符串的长度。
 * 例如，输入字符串"babcca"，其最长的不含重复字符的子字符串是"abc"，长度为3
 */


fn length_of_longest_substring(s: String) -> i32 {
    let mut res = 0;
    let mut arr = [0;256];
    let len = s.len() as i32;
    let mut p = 0;
    let mut q = 0;
    let mut dup_cnt = 0;
    while q  < len {
        let ss =s.chars().nth(q as usize).unwrap();
        arr[ss as usize] += 1;
        if arr[ss as usize] == 2 {
            dup_cnt+=1
        } 

        while dup_cnt > 0 {
            let p_ss = s.chars().nth(p as usize).unwrap();
            arr[p_ss as usize] -= 1;
            if arr[p_ss as usize] == 1 {
                dup_cnt-=1;
            }
            p+=1;
        }
        if res < (q - p + 1) {
            res = q - p + 1;
        }
        q+=1;

    }
    
    res
}

fn main() {
    let s = String::from("babcca");
    let res = length_of_longest_substring(s);
    println!("len={}", res);
}