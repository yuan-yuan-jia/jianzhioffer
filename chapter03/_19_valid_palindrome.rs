/**
 * 最多删除一个字符得到回文
 * 给定一个字符串，请判断如果最多从字符串中删除一个字符能不能得到一个回文字符串。
 * 例如，如果输入字符串"abca"，由于删除字符'b'或'c'就能得到一个回文字符串，
 * 因此输出为true。
 */


 fn is_palindrome(s: &String, mut i: i32, mut e: i32) -> bool{
    while i <= e {
        let i_char = s.get((i as usize)..(i + 1) as usize).unwrap();
        let e_char = s.get((e as usize)..(e + 1) as usize).unwrap();
        if i_char != e_char {
            break;
        } 
        i+=1;
        e-=1;
    }
    return i > e;
 }

fn valid_palindrome(s: String) -> bool {
    let mut i: i32 = 0;
    let mut end: i32 = (s.len() - 1) as i32;
    
    while i <= end {
        let i_char = s.get((i as usize)..(i + 1) as usize).unwrap();
        let e_char = s.get((end as usize)..(end + 1) as usize).unwrap();
        if  i_char != e_char  {
            break;
        }
        i += 1;
        end -=1;
    }

    if i > end {
        return true;
    }

    return is_palindrome(&s, i+1, end) || is_palindrome(&s, i, end - 1);
}


fn main() {
    let s = String::from("abca");
    let result = valid_palindrome(s);
    if result {
        println!("是回文");
    }else {
        println!("不是回文");
    }
}