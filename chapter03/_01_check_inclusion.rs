/**
 *面试题14: 字符串中的变位词
 *
 * 输入字符串s1和s2，如何判断字符串s2中是否包含字符串s1的某个变位词？
 * 如果字符串s2中包含字符串s1的某个变位词，则字符串s1至少有一个变位词是字符串s2的子字符串。
 * 假设两个字符串中只包含英文小写字母。例如，字符串s1为"ac"，字符串s2为"dgcaf"，
 * 由于字符串s2中包含字符串s1的变位词"ca"，因此输出为true。如果字符串s1为"ab"，字符串s2为"dgcaf"，
 * 则输出为false。 
 * 
 */


 fn all_zero(a: &[i32;26]) -> bool {
        for i in a.iter() {
            if *i != 0 {
                return false;
            }
        }
        true
 }

 fn check_inclusion(str1 :&str, str2 :&str) -> bool {
     
     if str1.len() > str2.len() {
         return false;
     } 
    let mut str_count = [0;26];
    {
        let mut index = 0;
        let len = str1.len();
        while index < len {
            str1.chars().nth(index).map(|c| {
                str_count[c as usize - 'a' as usize] += 1;
            });
            str2.chars().nth(index).map(|c| {
                str_count[c as usize - 'a' as usize] -= 1;
            });
            index += 1;
            
        }
    }

    if all_zero(&str_count) {
        return true;
    }

    let mut right = str1.len();
    let len = str1.len();

    while right < str2.len() {
        str2.chars().nth(right).map(|c| {
            str_count[c as usize - 'a' as usize] -= 1;
        });
        str2.chars().nth(right - len).map(|c| {
            str_count[c as usize - 'a' as usize] += 1;
        });
        if all_zero(&str_count) {
            return true;
        } 
        right += 1;  
    }


     false
 }

fn main() {
    let s1 = "ac";
    let s2 = "dgcaf";
    let result = check_inclusion(s1, s2);
    println!("{}", result);
}