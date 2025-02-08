fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut v = vec![0;temperatures.len()];
    let mut stack = std::collections::VecDeque::new();
    for i in temperatures.iter().enumerate() {
        // if stack.is_empty() {
        //     stack.push_back(i.0);
        // }else {
        //     if *temperatures.get(*stack.back().unwrap()).unwrap() < *i.1 {
        //         while !stack.is_empty() && *temperatures.get(*stack.back().unwrap()).unwrap() < *i.1 {
        //             *v.get_mut(*stack.back().unwrap()).unwrap() = i.0 as i32 - (*stack.back().unwrap()) as i32 ;
        //             stack.pop_back();
        //         }
        //         stack.push_back(i.0);
        //     }else {
        //         stack.push_back(i.0);
        //     }
        // }

        if !stack.is_empty() {
            while !stack.is_empty() && *temperatures.get(*stack.back().unwrap()).unwrap() < *i.1 {
                *v.get_mut(*stack.back().unwrap()).unwrap() = i.0 as i32 - (*stack.back().unwrap()) as i32 ;
                stack.pop_back();
            }
           
        }
        stack.push_back(i.0);
        

    }
    v
}



/*Driver Code */
fn main() {
    let v = vec![35,31,33,36,34];
    let r = daily_temperatures(v.clone());
    println!("{:?}", r);
    assert_eq!(r, vec![3,1,1,0,0]);
}