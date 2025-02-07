
fn eval_prn(expression : &str) -> i32 {    
    let mut statck = std::collections::VecDeque::new();
    for c in expression.chars() {
        if c >= '0' && c <= '9' {
            let r = c as i32 - '0' as i32;
            statck.push_back(r);
        }else {
            // 操作符
            match c {
                '+' => {
                    let r2 = statck.pop_back().unwrap();
                    let r1 = statck.pop_back().unwrap();
                    statck.push_back(r1 + r2);
                },
                '-' => {
                    let r2 = statck.pop_back().unwrap();
                    let r1 = statck.pop_back().unwrap();
                    statck.push_back(r1 - r2);
                },
                '/' => {
                    let r2 = statck.pop_back().unwrap();
                    let r1 = statck.pop_back().unwrap();
                    statck.push_back(r1 / r2);
                },
                '*' => {
                    let r2 = statck.pop_back().unwrap();
                    let r1 = statck.pop_back().unwrap();
                    statck.push_back(r1 * r2);
                },
                _ => {
                    panic!("unkown operator");
                }
            }
        }
    }
    statck.pop_back().unwrap()
}


/*Driver Code */
fn main() {
    let ex = String::from("213*+");
    let r = eval_prn(ex.as_str());
    println!("{}", r);
}