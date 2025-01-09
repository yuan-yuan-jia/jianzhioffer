/**
 * 
 * 单词长度的最大乘积
 * 输入一个字符串数组words，
 * 请计算不包含相同字符的两个字符串words[i]和words[j]的长度乘积的最大值。
 * 如果所有字符串都包含至少一个相同字符，那么返回0
 * 。假设字符串中只包含英文小写字母。
 * 例如，输入的字符串数组words为["abcw"，"foo"，"bar"，"fxyz"，"abcdef"]，
 * 数组中的字符串"bar"与"foo"没有相同的字符，
 * 它们长度的乘积为9。"abcw"与"fxyz"也没有相同的字符，
 * 它们长度的乘积为16，这是该数组不包含相同字符的一对字符串的长度乘积的最大值
 * 
 */

fn max_product(words: Vec<String>) -> i32 {
    let mut max_product = i32::MIN;
    let mut letter_flag: Vec<Vec<bool>> = Vec::with_capacity(words.len());
    // 初始化item of letter flag
    for i in &words {
        let mut v = vec![false;26];
        for c in i.chars() {
            v[c as usize - 'a' as usize] = true;    
        }
        letter_flag.push(v);
    }

    let len = words.len();
    let mut i = 0;
    while i < len - 1 {
        let mut j = i + 1;
        while j < len {
            let mut break_flag = 0;
            for k in 0..26 {
                if letter_flag[i][k] && letter_flag[j][k] {
                    break_flag = 1;
                    break;
                } 
            }
            if break_flag == 0 {
                let product = words[i].len() as i32 * words[j]. len() as i32;
                if max_product < product {
                    max_product = product;
                }
            }

            j = j + 1;

        }
        i = i + 1;
    }

    max_product
}

fn main() {
    let mut v = Vec::new();
    //["abcw"，"foo"，"bar"，"fxyz"，"abcdef"]
    v.push(String::from("abcw"));
    v.push(String::from("foo"));
    v.push(String::from("bar"));
    v.push(String::from("fxyz"));
    v.push(String::from("abcdef"));

    let result = max_product(v);
    println!("{}", result);
}