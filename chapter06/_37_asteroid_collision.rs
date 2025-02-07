fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut v = vec![];
    let mut stack = std::collections::VecDeque::new();
    
    for i in asteroids.iter() {
        if stack.is_empty() {
            stack.push_back(*i);
        } else {
            if  *i < 0 && !stack.is_empty() && *stack.back().unwrap() > 0 {
                let mut should_put_i_to_statck = true;
                while *i < 0 && !stack.is_empty() && *stack.back().unwrap() > 0 {
                    if i.abs() > stack.back().unwrap().abs() {
                        stack.pop_back();
                    }else if i.abs() == stack.back().unwrap().abs() {
                        stack.pop_back();
                        should_put_i_to_statck = false;
                        break;
                    }else {
                        should_put_i_to_statck = false;
                        break;
                    }
                }
                if should_put_i_to_statck {
                    stack.push_back(*i);
                }
            }else {
                stack.push_back(*i);
            }
        }
    }
    
    for i in stack.iter() {
        v.push(*i);
        
    }
    v
}

/*Driver Code */
fn main() {
    let v = [4, 5, -6, 4, 8, -5];
    let v = Vec::from(v);
    let r = asteroid_collision(v);
    println!("r:{:?}", r);
    assert_eq!(r, vec![-6, 4, 8]);
}